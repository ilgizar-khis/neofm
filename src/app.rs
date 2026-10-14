use crate::UI;
use crossterm::event;

pub struct App {
    ui: UI,
}

impl Default for App {
    fn default() -> Self {
        Self { ui: UI::default() }
    }
}

impl App {
    pub fn run(&mut self) -> std::io::Result<()> {
        ratatui::run(|term| {
            loop {
                term.draw(|frame| self.ui.render(frame))?;

                if event::read()?.is_key_press() {
                    break Ok(());
                }
            }
        })
    }
}
