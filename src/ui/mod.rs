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

mod focus;

pub struct UI {
    status: String,
    ratio: [u16; 3],
    focus: Focus,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            status: "app status".to_string(),
            // ratio of panels
            ratio: [1, 2, 3],
            // focused panel
            focus: Focus::List,
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
            let style = if i == self.focus.clone() as usize {
                Style::default().fg(Color::Red)
            } else {
                Style::default()
            };
            frame.render_widget(Block::bordered().border_style(style), *area);
        }
    }
}
