use ratatui::{
    Frame,
    layout::{
        Constraint::{self, Length},
        Layout, Rect,
    },
    style::{Color, Style},
    widgets::Block,
};

use crate::ui::focus::Focus;
use crate::ui::ui_style::{HL, HLGroup, UIStyle, gen_color};

mod focus;
mod ui_style;

pub struct UI {
    status: String,
    ratio: [u16; 3],
    focus: Focus,
    style: UIStyle,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            status: "app status".to_string(),
            // ratio of panels
            ratio: [1, 2, 3],
            // focused panel
            focus: Focus::List,
            style: UIStyle::default(),
        }
    }
}

impl UI {
    pub fn focus_next(&mut self) {
        self.focus = self.focus.next()
    }

    pub fn focus_prev(&mut self) {
        self.focus = self.focus.prev()
    }

    pub fn add_hl(&mut self, group: String, bg: Option<String>, fg: Option<String>) {
        if let Some(group) = HLGroup::from(group) {
            let hl = HL {
                bg: gen_color(bg),
                fg: gen_color(fg),
            };
            self.style.add_hl(group, hl);
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        // get areas of body and status
        let [body_area, status_area] =
            Layout::vertical([Constraint::Fill(1), Length(1)]).areas(frame.area());

        // render status text
        frame.render_widget(self.status.clone(), status_area);

        // get areas of side, list and view panels
        let body_area_list: [Rect; 3] =
            Layout::horizontal(self.ratio.map(|u| Constraint::Fill(u))).areas(body_area);

        // render panels
        for (i, area) in body_area_list.iter().enumerate() {
            let groups = if i == self.focus.clone() as usize {
                (HLGroup::ActiveBorder, HLGroup::ActivePanel)
            } else {
                (HLGroup::NormalBorder, HLGroup::NormalPanel)
            };
            let border_hl = self.style.get_hl(&groups.0);
            let panel_hl = self.style.get_hl(&groups.1);
            let border_style = Style::default().fg(border_hl.fg).bg(border_hl.bg);
            let panel_style = Style::default().fg(panel_hl.fg).bg(panel_hl.bg);
            frame.render_widget(
                Block::bordered()
                    .border_style(border_style)
                    .style(panel_style),
                *area,
            );
        }
    }
}
