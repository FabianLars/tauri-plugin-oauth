# Tauri Plugin oauth

WIP

Minimalistic rust library and Tauri plugin(soon) to spawn a temporary localhost server which you redirect to from browser based oauth flows ("Login with X").
Needed because many sites such as Google and GitHub don't allow custom URI schemes ("deep link") as redirect URLs.

See https://github.com/FabianLars/tauri-plugin-deep-link for an alternative based on deep linking. This one will automatically start your app if there is no open instance.
