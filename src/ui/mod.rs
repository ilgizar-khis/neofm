use ratatui::{
    Frame,
    layout::{
        Constraint::{self, Length},
        Layout,
    },
};

pub struct UI {
    title: String,
    status: String,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            title: "app title".to_string(),
            status: "app status".to_string(),
        }
    }
}

impl UI {
    pub fn render(&mut self, frame: &mut Frame) {
        let [title_area, _, status_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1), Length(1)])
                .areas(frame.area());

        frame.render_widget(self.title.clone(), title_area);
        frame.render_widget(self.status.clone(), status_area);
    }
}
