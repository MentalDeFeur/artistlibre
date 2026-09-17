use crate::document::{BrushKind, DocumentPage, Tool};
use gdk4::RGBA;
use gtk4::prelude::*;
use gtk4::{
    Adjustment, Application, ApplicationWindow, Box as GtkBox, Button, ColorDialog,
    ColorDialogButton, DropDown, HeaderBar, Label, Notebook, Orientation, SpinButton, Stack,
    StackSwitcher, StringObject,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppState {
    pub doc_count: usize,
    pub documents: Vec<DocumentPage>,
    pub lbl_layer_info: Label,
}

pub struct ArtistLibreApp {
    pub app: Application,
}

impl ArtistLibreApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("org.artistlibre.ArtistLibrePaint")
            .build();

        app.connect_activate(Self::build_ui);

        Self { app }
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("ArtistLibrePaint (GTK 4 / Rust)")
            .default_width(1200)
            .default_height(800)
            .build();

        let main_vbox = GtkBox::new(Orientation::Vertical, 0);

        let stack = Stack::new();
        let switcher = StackSwitcher::builder().stack(&stack).build();

        let header = HeaderBar::new();
        header.set_title_widget(Some(&switcher));
        window.set_titlebar(Some(&header));

        let notebook = Notebook::builder().vexpand(true).build();

        let lbl_layer_info = Label::new(Some("Calque actif : Arrière-plan"));

        let state = Rc::new(RefCell::new(AppState {
            doc_count: 0,
            documents: Vec::new(),
            lbl_layer_info: lbl_layer_info.clone(),
        }));

        // -------------------------------------------------------------
        // Onglet 1: Fichier
        // -------------------------------------------------------------
        let file_box = GtkBox::new(Orientation::Horizontal, 10);
        file_box.set_margin_start(10);
        file_box.set_margin_end(10);
        file_box.set_margin_top(5);
        file_box.set_margin_bottom(5);

        let btn_new = Button::with_label("[+] Nouveau Document");
        let state_new = state.clone();
        let notebook_new = notebook.clone();
        btn_new.connect_clicked(move |_| {
            Self::add_tab(&state_new, &notebook_new);
        });
        file_box.append(&btn_new);

        let btn_export = Button::with_label("💾 Exporter en PNG");
        let state_export = state.clone();
        let notebook_export = notebook.clone();
        btn_export.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_export.current_page() {
                let st = state_export.borrow();
                if (page_idx as usize) < st.documents.len() {
                    let doc = &st.documents[page_idx as usize];
                    let title = doc.state.borrow().title.clone();
                    let _ = doc.export_png(&title);
                }
            }
        });
        file_box.append(&btn_export);

        stack.add_titled(&file_box, Some("file"), "Fichier");

        // -------------------------------------------------------------
        // Onglet 2: Outils de dessin
        // -------------------------------------------------------------
        let tools_box = GtkBox::new(Orientation::Horizontal, 12);
        tools_box.set_margin_start(10);
        tools_box.set_margin_end(10);
        tools_box.set_margin_top(5);
        tools_box.set_margin_bottom(5);

        tools_box.append(&Label::new(Some("Outil :")));

        let btn_brush = Button::with_label("✏️ Pinceau");
        let state_brush = state.clone();
        let notebook_brush = notebook.clone();
        btn_brush.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_brush.current_page() {
                let st = state_brush.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].state.borrow_mut().tool = Tool::Brush;
                }
            }
        });
        tools_box.append(&btn_brush);

        let btn_eraser = Button::with_label("🧹 Gomme");
        let state_eraser = state.clone();
        let notebook_eraser = notebook.clone();
        btn_eraser.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_eraser.current_page() {
                let st = state_eraser.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].state.borrow_mut().tool = Tool::Eraser;
                }
            }
        });
        tools_box.append(&btn_eraser);

        tools_box.append(&Label::new(Some("Brosse :")));
        let brush_labels: Vec<&str> = BrushKind::ALL.iter().map(|(_, label)| *label).collect();
        let brush_dropdown = DropDown::from_strings(&brush_labels);
        brush_dropdown.set_selected(0);
        let state_brush_kind = state.clone();
        let notebook_brush_kind = notebook.clone();
        brush_dropdown.connect_selected_notify(move |dropdown| {
            let Some(item) = dropdown.selected_item() else { return };
            let Ok(item) = item.downcast::<StringObject>() else { return };
            let Some((kind, _)) = BrushKind::ALL
                .iter()
                .find(|(_, label)| item.string().as_str() == *label)
            else {
                return;
            };
            let Some(page_idx) = notebook_brush_kind.current_page() else { return };
            let st = state_brush_kind.borrow();
            if (page_idx as usize) < st.documents.len() {
                st.documents[page_idx as usize].state.borrow_mut().brush_kind = *kind;
            }
        });
        tools_box.append(&brush_dropdown);

        tools_box.append(&Label::new(Some("Couleur :")));

        let color_dialog = ColorDialog::new();
        let color_btn = ColorDialogButton::new(Some(color_dialog));
        color_btn.set_rgba(&RGBA::builder().red(0.0).green(0.0).blue(0.0).alpha(1.0).build());
        let state_color = state.clone();
        let notebook_color = notebook.clone();
        color_btn.connect_rgba_notify(move |cb| {
            let rgba = cb.rgba();
            if let Some(page_idx) = notebook_color.current_page() {
                let st = state_color.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].state.borrow_mut().brush_color = (
                        rgba.red() as f64,
                        rgba.green() as f64,
                        rgba.blue() as f64,
                        rgba.alpha() as f64,
                    );
                }
            }
        });
        tools_box.append(&color_btn);

        tools_box.append(&Label::new(Some("Taille :")));

        let adjustment = Adjustment::new(5.0, 1.0, 100.0, 1.0, 5.0, 0.0);
        let spin_size = SpinButton::new(Some(&adjustment), 1.0, 0);
        let state_size = state.clone();
        let notebook_size = notebook.clone();
        spin_size.connect_value_changed(move |sb| {
            let val = sb.value();
            if let Some(page_idx) = notebook_size.current_page() {
                let st = state_size.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].state.borrow_mut().brush_size = val;
                }
            }
        });
        tools_box.append(&spin_size);

        let btn_clear = Button::with_label("🗑️ Effacer le calque");
        let state_clear = state.clone();
        let notebook_clear = notebook.clone();
        btn_clear.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_clear.current_page() {
                let st = state_clear.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].clear_active_layer();
                }
            }
        });
        tools_box.append(&btn_clear);

        stack.add_titled(&tools_box, Some("tools"), "Outils de Dessin");

        // -------------------------------------------------------------
        // Onglet 3: Calques
        // -------------------------------------------------------------
        let layer_box = GtkBox::new(Orientation::Horizontal, 10);
        layer_box.set_margin_start(10);
        layer_box.set_margin_end(10);
        layer_box.set_margin_top(5);
        layer_box.set_margin_bottom(5);

        let btn_add_layer = Button::with_label("[+] Nouveau calque");
        let state_add_l = state.clone();
        let notebook_add_l = notebook.clone();
        btn_add_layer.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_add_l.current_page() {
                let st = state_add_l.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].add_layer(None);
                    Self::update_layer_info(&st, page_idx as usize);
                }
            }
        });
        layer_box.append(&btn_add_layer);

        let btn_del_layer = Button::with_label("[-] Supprimer calque");
        let state_del_l = state.clone();
        let notebook_del_l = notebook.clone();
        btn_del_layer.connect_clicked(move |_| {
            if let Some(page_idx) = notebook_del_l.current_page() {
                let st = state_del_l.borrow();
                if (page_idx as usize) < st.documents.len() {
                    st.documents[page_idx as usize].remove_active_layer();
                    Self::update_layer_info(&st, page_idx as usize);
                }
            }
        });
        layer_box.append(&btn_del_layer);

        layer_box.append(&lbl_layer_info);

        stack.add_titled(&layer_box, Some("layer"), "Calques");

        main_vbox.append(&stack);
        main_vbox.append(&notebook);

        window.set_child(Some(&main_vbox));

        // Ajouter un premier document par défaut
        Self::add_tab(&state, &notebook);

        window.present();
    }

    fn add_tab(state: &Rc<RefCell<AppState>>, notebook: &Notebook) {
        let mut st = state.borrow_mut();
        st.doc_count += 1;
        let title = format!("Image_{}.png", st.doc_count);
        let doc = DocumentPage::new(&title, 700, 500);

        let tab_hbox = GtkBox::new(Orientation::Horizontal, 6);
        tab_hbox.append(&Label::new(Some(&title)));

        let close_btn = Button::with_label("✕");
        close_btn.set_has_frame(false);
        tab_hbox.append(&close_btn);

        let page_idx = notebook.append_page(&doc.container, Some(&tab_hbox));
        notebook.set_tab_reorderable(&doc.container, true);

        st.documents.push(doc);

        notebook.set_current_page(Some(page_idx as u32));
        Self::update_layer_info(&st, page_idx as usize);
    }

    fn update_layer_info(st: &AppState, page_idx: usize) {
        if page_idx < st.documents.len() {
            let doc_st = st.documents[page_idx].state.borrow();
            if doc_st.active_layer_idx < doc_st.layers.len() {
                let layer = &doc_st.layers[doc_st.active_layer_idx];
                st.lbl_layer_info.set_label(&format!(
                    "Calque actif ({}/{}) : {}",
                    doc_st.active_layer_idx + 1,
                    doc_st.layers.len(),
                    layer.name
                ));
            }
        }
    }

    pub fn run(&self) -> glib::ExitCode {
        self.app.run()
    }
}
