use res::{app::DebuggerApp, system::System};

fn main() -> eframe::Result<()> {
    let system = System::default();
    let rom_label = "built-in demo program".to_string();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "olc6502 Demonstration",
        options,
        Box::new(move |_cc| Box::new(DebuggerApp::new(system, rom_label))),
    )
}
