use crate::UI;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

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

                if self.handle_keys()? {
                    break Ok(());
                }
            }
        })
    }

    fn handle_keys(&mut self) -> std::io::Result<bool> {
        let Event::Key(key) = event::read()? else {
            return Ok(false);
        };
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Tab => self.ui.focus_next(),
                KeyCode::BackTab => self.ui.focus_prev(),
                _ => {}
            }
        }
        Ok(false)
    }
}
