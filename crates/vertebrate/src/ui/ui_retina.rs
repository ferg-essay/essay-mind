use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Color, Mesh2dColor};
use essay_tensor::tensor::Tensor;
use mind_ecs::PostTick;
use ui_graphics::ViewPlugin;
use crate::retina::Retina;

struct UiRetina {
    view: View<RetinaView>,
}

impl UiRetina {
    fn new(
        view: View<RetinaView>,
    ) -> Self {
        Self {
            view
        }
    }
}

fn ui_retina_update(
    mut ui_retina: ResMut<UiRetina>,
    retina: Res<Retina>
) {
    if let Some(left) = retina.data_left() {
        ui_retina.view.write(|v| { 
            v.left_colors = fill_colors(left);
        });
    }

    if let Some(right) = retina.data_right() {
        ui_retina.view.write(|v| { 
            v.right_colors = fill_colors(right);
        });
    }
}

fn fill_colors(data: Tensor) -> Tensor<u32> {
    assert_eq!(data.rows(), Retina::SIZE);
    assert_eq!(data.cols(), Retina::SIZE);

    let colors: Vec<u32> = data.as_slice().iter().map(|v| {
        Color::from_grey(*v).to_rgba()
    }).collect();

    Tensor::from(colors).reshape([Retina::SIZE, Retina::SIZE])
}

struct RetinaView {
    size: usize,

    left_colors: Tensor<u32>,

    right_colors: Tensor<u32>,

    pos_canvas: Bounds<Canvas>,
}

impl RetinaView {
    fn new(size: usize) -> Self {
        let mut colors = Vec::<u32>::new();
        colors.resize(size * size, Color::from("red").to_rgba()); 
        let colors = Tensor::from(colors);
        
        Self {
            size: size,

            left_colors: colors.clone(),

            right_colors: colors.clone(),

            pos_canvas: Bounds::none(),
        }
    }

    fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos_canvas = pos.clone();
    }
}

impl Drawable for RetinaView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        self.set_pos(ui.pos());
        let pos = ui.pos();

        let w = 0.5 * (pos.width() - 5.);
        let h = pos.height();

        let s = w.min(h);

        let y0 = pos.ymin() + 0.5 * (h - s);

        let pos_left = Bounds::<Canvas>::from(([pos.xmin(), y0], [s, s]));
        let left = build_grid(self.size, &pos_left, &self.left_colors);
        ui.draw_mesh2d_color(&left)?;


        let pos_right = Bounds::<Canvas>::from(([pos.xmin() + s + 5., y0], [s, s]));
        let right = build_grid(self.size, &pos_right, &self.right_colors);
        ui.draw_mesh2d_color(&right)?;

        
        Ok(())
    }
}

fn build_grid(size: usize, pos: &Bounds<Canvas>, colors: &Tensor<u32>) -> Mesh2dColor {
    let (x, y) = (pos.xmin(), pos.ymin());
    let (w, h) = (pos.width(), pos.height());

    let mut mesh = Mesh2dColor::new();

    let dw = w / size as f32;
    let dh = h / size as f32;

    for j in 0..size {
        for i in 0..size {
            let x0 = x + i as f32 * dw;
            let y0 = y + (size - j - 1) as f32 * dh;
            let x1 = x0 + dw;
            let y1 = y0 + dh;

            let color = Color(colors[(j, i)]);

            mesh.triangle(
                ([x0, y0], color),
                ([x0, y1], color),
                ([x1, y1], color),
            );

            mesh.triangle(
                ([x0, y0], color),
                ([x1, y0], color),
                ([x1, y1], color),
            );
        }
    }

    mesh
}

pub struct UiRetinaPlugin {
    // pos: Bounds<Layout>,
    view: Option<View<RetinaView>>,
}

impl UiRetinaPlugin {
    pub fn new() -> Self {
        Self {
            // pos: pos.into(),
            view: None,
        }
    }
}

impl ViewPlugin for UiRetinaPlugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc> {
        if let Some(retina) = app.get_resource::<Retina>() {
            let size = retina.get_size();

            self.view = Some(View::from(RetinaView::new(size)));
        }

        self.view.as_ref().map(|v| v.arc())
     }
}

impl Plugin for UiRetinaPlugin {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            app.insert_resource(UiRetina::new(view.clone()));

            app.system(PostTick, ui_retina_update);
        }
    }
}
