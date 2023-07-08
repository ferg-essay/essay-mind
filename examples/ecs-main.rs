use essay_ecs_core::base_app::BaseApp;

fn main() {
    let mut app = BaseApp::new();

    app.add_system(|| { println!("Hello, world"); });

    app.tick();
}