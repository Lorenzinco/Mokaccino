use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();
    info!("Trying to print a message to the console");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    //println!("Hello, world!");
}
