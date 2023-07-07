use essay_plot::artist::PathStyle;
use essay_plot_base::{CanvasEvent, PathCode, Canvas, Point, Path};

use crate::backend::{ScreenApi, RendererApi, main_loop, CanvasState};

pub fn ui_main() {
    let screen = Screen { count: 0 };

    main_loop(screen);
    println!("Hello");
}

struct Screen {
    count: usize,
}

impl ScreenApi for Screen {
    fn tick(&mut self, canvas: &dyn RendererApi) {
        let count = self.count;
        self.count = count + 1;

        //println!("Tick {:?}", count);
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