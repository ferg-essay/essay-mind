use essay_ecs::core::Query;
use essay_ecs::prelude::*;
use essay_graphics::layout::{View, ViewArc};
use essay_plot::api::path_style::MeshStyle;
use essay_plot::{prelude::*, artist::paths};
use essay_tensor::tensor::Tensor;
use renderer::{Canvas, Drawable, Renderer};
use ui_graphics::ViewPlugin;

use crate::world::{Food, FoodKind, Odor, OdorInnate, OdorKind, World, WorldPlugin};

use crate::world::Wall;

pub fn draw_world(
    world: Res<World>, 
    odors: Query<&Odor<OdorKind>>, 
    foods: Query<&Food>,
    mut ui_world: ResMut<UiWorld>, 
    // mut ui_canvas: ResMut<UiCanvas>
) {
    ui_world.view.write(|v| v.image(world.get()));

    let mut xy : Vec<[f32; 2]> = Vec::new();
    let mut sizes : Vec<[f32; 2]> = Vec::new();

    for odor in odors.iter() {
        xy.push([odor.x(), odor.y()]);
        sizes.push([odor.r(), odor.r()]);
    }

        /* todo();
        let xy = ui_world.to_canvas().transform(&Tensor::from(xy));

        if xy.len() > 0 {
            ui.draw_markers(&circle, xy, sizes, &colors);
        }
        */

    let mut xy : Vec<[f32; 2]> = Vec::new();
    let mut sizes : Vec<[f32; 2]> = Vec::new();
    let mut colors : Vec<u32> = Vec::new();

    for food in foods.iter() {
        let pos = food.pos();

        xy.push([pos.x(), pos.y()]);
        sizes.push([food.radius(), food.radius()]);

        let color = match food.kind() {
            FoodKind::None => Color::from("black"),
            // FoodKind::Plain => Color::from("pumpkin orange"),
            FoodKind::Poor => Color::from("grey"),
            FoodKind::Plain => Color::from("teal"),
            FoodKind::Sweet => Color::from("cherry red"),
            FoodKind::Bitter => Color::from("mustard yellow"),
            FoodKind::Sick => Color::from("brownish green"),
        };
        colors.push(color.to_rgba());
    }

    let food = UiFood {
        xy: xy.into(),
        sizes: sizes.into(),
        colors: colors.into(),
    };

    ui_world.view.write(|v| v.food = Some(food));
}

#[derive(Component)]
pub struct UiWorld {
    view: View<UiWorldView>,
}

impl UiWorld {
    fn new(
        view: View<UiWorldView>, 
        width: usize, 
        height: usize,
        // hex: UiWorldHex<OdorKind>,
    ) -> Self {
        let mut values = Vec::new();

        values.resize_with(width * height, || Wall::Empty);

        Self {
            view,
        }
    }
    /*
    pub fn hex(&mut self) -> &mut UiWorldHex<OdorKind> {
        &mut self.hex
    }
    */

    fn pos(&self) -> Bounds<Canvas> {
        self.view.read(|v| v.pos.clone())    
    }

    pub fn bounds(&self) -> Bounds<UiWorld> {
        self.view.read(|v| v.bounds.clone())    
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.view.read(|v| v.to_canvas.clone())    
    }

    pub fn to_canvas_scale(&self) -> Affine2d {
        let pos = self.pos();
        let bounds = self.bounds();

        Affine2d::eye().scale(
            pos.width() / bounds.width(), 
            pos.height() / bounds.height(),
        )
    }

    pub fn clip(&self) -> &Clip {
        // &self.clip
        &Clip::None
    }

    // pub fn view_id(&self) -> ViewId {
    //    self.view.id()
    // }

    pub fn update(&mut self, _world: &World, _renderer: &mut dyn Renderer) {
        // self.hex.update_render(renderer, world.hex());
    }

