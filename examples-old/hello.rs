//use essaymind::my_test;

use log::{info, LevelFilter};

pub fn main() {
    env_logger::builder()
    .format_timestamp(None)
    .filter_level(LevelFilter::Info)
    .init();
    // main_loop.run().expect("run failed");
    info!("\nmy_test\n");
}