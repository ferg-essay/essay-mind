use essay_plot::artist::PathStyle;
use essay_ecs::prelude::*;
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path};

use ui_graphics::{backend::{ScreenApi, RendererApi, CanvasState}, ui_canvas::ui_main};

pub fn ecs_main() {
    let mut app = App::new();

    app.add_system(Update, || println!("tick2"));

    ui_main(app);
}

struct Screen {
    app: App,
}

impl ScreenApi for Screen {
    fn tick(&mut self) {
        // self.app.tick();
    }

    fn draw(&mut self, canvas: &mut dyn RendererApi) {
        let mut style = PathStyle::new();
        style.color("azure");

        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(20., 20.)),
            PathCode::LineTo(Point(400., 20.)),
            PathCode::LineTo(Point(400., 40.)),
            PathCode::ClosePoly(Point(20., 40.)),
        ]);

        canvas.draw_path(&path, &style);
    }

    fn event(&mut self, canvas: &CanvasState, event: &CanvasEvent) {
        println!("Event {:?}", event);
    }
}