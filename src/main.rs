#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Door Puzzle",
        native_options,
        Box::new(|_cc| Ok(Box::new(door_statistics::DoorPuzzleApp::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = door_statistics::start().await {
            web_sys::console::error_1(&error);
        }
    });
}
