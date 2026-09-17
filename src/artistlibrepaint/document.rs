use crate::layer::Layer;
use cairo::{Context, Format, ImageSurface, LineCap, LineJoin, Operator};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, DrawingArea, GestureDrag, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Brush,
    Eraser,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BrushKind {
    Round,
    Pencil,
    Ink,
    Marker,
    Airbrush,
    Spray,
    Calligraphy,
    Chalk,
}

impl BrushKind {
    pub const ALL: [(Self, &'static str); 8] = [
        (Self::Round, "Pinceau rond"),
        (Self::Pencil, "Crayon"),
        (Self::Ink, "Encre"),
        (Self::Marker, "Marqueur"),
        (Self::Airbrush, "Aérographe"),
        (Self::Spray, "Spray"),
        (Self::Calligraphy, "Calligraphie"),
        (Self::Chalk, "Craie"),
    ];

}

pub struct DocumentState {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub layers: Vec<Layer>,
    pub active_layer_idx: usize,
    pub tool: Tool,
    pub brush_kind: BrushKind,
    pub brush_size: f64,
    pub brush_color: (f64, f64, f64, f64), // RGBA
    pub last_x: f64,
    pub last_y: f64,
}

#[derive(Clone)]
pub struct DocumentPage {
    pub container: GtkBox,
    pub drawing_area: DrawingArea,
    pub state: Rc<RefCell<DocumentState>>,
}

impl DocumentPage {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);

        let bg_layer = Layer::new("Arrière-plan", width, height, true)
            .expect("Échec de création du calque d'arrière-plan");

        let state = Rc::new(RefCell::new(DocumentState {
            title: title.to_string(),
            width,
            height,
            layers: vec![bg_layer],
            active_layer_idx: 0,
            tool: Tool::Brush,
            brush_kind: BrushKind::Round,
            brush_size: 5.0,
            brush_color: (0.0, 0.0, 0.0, 1.0),
            last_x: 0.0,
            last_y: 0.0,
        }));

        let scrolled = ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .build();

        let drawing_area = DrawingArea::builder()
            .content_width(width)
            .content_height(height)
            .build();

        // Configurer la fonction de dessin Cairo
        let state_clone = state.clone();
        drawing_area.set_draw_func(move |_area, cr, _w, _h| {
            let st = state_clone.borrow();
            // Fond neutre
            cr.set_source_rgb(0.7, 0.7, 0.7);
            let _ = cr.paint();

            // Composition des calques visibles
            for layer in &st.layers {
                if layer.visible {
                    let _ = cr.set_source_surface(&layer.surface, 0.0, 0.0);
                    let _ = cr.paint_with_alpha(layer.opacity);
                }
            }
        });

        // Configurer le GestureDrag pour la capture de la souris
        let drag = GestureDrag::new();

        let state_drag_begin = state.clone();
        let da_begin = drawing_area.clone();
        drag.connect_drag_begin(move |_gesture, start_x, start_y| {
            let mut st = state_drag_begin.borrow_mut();
            st.last_x = start_x;
            st.last_y = start_y;
            Self::perform_stroke(&mut st, start_x, start_y, start_x, start_y);
            da_begin.queue_draw();
        });

        let state_drag_update = state.clone();
        let da_update = drawing_area.clone();
        let drag_clone = drag.clone();
        drag.connect_drag_update(move |_gesture, offset_x, offset_y| {
            let mut st = state_drag_update.borrow_mut();
            let (start_x, start_y) = drag_clone.start_point().unwrap_or((st.last_x, st.last_y));
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;
            let last_x = st.last_x;
            let last_y = st.last_y;
            Self::perform_stroke(&mut st, last_x, last_y, current_x, current_y);
            st.last_x = current_x;
            st.last_y = current_y;
            da_update.queue_draw();
        });

        drawing_area.add_controller(drag);

        scrolled.set_child(Some(&drawing_area));
        container.append(&scrolled);

