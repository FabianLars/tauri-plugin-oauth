const COMMANDS: &[&str] = &["start", "cancel"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        //.android_path("android")
        //.ios_path("ios")
        .build();
}
