use eframe::egui::{include_image, Image, ImageSource};
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;

#[macro_use]
mod icon_macros;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Icon {
    RedFlag,
    IncorrectFlag,
    Mine,
}
impl Icon {
    pub fn snake(&self) -> &'static str {
        match self {
            Icon::RedFlag => "red_flag",
            Icon::IncorrectFlag => "incorrect_flag",
            Icon::Mine => "mine",
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum IconStyle {
    Classic,
    Modern,
    HighRes,
}
impl IconStyle {
    pub fn snake(&self) -> &'static str {
        match self {
            IconStyle::Classic => "classic",
            IconStyle::Modern => "modern",
            IconStyle::HighRes => "high_res",
        }
    }
}

static ICONS: [(&str, [(&str, ImageSource); 3]); 3] = load_icons!(
    red_flag,incorrect_flag,mine; 
    classic,modern,high_res
);
static STYLE: Mutex<IconStyle> = Mutex::new(IconStyle::Modern);

type IconMap = HashMap<(Icon, IconStyle), ImageSource<'static>>;

pub fn icon(icon: Icon) -> Image<'static> {
    let icon_snake = icon.snake();
    let style_snake = STYLE.lock().unwrap().snake();
    
    let src = ICONS
        .iter().find(|(style, _)| *style == style_snake).unwrap().1
        .iter().find(|(icon, _)| *icon == icon_snake).unwrap().1.clone();
    
    Image::new(src)
}


pub fn set_style(style: IconStyle) { _ = mem::replace(&mut *STYLE.lock().unwrap(), style); }


