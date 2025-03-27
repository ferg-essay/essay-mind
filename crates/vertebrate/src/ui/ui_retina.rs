use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::{renderer::{self, Canvas, Drawable, Renderer}, Bounds, Color};
use essay_tensor::Tensor;
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
    assert_eq!(data.rank(), 2);
    // assert_eq!(data.rows(), Retina::SIZE);
    // assert_eq!(data.cols(), Retina::SIZE);

    let mut colors = Vec::<u32>::new();

    for value in data.as_slice() {
        let color = Color::from_grey(*value).to_rgba();

        for _ in 0..4 {
            colors.push(color);
        }
    }

    Tensor::from(colors)
}

struct RetinaView {
    size: usize,

    left_vertices: Tensor,
    left_triangles: Tensor<u32>,
    left_colors: Tensor<u32>,

    right_vertices: Tensor,
    right_triangles: Tensor<u32>,
    right_colors: Tensor<u32>,

    pos_canvas: Bounds<Canvas>,
}

impl RetinaView {
    fn new(size: usize) -> Self {
        let (vertices, triangles) = build_grid(size, &Bounds::from([100., 100.]));

        let mut colors = Vec::<u32>::new();
        colors.resize(4 * size * size, Color::black().to_rgba()); 
        let colors = Tensor::from(colors);
        
        Self {
            size: size,

            left_vertices: vertices.clone(),
            left_triangles: triangles.clone(),
            left_colors: colors.clone(),

            right_vertices: vertices.clone(),
            right_triangles: triangles.clone(),
            right_colors: colors.clone(),

            pos_canvas: Bounds::none(),
        }
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        if pos == &self.pos_canvas {
            return;
        }

        let w = 0.5 * (pos.width() - 5.);
        let h = pos.height();

        let s = w.min(h);

        let y0 = pos.ymin() + 0.5 * (h - s);

        let pos_left = Bounds::<Canvas>::from(((pos.xmin(), y0), [s, s]));

        let (vertices, triangles) = build_grid(self.size, &pos_left);
        
        self.left_vertices = vertices;
        self.left_triangles = triangles;

        let pos_right = Bounds::<Canvas>::from(((pos.xmin() + s + 5., y0), [s, s]));

        let (vertices, triangles) = build_grid(self.size, &pos_right);
        
        self.right_vertices = vertices;
        self.right_triangles = triangles;

        self.pos_canvas = pos.clone();
    }
}

fn build_grid(size: usize, pos: &Bounds<Canvas>) -> (Tensor, Tensor<u32>) {
    let (x, y) = (pos.xmin(), pos.ymin());
    let (w, h) = (pos.width(), pos.height());

    let mut vertices = Vec::<[f32; 2]>::new();
    let mut triangles = Vec::<[u32; 3]>::new();

    let dw = w / size as f32;
    let dh = h / size as f32;

    for j in 0..size {
        for i in 0..size {
            let x0 = x + i as f32 * dw;
            let y0 = y + (size - j - 1) as f32 * dh;
            let x1 = x0 + dw;
            let y1 = y0 + dh;

            add_square(&mut vertices, &mut triangles, x0, y0, x1, y1);
        }
    }

    (Tensor::from(vertices), Tensor::from(triangles))
}

fn add_square(
    vertices: &mut Vec<[f32; 2]>, 
    triangles: &mut Vec<[u32; 3]>, 
    x0: f32, 
    y0: f32, 
    x1: f32, 
    y1: f32) {
    let v0 = vertices.len() as u32;

    vertices.push([x0, y0]);
    vertices.push([x1, y0]);
    vertices.push([x0, y1]);
    vertices.push([x1, y1]);

    triangles.push([v0 + 0, v0 + 1, v0 + 2]);
    triangles.push([v0 + 3, v0 + 2, v0 + 1]);
}

impl Drawable for RetinaView {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> renderer::Result<()> {
        self.set_pos(renderer.pos());

        renderer.draw_triangles(
            self.left_vertices.clone(), 
            self.left_colors.clone(), 
            self.left_triangles.clone(), 
        )?;

        renderer.draw_triangles(
            self.right_vertices.clone(), 
            self.right_colors.clone(), 
            self.right_triangles.clone(), 
        )?;
        
        Ok(())
    }
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
