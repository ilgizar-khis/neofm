use app::App;
use ui::UI;
mod app;
mod ui;

fn main() -> Result<(), String> {
    let mut app = App::default();
    app.ui
        .add_hl("active_border".to_string(), None, Some("red".to_string()));
    app.run().map_err(|e| e.to_string())
}
