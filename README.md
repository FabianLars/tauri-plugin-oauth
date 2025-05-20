# Tauri Plugin OAuth

A minimalistic Rust library and Tauri plugin for handling browser-based OAuth flows in desktop
applications. This plugin spawns a temporary localhost server to capture OAuth redirects, solving
the challenge of using OAuth with desktop apps.

## Why This Plugin?

Many OAuth providers (like Google and GitHub) don't allow custom URI schemes ("deep links") as
redirect URLs. This plugin provides a solution by:

1. Spawning a temporary local server
2. Capturing the OAuth redirect
3. Passing the authorization data back to your app

> **Note**: For an alternative approach using deep linking,
> see [tauri-plugin-deep-link](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/deep-link). The deep-link
> plugin can automatically start your app if there's no open instance.

## Installation

```toml
# Cargo.toml
[dependencies]
tauri-plugin-oauth = "2"
```

For Tauri projects using npm or yarn:

```bash
npm install @fabianlars/tauri-plugin-oauth@2
# or
yarn add @fabianlars/tauri-plugin-oauth@2
```

## Usage

### Rust

```rust
use tauri::{command, Emitter, Window};
use tauri_plugin_oauth::start;

#[command]
async fn start_server(window: Window) -> Result<u16, String> {
    start(move |url| {
        // Because of the unprotected localhost port, you must verify the URL here.
        // Preferebly send back only the token, or nothing at all if you can handle everything else in Rust.
        let _ = window.emit("redirect_uri", url);
    })
        .map_err(|err| err.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()

        .plugin(tauri_plugin_oauth::init())
        .invoke_handler(tauri::generate_handler![start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

```

### TypeScript

```typescript
import {
  start,
  cancel,
  onUrl,
  onInvalidUrl,
} from "@fabianlars/tauri-plugin-oauth";

async function startOAuthFlow() {
  try {
    const port = await start();
    console.log(`OAuth server started on port ${port}`);

    // Set up listeners for OAuth results
    await onUrl((url) => {
      console.log("Received OAuth URL:", url);
      // Handle the OAuth redirect
    });

    // Initiate your OAuth flow here
    // ...
  } catch (error) {
    console.error("Error starting OAuth server:", error);
  }
}

// Don't forget to stop the server when you're done
async function stopOAuthServer() {
  try {
    await cancel(port);
    console.log("OAuth server stopped");
  } catch (error) {
    console.error("Error stopping OAuth server:", error);
  }
}
```

## Configuration

You can configure the plugin behavior using the `OauthConfig` struct:

If you set the `redirect_uri` field, the plugin will redirect to the provided URL after the OAuth process is complete instead of returning the `response` content.

```rust
use tauri_plugin_oauth::OauthConfig;

let config = OauthConfig {
    ports: Some(vec![8000, 8001, 8002]),
    response: Some("OAuth process completed. You can close this window.".into()),
    redirect_uri: Some("http://tauri.localhost/homepage".into()),
};

start_with_config(config, |url| {
    // Handle OAuth URL
})
.await
.expect("Failed to start OAuth server");
```

## Security Considerations

- Always validate the received OAuth URL on your server-side before considering it authentic.
- Use HTTPS for your OAuth flow to prevent man-in-the-middle attacks.
- Implement proper token storage and refresh mechanisms in your application.

## Contributing

Contributions are always welcome! Please feel free to submit a Pull Request.

## License

This project is dual-licensed under either of the following licenses, at your option:

- [Apache License, Version 2.0](LICENSE_APACHE-2.0)
- [MIT License](LICENSE_MIT)
