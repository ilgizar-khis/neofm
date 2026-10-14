use app::App;
mod app;
mod ui;

fn main() -> Result<(), String> {
    let mut app = App::default();

    app.run().map_err(|e| e.to_string())
}
