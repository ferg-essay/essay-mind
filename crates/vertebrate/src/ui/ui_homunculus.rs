use std::cell::RefCell;

use mind_ecs::AfterTicks;
use renderer::{Canvas, Drawable, Event, Renderer};
use essay_ecs::prelude::*;
use essay_graphics::layout::{Layout, View};
use essay_plot::{
    prelude::*, 
    artist::{paths::Unit, PathStyle, ColorMaps, ColorMap}
};

use ui_graphics::{UiCanvas, ui_canvas::UiRender};
use crate::{
    body::Body, hab_taxis::Taxis, hind_motor::HindLevyMove, mid_motor::tectum::TectumMap, util::Turn 
};
use crate::ui::ui_world_map::UiWorldPlugin;
use crate::util::{Angle, Heading};

use super::ui_emoji::Emoji;

#[derive(Component)]
pub struct UiHomunculus {
    view: View<UiHomunculusView>,

    next_emoji: Emoji,
    next_emoji_id: usize,
}

impl UiHomunculus {
    pub const N_DIR : usize = 12;

    fn new(view: View<UiHomunculusView>) -> Self {
        Self {
            view,

            next_emoji: Emoji::FaceThinking,
            next_emoji_id: usize::MAX,
        }
    }

    fn emoji(&mut self, id: usize, emoji: Emoji) {
        if id < self.next_emoji_id {
            self.next_emoji = emoji;
            self.next_emoji_id = id;
        }
    }
}

pub fn ui_homunculus_draw(
    mut ui_homunculus: ResMut<UiHomunculus>,
    body: Res<Body>,
    hind_taxis: Res<HindLevyMove>,
    taxis: Res<Taxis>,
    tectum: Res<TectumMap>,
) {
    let next_emoji = ui_homunculus.next_emoji;

    let mut left_delta = hind_taxis.get_left_delta();
    if body.is_collide_left() { 
        left_delta = 1.;
    }

    let mut right_delta = hind_taxis.get_right_delta();
    if body.is_collide_right() { 
        right_delta = 1.;
    }

    let approach_dir = taxis.approach_dir();
    let value = approach_dir.value();
    let n = UiHomunculus::N_DIR;
    let approach_values = approach_vec(n, body.head_dir(), value);

    ui_homunculus.view.write(|v| {
        v.body_turn = body.turn();
        v.body_head_dir = body.head_dir();

        v.next_emoji = next_emoji;

        v.left_delta = left_delta;
        v.right_delta = right_delta;
        v.forward_delta = hind_taxis.get_forward_delta();
        v.is_collide_left = body.is_collide_left();
        v.is_collide_right = body.is_collide_right();

        v.tectum_values = tectum.values();
        v.approach_values = approach_values;
    });
}

struct UiHomunculusView {
    body_head_dir: Heading,
    body_turn: Turn,

    left_delta: f32,
    right_delta: f32,
    forward_delta: f32,

    is_collide_left: bool,
    is_collide_right: bool,

    next_emoji: Emoji,

    tectum_values: Vec<f32>,
    approach_values: Vec<f32>,

    bounds: Bounds<Unit>,
    pos: Bounds<Canvas>,

    paths_unit: UiHomunculusPath<Unit>,
    paths_canvas: UiHomunculusPath<Canvas>,

    head_dir: HeadDir,
    inner_dir: HeadDir,
    outer_dir: HeadDir,

    emoji_pos: Point,

    colors: ColorMap,
    emoji: Option<FontTypeId>,
}

impl UiHomunculusView {
    fn new() -> Self {
        let paths_unit = UiHomunculusPath::<Unit>::new();
        let affine = Affine2d::eye();
        let paths_canvas = paths_unit.transform(&affine);

        let mut inner_dir = HeadDir::new(UiHomunculus::N_DIR, 1. - 2. * HeadDir::WIDTH);
        inner_dir.set_colors(approach_colormap());
        inner_dir.set_head(false);
        
        let mut outer_dir = HeadDir::new(UiHomunculus::N_DIR, 1.);
        outer_dir.set_colors(ColorMaps::OrangeBlue.into());
        outer_dir.set_head(false);

        Self {
            body_head_dir: Heading::Unit(0.),
            body_turn: Turn::Unit(0.),

            next_emoji: Emoji::Crab,
            approach_values: Vec::new(),
            tectum_values: Vec::new(),

            left_delta: 0.,
            right_delta: 0.,
            forward_delta: 0.,

            is_collide_left: false,
            is_collide_right: false,

            bounds: Bounds::unit(),
            pos: Bounds::zero(),

            paths_unit,
            paths_canvas,

            head_dir: HeadDir::new(UiHomunculus::N_DIR, 1. - HeadDir::WIDTH),
            inner_dir,
            outer_dir,

            emoji_pos: Point(0., 0.),

            colors: sensorimotor_colormap(),
            emoji: None,
        }
    }

