use log::info;
mod utils;
fn main() {
    info!("Starting Mokaccino...");
    print!("{}",utils::terminal::user_interface::BANNER);
}
