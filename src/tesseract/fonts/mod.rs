use std::convert::Into;
use std::mem;
use std::sync::{Arc, LazyLock, Mutex};
use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::egui::FontFamily::{Monospace, Proportional};
use FontFamily::Name;

pub struct Font {
    name: String,
    scaling: f32,
    data: Arc<FontData>,
}

impl Font {
    pub fn new(name: String, scaling: f32, data: Arc<FontData>) -> Self {
        Self { name, scaling, data }
    }
}

type FontMutex = Mutex<Option<Arc<Font>>>;

static FONT_SCALING: Mutex<f32> = Mutex::new(1.0);
pub static PROPORTIONAL_FONT: FontMutex = Mutex::new(None);
pub static MONOSPACE_FONT: FontMutex = Mutex::new(None);
pub static TITLE_FONT: FontMutex = Mutex::new(None);

pub fn get_scale(mutex: &FontMutex) -> f32 {
    mutex.lock().unwrap().as_ref().unwrap().scaling
}
pub fn set_font(mutex: &FontMutex, font_name: &str) -> Result<(), String> {
    let font = find_font(font_name)?;
    let _ = mutex.lock().unwrap().insert(font);
    Ok(())
}
pub fn find_font(font_name: &str) -> Result<Arc<Font>, String> {
    let lock = FONTS.lock().unwrap();
    let font= lock.as_ref()
        .ok_or("Fonts are not initialized".to_owned())?
        .iter().find(|f| f.name == font_name)
        .ok_or("No such font found".to_owned())?
        .clone();

    Ok(font)
}

fn apply_font(defs: &mut FontDefinitions, family: FontFamily, font_mutex: &FontMutex) {
    let font = font_mutex.lock().unwrap().as_ref().unwrap().clone();
    defs.font_data.insert(font.name.clone(), font.data.clone());
    defs.families
        .entry(family)
        .or_default()
        .insert(0, font.name.clone());
}

pub fn title_family() -> FontFamily { Name(TITLE_FAMILY.clone()) }
static TITLE_FAMILY: LazyLock<Arc<str>> = LazyLock::new(|| "Title".into());

pub fn reload_active_fonts(ctx: &egui::Context) {
    let mut defs = FontDefinitions::default();

    apply_font(&mut defs, Proportional, &PROPORTIONAL_FONT);
    apply_font(&mut defs, Monospace, &MONOSPACE_FONT);
    apply_font(&mut defs, title_family(), &TITLE_FONT);

    ctx.set_fonts(defs);
}

pub static FONTS: Mutex<Option<Vec<Arc<Font>>>> = Mutex::new(None);
pub fn init_fonts() {
    let mut lock = FONTS.lock().unwrap();
    let fonts = lock.insert(vec![]);

    fonts.push(Arc::new(Font::new(
        "F25 Bank Printer".to_owned(), 1.,
        Arc::new(FontData::from_static(include_bytes!("f25_bank_printer.ttf")))
    )));

    fonts.push(Arc::new(Font::new(
        "Hack Regular".to_owned(), 0.75,
        Arc::new(FontData::from_static(include_bytes!("hack_regular.ttf")))
    )));

    fonts.push(Arc::new(Font::new(
        "Nouveau IBM".to_owned(), 1.,
        Arc::new(FontData::from_static(include_bytes!("nouveau_ibm.ttf")))
    )));

    fonts.push(Arc::new(Font::new(
        "White Rabbit".to_owned(), 1.,
        Arc::new(FontData::from_static(include_bytes!("white_rabbit.ttf")))
    )));

    fonts.push(Arc::new(Font::new(
        "Xanmono Regular".to_owned(), 1.,
        Arc::new(FontData::from_static(include_bytes!("xanmono_regular.ttf")))
    )));

    fonts.push(Arc::new(Font::new(
        "Slice".to_owned(), 1.,
        Arc::new(FontData::from_static(include_bytes!("slice.ttf")))
    )))
}