    fn resize(&mut self, pos: &Bounds<Canvas>) {
        // let pos = self.view.pos();

        self.pos = Bounds::from((
            pos.xmin() + 0.05 * pos.width(),
            pos.ymin() + 0.05 * pos.height(),
            pos.xmax() - 0.05 * pos.width(),
            pos.ymax() - 0.05 * pos.height()
        ));

        let to_canvas = self.to_canvas();

        // self.clip = Clip::from(&self.pos);

        self.paths_canvas = self.paths_unit.transform(&to_canvas);

        self.head_dir.set_pos(&pos);
        self.outer_dir.set_pos(&pos);
        self.inner_dir.set_pos(&pos);

        self.emoji_pos = self.to_canvas().transform_point(Point(0.5, 0.75));
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }
}

impl Drawable for UiHomunculusView {
    fn draw(&mut self, ui: &mut dyn Renderer) -> renderer::Result<()> {
        let turn = self.body_turn.to_unit(); // (body.turn().to_unit() + 0.5) % 1.;

        let paths = &self.paths_canvas;

        let mut style = PathStyle::new();
        style.edge_color("black");
        style.face_color(self.colors.map(0.5));

        ui.draw_path(&paths.outline, &style)?;

        style.edge_color(self.colors.map(1.));
        style.face_color(self.colors.map(1.));

        if self.left_delta != 0.5 {
            let color = self.colors.map(self.left_delta);

            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.ss_ul, &style)?;
        }

