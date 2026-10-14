use ratatui::{
    Frame,
    layout::{
        Constraint::{self, Length},
        Layout,
    },
    widgets::Block,
};

pub struct UI {
    status: String,
    ratio: [u16; 3],
}

impl Default for UI {
    fn default() -> Self {
        Self {
            status: "app status".to_string(),
            // ratio of panels
            ratio: [1, 2, 3],
        }
    }
}

impl UI {
    pub fn render(&mut self, frame: &mut Frame) {
        // get areas of body and status
        let [body_area, status_area] =
            Layout::vertical([Constraint::Fill(1), Length(1)]).areas(frame.area());

        // render status text
        frame.render_widget(self.status.clone(), status_area);

        // get areas of side, list and view panels
        let [side_area, list_area, view_area] =
            Layout::horizontal(self.ratio.map(|u| Constraint::Fill(u))).areas(body_area);

        // render panels
        frame.render_widget(Block::bordered(), side_area);
        frame.render_widget(Block::bordered(), list_area);
        frame.render_widget(Block::bordered(), view_area);
    }
}
