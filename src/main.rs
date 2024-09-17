use mokaccino::ui::terminal::App;

fn main() {
    let mut app = App::new().unwrap();
    app.run();
    app.exit();
}
