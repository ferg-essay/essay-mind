use ui_graphics::main_loop;

fn main() {
    print!("hello\n");
    let builder = main_loop::MainLoop::new();
    builder.run().expect("run failed");
}
