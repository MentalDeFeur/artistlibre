mod app;
mod document;
mod layer;

#[cfg(feature = "qt")]
mod qt_app;

use app::ArtistLibreApp;

fn main() {
    if is_kde_session() {
        #[cfg(feature = "qt")]
        {
            qt_app::run();
            return;
        }

        eprintln!(
            "Session KDE/Plasma détectée, mais le backend Qt n'est pas compilé; utilisation de GTK."
        );
    }

    let app = ArtistLibreApp::new();
    app.run();
}

fn is_kde_session() -> bool {
    ["XDG_CURRENT_DESKTOP", "XDG_SESSION_DESKTOP", "DESKTOP_SESSION"]
        .into_iter()
        .filter_map(|name| std::env::var(name).ok())
        .map(|value| value.to_ascii_uppercase())
        .any(|value| value.contains("KDE") || value.contains("PLASMA"))
}