        if self.right_delta != 0.5 {
            let color = self.colors.map(self.right_delta);

            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.ss_ur, &style)?;
        }

        if self.is_collide_left { 
            ui.draw_path(&paths.ss_ul, &style)?;
        }

        if self.is_collide_right { 
            ui.draw_path(&paths.ss_ur, &style)?;
        }

        style.edge_color(self.colors.map(0.2));
        style.face_color(self.colors.map(0.2));
        style.edge_color("dark green");
        style.face_color("dark green");

        let turn_left = turn.clamp(0., 0.5) * 2.;
        let turn_right = turn.clamp(0.5, 1.) * 2. - 1.;

        if 0. < turn_left && turn_left < 1. {
            ui.draw_path(&paths.mo_lr, &style)?;
        }

        if turn_right > 0. {
            ui.draw_path(&paths.mo_ll, &style)?;
        }

        if self.forward_delta != 0.5 {
            let color = self.colors.map(self.forward_delta);
            style.edge_color(color);
            style.face_color(color);
            ui.draw_path(&paths.u_turn, &style)?;
        }

        let n = self.head_dir.paths.len();

        let value = 0.75;
        let values = head_dir_vec(n, self.body_head_dir, value);
        self.head_dir.draw(ui, &values)?;

        self.inner_dir.draw(ui, &self.approach_values)?;
        self.outer_dir.draw(ui, &self.tectum_values)?;

        // let path_style = PathStyle::new();

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Center);
        text_style.size(14.);
        text_style.font(self.emoji.unwrap());

        let state = self.next_emoji;

        let path_style = PathStyle::new();

        let mut text_style = TextStyle::new();
        text_style.valign(VertAlign::Center);
        text_style.size(14.);
        text_style.font(self.emoji.unwrap());

        ui.draw_text(self.emoji_pos, state.code(), 0., &path_style, &text_style)
    }

    fn event(&mut self, ui: &mut dyn Renderer, event: &Event) {
        if let Event::Resize(pos) = event {
            if self.emoji.is_none() {
                let emoji_family = "/Users/ferg/wsp/essay-mind/assets/font/NotoEmoji-Bold.ttf";

                let mut style = FontStyle::new();
        
                style.family(emoji_family);
        
                self.emoji = Some(ui.font(&style).unwrap());
            }

            self.resize(pos);
        }
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

fn _avoid_colormap() -> ColorMap {
    ColorMap::from(ColorMaps::WhiteRed)
}

fn approach_colormap() -> ColorMap {
    ColorMap::from(ColorMaps::WhiteBlue)
    //ColorMap::from(ColorMaps::BlueOrange)
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
    pub const _MIN : f32 = 0.05;

    fn new(n: usize, radius: f32) -> Self {
        assert!(n > 0);
        assert!(n % 2 == 0);

        let h = 2. * UiHomunculusPath::<Unit>::H - 1.;
        let h = h - 0.05;
        // let h = UiHomunculusPath::<Unit>::H;

        let unit = Bounds::<Unit>::new((-1., -1.), (1., 1.));
        
        let pos = Bounds::<Unit>::new(
            //(-0.25, h - 0.2),
            //(0.25, h + 0.2)
            (-0.5, h - 0.2),
            (0.5, h + 0.2)
        );

        let to_pos = unit.affine_to(&pos);

        //let a_2 = TAU / n as f32;
        let a_2 = 1. / n as f32;
        let h1 = radius;
        let h2 = radius - Self::WIDTH;

        let mut unit_paths = Vec::new();

        for i in 0..n {
            // TODO: unit match degree
            let a0 = Angle::Unit(0.25 - i as f32 * a_2); // - a_2 / 2.;
            let a1 = Angle::Unit(0.25 - (i as f32 + 1.) * a_2);

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

    fn draw<'a>(&self, ui: &mut dyn Renderer, values: &Vec<f32>) -> renderer::Result<()> {
        let mut style = PathStyle::new();

        for (path, value) in self.paths.iter().zip(values) {
            style.color(self.colors.map(*value));
            ui.draw_path(path, &style)?;
        }

        Ok(())
    }

    fn _draw<'a>(&self, ui: &mut UiRender<'a>, dir: Angle, value: f32) {
        let mut style = PathStyle::new();
    
        let da = 1. / self.paths.len() as f32;
    
        for (i, path) in self.paths.iter().enumerate() {
            let angle = Angle::Unit((i as f32 + 0.5) * da + dir.to_unit());
    
            if self.is_head {
                let cos = angle.cos().max(0.);
                let v = value * cos * cos.abs().powi(3);
    
                if Self::_MIN <= v {
                    style.color(self.colors.map(v));
                    ui.draw_path(path, &style);
                }
            } else {
                if Self::_MIN <= value || true {
                    let v = (value * angle.cos()).clamp(0., 1.);
    
                    style.color(self.colors.map(v));
                    ui.draw_path(path, &style);
                }
            };
        }
    }
}

fn head_dir_vec(n: usize, dir: Heading, value: f32) -> Vec<f32> {
    let da = 1. / n as f32;
    let mut vec = Vec::new();

    for i in 0..n {
        // todo: unit direction issues
        let angle = Angle::Unit(0.25 - (i as f32 + 0.5) * da + dir.to_unit());

        let cos = angle.cos().max(0.);
        let v = value * cos * cos.abs().powi(3);

        vec.push(v);
    }

    vec
}

fn approach_vec(n: usize, dir: Heading, value: f32) -> Vec<f32> {
    let da = 1. / n as f32;
    let mut vec = Vec::new();

    for i in 0..n {
        let angle = Angle::Unit((i as f32 + 0.5) * da + dir.to_unit());

        let v = (value * angle.cos()).clamp(0., 1.);

        vec.push(v * 0.5);
    }

    vec
}

pub trait UiState {
    fn draw(&self, ui: &mut UiRender, pos: Point, style: &mut TextStyle);
}

//
// UiHomunculusPlugin
//

pub struct UiHomunculusPlugin {
    bounds: Bounds::<Layout>,

    emoji_items: Vec<Box<dyn PluginItem>>,
}

impl UiHomunculusPlugin {
    pub fn new(xy: impl Into<Point>, wh: impl Into<Point>) -> Self {
        let xy = xy.into();
        let wh = wh.into();

        Self {
            bounds: Bounds::new(xy, (xy.0 + wh.0, xy.1 + wh.1)),
            emoji_items: Vec::new(),
        }
    }

    pub fn item<T>(
        mut self, 
        emoji: Emoji,
        fun: impl Fn(&T) -> bool + Send + Sync + 'static
    ) -> Self
    where T: Default + Send + Sync + 'static
    {
        self.emoji_items.push(Box::new(Item {
            emoji,
            fun: RefCell::new(Some(Box::new(fun)))
        }));

        self
    }
}

struct Item<T: Send + Sync + 'static> {
    emoji: Emoji,
    fun: RefCell<Option<Box<dyn Fn(&T) -> bool + Send + Sync + 'static>>>,
}

trait PluginItem {
    fn system(&self, id: usize, app: &mut App);
}

impl<T: Default + Send + Sync + 'static> PluginItem for Item<T> {
    fn system(&self, id: usize, app: &mut App) {
        app.init_resource::<T>();

        let fun = self.fun.take().unwrap();
        let emoji = self.emoji;

        app.system(PostUpdate, 
            move |mut hom: ResMut<UiHomunculus>, item: Res<T>| {
                if fun(item.get()) {
                    hom.emoji(id, emoji);
                }
            }
        );
    }
}

impl Plugin for UiHomunculusPlugin {
    fn build(&self, app: &mut App) {
        if app.contains_plugin::<UiWorldPlugin>() {
            let view = UiHomunculusView::new();
            let view = app.resource_mut::<UiCanvas>().view(self.bounds.clone(), view);

            let ui_homunculus = UiHomunculus::new(view);

            app.init_resource::<Taxis>();

            app.insert_resource(ui_homunculus);

            for (i, item) in self.emoji_items.iter().enumerate() {
                item.system(i, app);

            }

            app.system(AfterTicks, ui_homunculus_draw);
        }
    }
}
