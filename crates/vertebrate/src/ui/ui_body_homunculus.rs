use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{paths::Unit, PathStyle, ColorMaps, ColorMap}
};

use ui_graphics::{ui_layout::{BoxId, UiLayout, UiLayoutEvent}, UiCanvas, ui_canvas::UiRender};
use crate::{body::Body, locomotor::taxis_pons::TaxisPons};
use crate::ui::ui_world::UiWorldPlugin;
use crate::util::Angle;

#[derive(Component)]
pub struct UiHomunculus {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Unit>,

    paths_unit: UiHomunculusPath<Unit>,
    paths_canvas: UiHomunculusPath<Canvas>,

    head_dir: HeadDir,
    goal_dir: HeadDir,

    colors: ColorMap,
    _head_dir_colors: ColorMap,
    avoid_colors: ColorMap,
}

impl UiHomunculus {
    pub const N_DIR : usize = 12;

    pub fn new(id: BoxId) -> Self {
        let paths_unit = UiHomunculusPath::<Unit>::new();
        let affine = Affine2d::eye();
        let paths_canvas = paths_unit.transform(&affine);
        let mut goal_dir = HeadDir::new(Self::N_DIR, 1. - HeadDir::WIDTH);
        goal_dir.set_colors(avoid_colormap());
        goal_dir.set_head(false);

        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            bounds: Bounds::from([1., 1.]),

            paths_unit,
            paths_canvas,

            head_dir: HeadDir::new(Self::N_DIR, 1.),
            goal_dir: goal_dir,

            colors: sensorimotor_colormap(),
            _head_dir_colors: head_colormap(),
            avoid_colors: avoid_colormap(),
        }
    }

    pub fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.pos = Bounds::from([
            pos.xmin() + 0.05 * pos.width(),
            pos.ymin() + 0.05 * pos.height(),
            pos.xmax() - 0.05 * pos.width(),
            pos.ymax() - 0.05 * pos.height()
        ]);

        self.clip = Clip::from(&self.pos);

        self.paths_canvas = self.paths_unit.transform(&self.to_canvas());

        self.head_dir.set_pos(pos);
        self.goal_dir.set_pos(pos);
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
}

fn head_colormap() -> ColorMap {
    // ColorMap::from(ColorMaps::BlueOrange)
    ColorMap::from([
        ColorMap::from(ColorMaps::BlueOrange).map(0.5),
        Color::from("midnight blue")
    ])
}

fn sensorimotor_colormap() -> ColorMap {
    ColorMap::from(ColorMaps::BlueOrange)
}

fn avoid_colormap() -> ColorMap {
    ColorMap::from(ColorMaps::WhiteRed)
}

struct UiHomunculusPath<C: Coord> {
    outline: Path<C>,

    ss_ul: Path<C>,
    ss_ur: Path<C>,
    ss_ll: Path<C>,
    ss_lr: Path<C>,

    mo_ul: Path<C>,
    mo_ur: Path<C>,
    mo_ll: Path<C>,
    mo_lr: Path<C>,

    u_turn: Path<C>,
}

impl<C: Coord> UiHomunculusPath<C> {
    const W: f32 = 0.05;
    const H: f32 = 0.75;
    
    fn new() -> UiHomunculusPath<Unit> {
        UiHomunculusPath {
            outline: outline(0),

            ss_ul: corner_ul(0),
            ss_ur: corner_ur(0),
            ss_ll: corner_ll(0),
            ss_lr: corner_lr(0),

            mo_ul: corner_ul(1),
            mo_ur: corner_ur(1),
            mo_ll: corner_ll(1),
            mo_lr: corner_lr(1),

            u_turn: u_turn(1),
        }
    }

    fn transform<D: Coord>(&self, to_canvas: &Affine2d) -> UiHomunculusPath<D> {
        UiHomunculusPath {
            outline: self.outline.transform(to_canvas),

            ss_ul: self.ss_ul.transform(to_canvas),
            ss_ur: self.ss_ur.transform(to_canvas),
            ss_ll: self.ss_ll.transform(to_canvas),
            ss_lr: self.ss_lr.transform(to_canvas),

            mo_ul: self.mo_ul.transform(to_canvas),
            mo_ur: self.mo_ur.transform(to_canvas),
            mo_ll: self.mo_ll.transform(to_canvas),
            mo_lr: self.mo_lr.transform(to_canvas),

            u_turn: self.u_turn.transform(to_canvas),
        }
    }
}

impl Coord for UiHomunculus {}

