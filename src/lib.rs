
// type KeyArgs = (String,f32,f32);
//pub use ticker::SystemBuilder;
//pub use graphics::main_loop;

use graphics::main_loop::{MainLoop};

pub fn my_test() {
    let mut main_loop = MainLoop::builder().build();

    main_loop.run().expect("run failed");
    print!("\nmy_test\n");
}