use std::collections::HashMap;

use ratatui::style::Color;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum HLGroup {
    ActivePanel,
    NormalPanel,
    ActiveBorder,
    NormalBorder,
}

impl HLGroup {
    pub fn from(from: String) -> Option<Self> {
        match from.to_lowercase().as_str() {
            "active_panel" | "activepanel" => Some(Self::ActivePanel),
            "normal_panel" | "normalpanel" => Some(Self::NormalPanel),
            "active_border" | "activeborder" => Some(Self::ActiveBorder),
            "normal_border" | "normalborder" => Some(Self::NormalBorder),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HL {
    pub bg: Color,
    pub fg: Color,
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

pub fn gen_color(color: Option<String>) -> Color {
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

    pub fn add_hl(&mut self, group: HLGroup, hl: HL) {
        self.colors.insert(group, hl);
    }
}
