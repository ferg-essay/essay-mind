
// type KeyArgs = (String,f32,f32);
//pub use ticker::SystemBuilder;
//pub use graphics::main_loop;

use ui_graphics::main_loop::{MainLoop};

pub fn my_test() {
    let mut main_loop = MainLoop::new();

    main_loop.run(move |ui| {}).expect("run failed");
    print!("\nmy_test\n");
}