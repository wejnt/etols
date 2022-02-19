use etols_frame::app;

fn main() {
    tracing_subscriber::fmt::init();

    let app = app::EtolsApp::new("etols");

    let options = eframe::NativeOptions {
        resizable: true,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}
