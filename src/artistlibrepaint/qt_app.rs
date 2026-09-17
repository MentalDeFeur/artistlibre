use qt_widgets::{qt_core::QString, QApplication, QLabel, QVBoxLayout, QWidget};

pub fn run() -> ! {
    QApplication::init(|_| unsafe {
        let window = QWidget::new_0a();
        window.set_window_title(&QString::from_std_str("ArtistLibrePaint"));
        window.resize_2a(1200, 800);

        let layout = QVBoxLayout::new_0a();
        layout.add_widget(QLabel::from_q_string(&QString::from_std_str(
            "ArtistLibrePaint - backend Qt KDE",
        )));
        window.set_layout(layout.into_ptr());
        window.show();

        QApplication::exec()
    })
}