pub fn ui_homunculus_resize(
    mut ui_homunculus: ResMut<UiHomunculus>, 
    ui_layout: Res<UiLayout>,
    mut read: InEvent<UiLayoutEvent>
) {
    for _ in read.iter() {
        let id = ui_homunculus.id;
        ui_homunculus.set_pos(ui_layout.get_box(id));
    }
}

pub fn ui_homunculus_draw(
    ui_homunculus: ResMut<UiHomunculus>,
    body: Res<Body>,
    explore: Res<TaxisPons>,
    mut ui_canvas: ResMut<UiCanvas>
) {
    if let Some(mut ui) = ui_canvas.renderer(Clip::None) {
        let turn = body.turn().to_unit(); // (body.turn().to_unit() + 0.5) % 1.;

        let paths = &ui_homunculus.paths_canvas;

        let mut style = PathStyle::new();
        style.edge_color("black");
        style.face_color(ui_homunculus.colors.map(0.5));

        ui.draw_path(&paths.outline, &style);

        style.edge_color(ui_homunculus.colors.map(1.));
        style.face_color(ui_homunculus.colors.map(1.));

        if body.is_collide_left() { 
            ui.draw_path(&paths.ss_ul, &style);
        }

        if body.is_collide_right() { 
            ui.draw_path(&paths.ss_ur, &style);
        }

        style.edge_color(ui_homunculus.colors.map(0.2));
        style.face_color(ui_homunculus.colors.map(0.2));

        let turn_left = turn.clamp(0., 0.5) * 2.;
        let turn_right = turn.clamp(0.5, 1.) * 2. - 1.;

        if 0. < turn_left && turn_left < 1. {
            ui.draw_path(&paths.mo_lr, &style);
        }

        if turn_right > 0. {
            ui.draw_path(&paths.mo_ll, &style);
        }

        if explore.avoid_forward() > 0. {
            let color = ui_homunculus.avoid_colors.map(explore.avoid_forward());
            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.u_turn, &style);
        }

        if explore.avoid_left() > 0. {
            let color = ui_homunculus.avoid_colors.map(explore.avoid_left());

            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.ss_ll, &style);
        }

        if explore.avoid_right() > 0. {
            let color = ui_homunculus.avoid_colors.map(explore.avoid_right());

            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.ss_lr, &style);
        }

        let value = 0.75;
        ui_homunculus.head_dir.draw(&mut ui, body.head_dir(), value);

        let goal_dir = body.goal_dir();
        ui_homunculus.goal_dir.draw(&mut ui, goal_dir.dir(), goal_dir.value());
    }
}

fn corner_ul(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let w1 = (i as f32 + 1.) * w;

    Path::<Unit>::move_to(0.5, 1. - w0)
        //.bezier2_to((0. + w0, 1. - w0), (0. - w0, 0.75))
        .line_to(0. - w0, h)
        .line_to(0. + w1, h)
        //.bezier2_to((0. + w1, 1. - w1),(0.5, 1. - w1))
        .line_to(0.5, 1. - w1)
        .close_poly(0.5, 1. - w0).into()
}

fn corner_ur(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let w1 = (i as f32 + 1.) * w;

    Path::<Unit>::move_to(0.5, 1. - w1)
        //.bezier2_to((1. - w1, 1. - w1), (1. - w1, 0.75))
        .line_to(1. - w1, h)
        .line_to(1. - w0, h)
        //.bezier2_to((1. - w0, 1. - w0),(0.5, 1. - w0))
        .line_to(0.5, 1. - w0)
        .close_poly(0.5, 1. - w1).into()
}

fn corner_ll(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let w1 = (i as f32 + 1.) * w;

    Path::<Unit>::move_to(0. + w0, h)
        //.bezier2_to((w0, w0), (0.5, w0))
        .line_to(0.5, w0)
        .line_to(0.5, w1)
        //.bezier2_to((w1, w1),(w1, h))
        .line_to(w1, h)
        .close_poly(w0, h).into()
}

fn corner_lr(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let w1 = (i as f32 + 1.) * w;

    Path::<Unit>::move_to(0.5, w0)
        //.bezier2_to((1. - w1, 1. - w1), (1. - w1, 0.75))
        .line_to(1. - w0, h)
        .line_to(1. - w1, h)
        //.bezier2_to((1. - w0, 1. - w0),(0.5, 1. - w0))
        .line_to(0.5, w1)
        .close_poly(0.5, w0).into()
}

