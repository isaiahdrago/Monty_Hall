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
