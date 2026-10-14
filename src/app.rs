use crossterm::event;

pub struct App {
    pub title: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "Hello world".to_string(),
        }
    }
}

impl App {
    pub fn run(&mut self) -> std::io::Result<()> {
        ratatui::run(|term| {
            loop {
                term.draw(|frame| frame.render_widget(self.title.clone(), frame.area()))?;

                if event::read()?.is_key_press() {
                    break Ok(());
                }
            }
        })
    }
}