fn outline(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;

    Path::<Unit>::move_to(0.5, w0)
        //.bezier2_to((1. - w1, 1. - w1), (1. - w1, 0.75))
        .line_to(1. - w0, h)
        .line_to(0.5, 1. - w0)
        //.bezier2_to((1. - w0, 1. - w0),(0.5, 1. - w0))
        .line_to(w0, h)
        .close_poly(0.5, w0).into()
}

fn u_turn(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let h0 = 1. - w0;

    let p = 0.7;

    let x0 = p * 0.5 + (1. - p) * w0;
    let h1 = p * h0 + (1. - p) * h;

    Path::<Unit>::move_to(0.5, h0)
        //.bezier2_to((0. + w0, 1. - w0), (0. - w0, 0.75))
        .line_to(x0, h1)
        .close_poly(1. - x0, h1).into()
}

//
// HeadDir
//

struct HeadDir {
    unit: Bounds<Unit>,
    _pos: Bounds<Unit>,
    to_pos: Affine2d,

    unit_paths: Vec<Path<Unit>>,
    paths: Vec<Path<Canvas>>,

    is_head: bool,

    colors: ColorMap,
}

impl HeadDir {
    pub const WIDTH : f32 = 0.1;
    pub const MIN : f32 = 0.05;

    fn new(n: usize, radius: f32) -> Self {
        assert!(n > 0);
        assert!(n % 2 == 0);

        let unit = Bounds::<Unit>::new((-1., -1.), (1., 1.));
        let pos = Bounds::<Unit>::new(
            (-0.5, 0.15),
            (0.5, 0.65)
        );

        let to_pos = unit.affine_to(&pos);

        //let a_2 = TAU / n as f32;
        let a_2 = 1. / n as f32;
        let h1 = radius;
        let h2 = radius - Self::WIDTH;

        let mut unit_paths = Vec::new();

        for i in 0..n {
            let a0 = Angle::Unit(i as f32 * a_2); // - a_2 / 2.;
            let a1 = Angle::Unit((i as f32 + 1.) * a_2);

            let (x0, y0) = a0.sin_cos();
            let (x1, y1) = a1.sin_cos();

            let path = Path::<Unit>::move_to(x0 * h1, y0 * h1)
                .line_to(x1 * h1, y1 * h1)
                .line_to(x1 * h2, y1 * h2)
                .close_poly(x0 * h2, y0 * h2)
                .to_path();

            unit_paths.push(path);
        }

        let colors = head_colormap();

        Self {
            unit,
            _pos: pos,
            to_pos,
            unit_paths,
            is_head: true,
            paths: Vec::new(),
            colors,
        }
    }

    fn set_head(&mut self, is_head: bool) {
        self.is_head = is_head;
    }

    fn set_colors(&mut self, colors: ColorMap) {
        self.colors = colors;
    }

    fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        let to_canvas = self.unit.affine_to(pos).matmul(&self.to_pos);

        let mut paths = Vec::new();

        for path in &self.unit_paths {
            paths.push(path.transform(&to_canvas));
        }

        self.paths = paths;
    }

    fn draw<'a>(&self, ui: &mut UiRender<'a>, dir: Angle, value: f32) {
        let mut style = PathStyle::new();
        // style.edge_color("midnight blue");
        //style.face_color("white");

        // let da = TAU / self.paths.len() as f32;
        let da = 1. / self.paths.len() as f32;

        // let dir = TAU * dir;

        for (i, path) in self.paths.iter().enumerate() {
            let angle = Angle::Unit((i as f32 + 0.5) * da + dir.to_unit());

            if self.is_head {
                let cos = angle.cos().max(0.);
                let v = value * cos * cos.abs().powi(3);

                if Self::MIN <= v {
                    style.color(self.colors.map(v));
                    ui.draw_path(path, &style);
                }
            } else {
                if Self::MIN <= value {
                    let v = (value * angle.cos()).clamp(0., 1.);

                    style.color(self.colors.map(v));
                    ui.draw_path(path, &style);
                }
            };

            // let v = v.clamp(0., 1.) * 0.5 + 0.5;

        }
    }
}

//
// UiHomunculusPlugin

pub struct UiHomunculusPlugin {
    bounds: Bounds::<UiLayout>,
}

impl UiHomunculusPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
        }
    }
}

impl Plugin for UiHomunculusPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let box_id = app.resource_mut::<UiLayout>().add_box(self.bounds.clone());

            let ui_homunculus = UiHomunculus::new(box_id);

            app.insert_resource(ui_homunculus);

            app.system(PreUpdate, ui_homunculus_resize);
            app.system(Update, ui_homunculus_draw);
        }
    }
}
