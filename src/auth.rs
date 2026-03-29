use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::ProxyError;
use crate::state::AppState;
use crate::telemetry::metrics::RATE_LIMITED_TOTAL;
use crate::types::RequestContext;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ProxyError> {
    let token = extract_bearer_token(&req)?;

    let key_config = state
        .config
        .virtual_keys
        .iter()
        .find(|k| k.key == token)
        .ok_or_else(|| ProxyError::Unauthorized("Invalid API key".to_string()))?;

    // Per-key rate limiting (token bucket, per-pod)
    if let Some(bucket) = state.rate_limiter.get(&key_config.name) {
        if !bucket.try_consume() {
            RATE_LIMITED_TOTAL
                .with_label_values(&[&key_config.name])
                .inc();
            tracing::warn!(key_name = %key_config.name, "Rate limit exceeded");
            return Err(ProxyError::RateLimited(format!(
                "Rate limit exceeded for key '{}'",
                key_config.name
            )));
        }
    }

    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let ctx = RequestContext {
        request_id,
        key_name: key_config.name.clone(),
        allowed_models: key_config.allowed_models.clone(),
    };

    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}

fn extract_bearer_token(req: &Request) -> Result<String, ProxyError> {
    let header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ProxyError::Unauthorized("Missing Authorization header".to_string()))?;

    let value = header.to_str().map_err(|_| {
        ProxyError::Unauthorized("Invalid Authorization header encoding".to_string())
    })?;

    let token = value.strip_prefix("Bearer ").ok_or_else(|| {
        ProxyError::Unauthorized("Authorization header must use Bearer scheme".to_string())
    })?;

    if token.is_empty() {
        return Err(ProxyError::Unauthorized("Empty bearer token".to_string()));
    }

    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request as HttpRequest;

    fn build_request(auth_header: Option<&str>) -> HttpRequest<Body> {
        let mut builder = HttpRequest::builder().uri("/");
        if let Some(value) = auth_header {
            builder = builder.header("Authorization", value);
        }
        builder.body(Body::empty()).unwrap()
    }

    #[test]
    fn missing_auth_header_returns_unauthorized() {
        let req = build_request(None);
        let err = extract_bearer_token(&req).unwrap_err();
        assert!(matches!(err, ProxyError::Unauthorized(_)));
        assert!(err.to_string().contains("Missing Authorization header"));
    }

    #[test]
    fn non_bearer_scheme_returns_unauthorized() {
        let req = build_request(Some("Basic dXNlcjpwYXNz"));
        let err = extract_bearer_token(&req).unwrap_err();
        assert!(matches!(err, ProxyError::Unauthorized(_)));
        assert!(err.to_string().contains("Bearer scheme"));
    }

    #[test]
    fn empty_token_returns_unauthorized() {
        let req = build_request(Some("Bearer "));
        let err = extract_bearer_token(&req).unwrap_err();
        assert!(matches!(err, ProxyError::Unauthorized(_)));
        assert!(err.to_string().contains("Empty bearer token"));
    }

    #[test]
    fn valid_bearer_token_is_extracted() {
        let req = build_request(Some("Bearer sk-my-secret-key"));
        let token = extract_bearer_token(&req).unwrap();
        assert_eq!(token, "sk-my-secret-key");
    }

    #[test]
    fn token_with_spaces_is_preserved() {
        // Everything after "Bearer " is the token, including spaces
        let req = build_request(Some("Bearer tok en with spaces"));
        let token = extract_bearer_token(&req).unwrap();
        assert_eq!(token, "tok en with spaces");
    }
}
