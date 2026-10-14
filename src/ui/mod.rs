use ratatui::{
    Frame,
    layout::{
        Constraint::{self, Length},
        Layout,
    },
};

pub struct UI {
    status: String,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            status: "app status".to_string(),
        }
    }
}

impl UI {
    pub fn render(&mut self, frame: &mut Frame) {
        let [_, status_area] =
            Layout::vertical([Constraint::Fill(1), Length(1)]).areas(frame.area());

        frame.render_widget(self.status.clone(), status_area);
    }
}
