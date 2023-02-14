//use essaymind::my_test;

use ui_graphics::main_loop::MainLoop;
use log::{info, LevelFilter};

pub fn main() {
    env_logger::builder()
    .format_timestamp(None)
    .filter_level(LevelFilter::Info)
    .init();

 
    let mut main_loop = MainLoop::new().run(move |ui| {
        
    }).expect("run failed");

    // main_loop.run().expect("run failed");
    info!("\nmy_test\n");
}