    pub fn draw(&mut self, _renderer: &mut dyn Renderer, _camera: &Affine2d) -> renderer::Result<()> {
        // self.hex.draw(renderer, camera)
        Ok(())
    }
}

impl Coord for UiWorld {}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawWorld;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawItem;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Phase)]
pub struct DrawAgent;

pub struct UiWorldView {
    bounds: Bounds<UiWorld>,
    pos: Bounds<Canvas>,
    cache_pos: Bounds<Canvas>,

    clip: Clip,
    to_canvas: Affine2d,
    to_canvas_view: Affine2d,

    colors: Option<Tensor<u8>>,
    image: Option<TextureId>,
    food: Option<UiFood>,

    food_x: f32,
}

impl UiWorldView {
    fn new(bounds: impl Into<Bounds<UiWorld>>) -> Self {
        Self {
            bounds: bounds.into(),
            pos: Bounds::zero(),
            cache_pos: Bounds::zero(),

            colors: None,
            image: None,
            food: None,
            food_x: 0.,

            clip: Clip::None,
            to_canvas: Affine2d::eye(),
            to_canvas_view: Affine2d::eye(),
        }
    }

    fn image(&mut self, world: &World) {
        if self.colors.is_none() {
            let mut vec = Vec::<[u8; 4]>::new();
    
            for j in 0..world.height() {
                for i in 0..world.width() {
                    vec.push(Color::from(&world[(i, j)]).to_rgba_vec());
                }
            }
    
            let colors = Tensor::from(&vec);
            let colors = colors.reshape([world.height() as usize, world.width() as usize, 4]);

            self.colors = Some(colors);
    
            // let image = ui_canvas.create_image(colors);
            // ui_world.image = image.clone();
            // ui_world.view.write(|v| v.image = image);
        }
    
    }

    fn resize(&mut self, pos: &Bounds<Canvas>) {
        if &self.cache_pos == pos {
            return;
        }

        self.cache_pos = pos.clone();

        let aspect = self.bounds.width() / self.bounds.height();

        // force bounds to match aspect ratio
        let (c_width, c_height) = if aspect * pos.height() <= pos.width() {
            (aspect * pos.height(), pos.height())
        } else {
            (pos.width(), pos.width() / aspect)
        };

        // center the box
        let xmin = pos.xmin() + 0.5 * (pos.width() - c_width);
        let ymin = pos.ymin() + 0.5 * (pos.height() - c_height);

        let xmin = xmin.max(10.);
        let ymin = ymin.max(10.);

        //let xmin = pos.xmin();
        //let ymin = pos.ymin();

        let c_width = c_width - xmin - pos.xmin();
        let c_height = c_height - xmin - pos.xmin();

        let pos = Bounds::<Canvas>::new(
            Point(xmin, ymin),
            Point(xmin + c_width, ymin + c_height),
        );

        self.pos = pos;
        self.clip = Clip::from(&self.pos);
        let extent = Bounds::<Canvas>::from([xmin + c_width, ymin + c_height]);
        self.to_canvas = self.bounds.affine_to(&self.pos);
        self.to_canvas_view = self.bounds.affine_to(&extent);
    }
}

