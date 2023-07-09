use essay_ecs_core::ResMut;
use essay_plot::artist::PathStyle;
use essay_plot_base::{Path, Canvas, PathCode, Point};
use ui_graphics::ui_canvas::UiCanvas;

pub fn ui_panel(mut renderer: ResMut<UiCanvas>) {
    let path = Path::<Canvas>::new(vec![
        PathCode::MoveTo(Point(100., 100.)),
        PathCode::LineTo(Point(400., 100.)),
        PathCode::ClosePoly(Point(200., 300.)),
    ]);

    let mut style = PathStyle::new();
    style.color("azure");
    
    renderer.draw_path(&path, &style);
}