        Self {
            container,
            drawing_area,
            state,
        }
    }

    fn perform_stroke(st: &mut DocumentState, x1: f64, y1: f64, x2: f64, y2: f64) {
        if st.active_layer_idx >= st.layers.len() {
            return;
        }
        let layer = &st.layers[st.active_layer_idx];
        if !layer.visible {
            return;
        }

        if let Ok(cr) = Context::new(&layer.surface) {
            match st.tool {
                Tool::Eraser => {
                    cr.set_operator(Operator::Clear);
                }
                Tool::Brush => {
                    cr.set_operator(Operator::Over);
                    let (r, g, b, a) = st.brush_color;
                    cr.set_source_rgba(r, g, b, a);
                }
            }

            match st.brush_kind {
                BrushKind::Round => Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size, LineCap::Round),
                BrushKind::Pencil => {
                    cr.set_source_rgba(st.brush_color.0, st.brush_color.1, st.brush_color.2, st.brush_color.3 * 0.65);
                    Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size * 0.7, LineCap::Round);
                }
                BrushKind::Ink => Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size * 1.25, LineCap::Round),
                BrushKind::Marker => {
                    cr.set_source_rgba(st.brush_color.0, st.brush_color.1, st.brush_color.2, st.brush_color.3 * 0.35);
                    Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size * 2.4, LineCap::Square);
                }
                BrushKind::Calligraphy => Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size * 0.55, LineCap::Butt),
                BrushKind::Chalk => {
                    cr.set_source_rgba(st.brush_color.0, st.brush_color.1, st.brush_color.2, st.brush_color.3 * 0.55);
                    cr.set_dash(&[st.brush_size * 0.45, st.brush_size * 0.7], 0.0);
                    Self::draw_line(&cr, x1, y1, x2, y2, st.brush_size * 1.3, LineCap::Round);
                    cr.set_dash(&[], 0.0);
                }
                BrushKind::Airbrush => Self::draw_soft_dots(&cr, x1, y1, x2, y2, st.brush_size, false),
                BrushKind::Spray => Self::draw_soft_dots(&cr, x1, y1, x2, y2, st.brush_size, true),
            }
        }
    }

    fn draw_line(cr: &Context, x1: f64, y1: f64, x2: f64, y2: f64, width: f64, cap: LineCap) {
        cr.set_line_cap(cap);
        cr.set_line_join(LineJoin::Round);
        cr.set_line_width(width.max(1.0));
        cr.move_to(x1, y1);
        cr.line_to(x2, y2);
        let _ = cr.stroke();
    }

    fn draw_soft_dots(cr: &Context, x1: f64, y1: f64, x2: f64, y2: f64, size: f64, spray: bool) {
        let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        let steps = (distance / (size.max(1.0) * 0.35)).ceil().max(1.0) as usize;
        let dots = if spray { 10 } else { 4 };
        for step in 0..=steps {
            let progress = step as f64 / steps as f64;
            let x = x1 + (x2 - x1) * progress;
            let y = y1 + (y2 - y1) * progress;
            for dot in 0..dots {
                let angle = (step * dots + dot) as f64 * 2.399;
                let radius = if spray { size * (0.5 + (dot % 5) as f64 * 0.35) } else { size * 0.35 };
                let dot_x = x + angle.cos() * radius;
                let dot_y = y + angle.sin() * radius;
                cr.arc(dot_x, dot_y, (size * if spray { 0.12 } else { 0.3 }).max(0.5), 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            }
        }
    }

    pub fn add_layer(&self, name: Option<&str>) {
        let mut st = self.state.borrow_mut();
        let layer_num = st.layers.len() + 1;
        let layer_name = name.map(String::from).unwrap_or_else(|| format!("Calque {}", layer_num));
        if let Ok(new_layer) = Layer::new(&layer_name, st.width, st.height, false) {
            st.layers.push(new_layer);
            st.active_layer_idx = st.layers.len() - 1;
            self.drawing_area.queue_draw();
        }
    }

    pub fn remove_active_layer(&self) {
        let mut st = self.state.borrow_mut();
        if st.layers.len() > 1 {
            let active_idx = st.active_layer_idx;
            st.layers.remove(active_idx);
            if st.active_layer_idx >= st.layers.len() {
                st.active_layer_idx = st.layers.len() - 1;
            }
            self.drawing_area.queue_draw();
        }
    }

    pub fn clear_active_layer(&self) {
        let st = self.state.borrow();
        let active_idx = st.active_layer_idx;
        if active_idx < st.layers.len() {
            let layer = &st.layers[active_idx];
            if let Ok(cr) = Context::new(&layer.surface) {
                cr.set_operator(Operator::Clear);
                let _ = cr.paint();
                cr.set_operator(Operator::Over);
                if active_idx == 0 {
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    let _ = cr.paint();
                }
            }
            self.drawing_area.queue_draw();
        }
    }

    pub fn export_png(&self, filepath: &str) -> Result<(), cairo::Error> {
        let st = self.state.borrow();
        let composite = ImageSurface::create(Format::ARgb32, st.width, st.height)?;
        let cr = Context::new(&composite)?;
        for layer in &st.layers {
            if layer.visible {
                cr.set_source_surface(&layer.surface, 0.0, 0.0)?;
                cr.paint_with_alpha(layer.opacity)?;
            }
        }
        let mut file = std::fs::File::create(filepath).map_err(|_| cairo::Error::WriteError)?;
        composite.write_to_png(&mut file).map_err(|_| cairo::Error::WriteError)?;
        println!("Exporté avec succès : {}", filepath);
        Ok(())
    }
}