impl Drawable for UiWorldView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        let pos = ui.pos().clone();

        self.resize(&pos);

        if self.image.is_none() {
            if let Some(colors) = &self.colors {
                self.image = Some(ui.create_texture_rgba8(colors));
            }
            // ui_world.image = image.clone();
            // ui_world.view.write(|v| v.image = image);
        }
            // todo!()

        if let Some(image) = &self.image {
            let mut mesh = Mesh2d::new();

            let (x0, y0) = (pos.xmin(), pos.ymin());
            let (x1, y1) = (pos.xmax(), pos.ymax());

            mesh.triangle_uv(
                ([x0, y0], [0., 0.]),
                ([x0, y1], [0., 1.]),
                ([x1, y1], [1., 1.]),
            );

            mesh.triangle_uv(
                ([x0, y0], [0., 0.]),
                ([x1, y0], [1., 0.]),
                ([x1, y1], [1., 1.]),
            );

            ui.draw_mesh2d(&mesh, *image, &[Color::white().with_alpha(1.).into()])?;
        }

        if let Some(food) = &self.food {
            if food.xy.len() > 0 {
                //println!("XY {:?}", food.xy);
                let xy = &self.to_canvas.transform(&food.xy);

                let star: Path<Canvas> = paths::unit_star(8, 0.6)
                    .transform(&self.to_canvas_view);

                if self.food_x != food.xy[0] {
                    self.food_x = food.xy[0];
                }
                let style = PathStyle::new();
                // TODO:
                //ui.draw_markers(&star, &style, &xy, &food.sizes, &food.colors, &style)?;
                ui.draw_markers(&star, &style, &food.to_marker_style(&self.to_canvas))?;
        
            }
        }

        Ok(())
    }
}

struct UiFood {
    xy: Tensor,
    sizes: Tensor,
    colors: Tensor<u32>,
}

impl UiFood {
    fn to_marker_style(&self, to_canvas: &Affine2d) -> Vec<MeshStyle> {
        self.xy.iter_row().enumerate().map(|(i, xy)| {
            let size_len = self.sizes.len();
            let size = self.sizes[i % size_len];

            let color_len = self.colors.len();
            let color = Color(self.colors[i % color_len]);

            let Point(x, y) = to_canvas.transform_point(Point(xy[0], xy[1]));

            MeshStyle {
                affine: affine2d::scale(size, size)
                    .translate(x, y),
                color,
            }
        }).collect()
    }
}

impl From<&Wall> for Color {
    fn from(value: &Wall) -> Self {
        match value {
            Wall::Empty => Color::from_hsv(0.25, 0.0, 0.98).with_alpha(0.1),
            Wall::Food => Color::from("green"),
            Wall::Wall => Color::from("dark beige"),

            Wall::FloorLight => Color(0xf8f8f800),
            Wall::FloorDark => Color::from(0x606060),
        }
    }
}

impl From<OdorInnate> for Color {
    fn from(value: OdorInnate) -> Self {
        match value {
            OdorInnate::Food => Color::from("green"),
            OdorInnate::Avoid => Color::from("red"),
            OdorInnate::None => Color::from("purple"),
        }
    }
}

pub struct UiWorldPlugin {
    view: Option<View<UiWorldView>>,
}

impl UiWorldPlugin {
    pub fn new() -> Self {
        Self {
            view: None,
        }
    }

    pub fn none(&mut self, _key: OdorKind) {
        // self.hex.none(key);
    }

    //pub fn tile(&mut self, key: OdorKind) -> TileBuilder {
        // self.hex.tile(key)
    //}
}

impl ViewPlugin for UiWorldPlugin {
    fn view(&mut self, app: &mut App) -> Option<&ViewArc> {
        let (width, height) = {
            let world = app.get_plugin::<WorldPlugin>().unwrap();
            (world.width(), world.height())
        };

        let view = UiWorldView::new([width as f32, height as f32]);

        self.view = Some(View::from(view));

        self.view.as_ref().map(|v| v.arc())

    }
}

impl Plugin for UiWorldPlugin {
    fn build(&self, app: &mut App) {
        if let Some(view) = &self.view {
            assert!(app.contains_plugin::<WorldPlugin>());

            let (width, height) = {
                let world = app.get_plugin::<WorldPlugin>().unwrap();
                (world.width(), world.height())
            };
    
            let ui_world = UiWorld::new(view.clone(), width, height);

            app.insert_resource(ui_world);

            app.phase(Update, (DrawWorld, DrawItem, DrawAgent).chain());
            app.system(Update, draw_world.phase(DrawWorld));
            // app.system(PreUpdate, world_resize);

            // app.system(Startup, spawn_ui_world);
        }
    }
}