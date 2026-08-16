mod app;
mod document;
mod layer;

use app::ArtistLibreApp;

fn main() -> glib::ExitCode {
    let app = ArtistLibreApp::new();
    app.run()
}
