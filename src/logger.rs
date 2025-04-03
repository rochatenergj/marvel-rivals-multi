use log::{info, error};

pub fn init_logger() {
    simple_logger::init().unwrap();
}

pub fn log_feature_toggle(feature: &str, status: bool) {
    if status {
        info!("{} has been enabled.", feature);
    } else {
        info!("{} has been disabled.", feature);
    }
}