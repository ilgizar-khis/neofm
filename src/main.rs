use app::App;
use ui::UI;
mod app;
mod ui;

fn main() -> Result<(), String> {
    let mut app = App::default();

    app.run().map_err(|e| e.to_string())
}
