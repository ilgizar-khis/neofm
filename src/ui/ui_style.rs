use std::collections::HashMap;

use ratatui::style::Color;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum HLGroup {
    ActivePanel,
    NormalPanel,
    ActiveBorder,
    NormalBorder,
}

#[derive(Clone, Copy, Debug)]
pub struct HL {
    bg: Color,
    fg: Color,
}

impl Default for HL {
    fn default() -> Self {
        Self {
            bg: Color::default(),
            fg: Color::default(),
        }
    }
}
impl HL {
    pub fn reverse(&self) -> Self {
        Self {
            bg: self.fg,
            fg: self.bg,
        }
    }
}

fn gen_color(color: Option<String>) -> Color {
    if let Some(color) = color {
        if let Some(color_strip) = color.strip_prefix("x") {
            if let Ok(hex) = u32::from_str_radix(color_strip, 16) {
                return Color::Rgb(
                    ((hex >> 16) & 0xFF) as u8,
                    ((hex >> 8) & 0xFF) as u8,
                    (hex & 0xFF) as u8,
                );
            }
        } else {
            if let Ok(color) = color.parse() {
                return color;
            }
        }
    }
    Color::default()
}

pub struct UIStyle {
    colors: HashMap<HLGroup, HL>,
}

impl Default for UIStyle {
    fn default() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }
}

impl UIStyle {
    pub fn get_hl(&self, group: &HLGroup) -> HL {
        match self.colors.get(group) {
            Some(hl) => hl.clone(),
            None => HL::default(),
        }
    }
}
