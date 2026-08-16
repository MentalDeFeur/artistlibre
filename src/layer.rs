use cairo::{Context, Format, ImageSurface};

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub opacity: f64,
    pub surface: ImageSurface,
}

impl Layer {
    pub fn new(name: &str, width: i32, height: i32, fill_white: bool) -> Result<Self, cairo::Error> {
        let surface = ImageSurface::create(Format::ARgb32, width, height)?;

        if fill_white {
            let cr = Context::new(&surface)?;
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
        }

        Ok(Self {
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            surface,
        })
    }
}
