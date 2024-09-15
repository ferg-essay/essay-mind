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
    body::Body, 
    hind_brain::{HindMove, MoveKind}, 
    taxis::Taxis, 
    tectum::TectumMap,
    util::Turn 
};
use crate::ui::ui_world_map::UiWorldPlugin;
use crate::util::{Angle, Heading};

use super::ui_emoji::Emoji;

#[derive(Component)]
pub struct UiHomunculus {
    view: View<UiHomunculusView>,

    emojis: Vec<EmojiValue>,
}

impl UiHomunculus {
    pub const N_DIR : usize = 12;

    fn new(view: View<UiHomunculusView>) -> Self {
        Self {
            view,

            emojis: Vec::new(),
        }
    }

    fn emoji(&mut self, id: usize, emoji: Emoji, is_active: bool) {
        let value = EmojiValue { emoji, is_active };

        if self.emojis.len() <= id {
            self.emojis.resize(id + 1, value);
        } else {
            self.emojis[id] = value;
        }
    }

    fn next_emoji(&self) -> Option<Emoji> {
        for value in &self.emojis {
            if value.is_active {
                return Some(value.emoji);
            }
        }

        None
    }
}

#[derive(Clone)]
struct EmojiValue {
    emoji: Emoji,
    is_active: bool,
}

pub fn ui_homunculus_draw(
    mut ui_homunculus: ResMut<UiHomunculus>,
    body: Res<Body>,
    hind_move: Res<HindMove>,
    taxis: Res<Taxis>,
    tectum: Res<TectumMap>,
) {
    let next_emoji = if let Some(next_emoji) = ui_homunculus.next_emoji() {
        next_emoji
    } else {
        match hind_move.action_kind() {
            MoveKind::None => { Emoji::FaceThinking },
            MoveKind::Halt => { Emoji::Candy }, // TODO:
            MoveKind::Roam => { Emoji::Footprints },
            MoveKind::Seek => { Emoji::DirectHit },
            MoveKind::Avoid => { Emoji::NoEntry },
            MoveKind::Escape(_) | MoveKind::UTurn(_) => { 
                Emoji::NoEntry }
            MoveKind::Startle => {
                Emoji::FaceOpenMouth
            }
        }
    };

    let approach_dir = taxis.approach_dir();
    let value = approach_dir.value();
    let n = UiHomunculus::N_DIR;
    let approach_values = approach_vec(n, body.head_dir(), value);

    ui_homunculus.view.write(|v| {
        v.body_turn = body.turn();
        v.body_head_dir = body.head_dir();

        v.next_emoji = next_emoji;

        v.mo_freeze = hind_move.is_freeze();

        v.mo_forward = hind_move.mo_forward();
        v.mo_left = hind_move.mo_left();
        v.mo_right = hind_move.mo_right();

        v.ss_forward = hind_move.ss_forward();
        v.ss_head_left = hind_move.ss_left();
        v.ss_head_right = hind_move.ss_right();

        v.tectum_values = tectum.values();
        v.approach_values = approach_values;
    });
}

struct UiHomunculusView {
    body_head_dir: Heading,
    body_turn: Turn,

    ss_forward: f32,
    ss_head_left: f32,
    ss_head_right: f32,

    mo_freeze: bool,

    mo_forward: f32,
    mo_left: f32,
    mo_right: f32,

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
    motor_colors: ColorMap,
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

            ss_forward: 0.,
            ss_head_left: 0.,
            ss_head_right: 0.,

            mo_freeze: false,

            mo_left: 0.,
            mo_right: 0.,
            mo_forward: 0.,

            bounds: Bounds::unit(),
            pos: Bounds::zero(),

            paths_unit,
            paths_canvas,

            head_dir: HeadDir::new(UiHomunculus::N_DIR, 1. - HeadDir::WIDTH),
            inner_dir,
            outer_dir,

            emoji_pos: Point(0., 0.),

            colors: sensorimotor_colormap(),
            motor_colors: motor_colormap(),
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
        let paths = &self.paths_canvas;

        let mut style = PathStyle::new();
        style.color(self.colors.map(0.5));

        ui.draw_path(&paths.outline, &style)?;

        if self.ss_forward > 0. { 
            let color = self.colors.map(self.ss_forward);

            style.color(color);

            ui.draw_path(&paths.ss_fwd, &style)?;
        }

        if self.ss_head_left > 0. { 
            let color = self.colors.map(self.ss_head_left);

            style.color(color);

            ui.draw_path(&paths.ss_ul, &style)?;
        }

        if self.ss_head_right > 0. { 
            let color = self.colors.map(self.ss_head_right);

            style.color(color);

            ui.draw_path(&paths.ss_ur, &style)?;
        }

        if self.mo_freeze {
            style.color(Color::black());

            ui.draw_path(&paths.mo_tail, &style)?;
            ui.draw_path(&paths.ss_ll, &style)?;
            ui.draw_path(&paths.ss_lr, &style)?;
        } else {
            if self.mo_forward > 0. {
                style.color(self.motor_colors.map(self.mo_forward));

                ui.draw_path(&paths.mo_tail, &style)?;
            }

            if self.mo_left > 0. {
                let color = self.colors.map(self.mo_left);

                style.color(color);

                ui.draw_path(&paths.ss_ll, &style)?;
            }

            if self.mo_right > 0. {
                let color = self.colors.map(self.mo_right);

                style.color(color);

                ui.draw_path(&paths.ss_lr, &style)?;
            }
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

        ui.draw_text(self.emoji_pos, state.code(), 0., &path_style, &text_style)?;

        let mut style = PathStyle::new();
        style.edge_color("black");
        style.face_color(Color::none());

        ui.draw_path(&paths.outline, &style)?;

        Ok(())
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

fn motor_colormap() -> ColorMap {
    ColorMap::from([
        (0., Color::white()),
        // (0.25, Color::from("azure")),
        //(0.25, Color::from("azure")),
        (0.5, Color::from_hsv(0.66, 0.98, 0.65)), // "cobalt blue" 
        (0.51, Color::from_hsv(0.8, 0.9, 0.4)),
        (0.75, Color::from("orange")),
        // (1., Color::from("tomato")),
        (1., Color::from("red")),
    ])
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

    ss_fwd: Path<C>,

    ss_ul: Path<C>,
    ss_ur: Path<C>,
    ss_ll: Path<C>,
    ss_lr: Path<C>,

    mo_ul: Path<C>,
    mo_ur: Path<C>,
    mo_ll: Path<C>,
    mo_lr: Path<C>,
    mo_tail: Path<C>,

    u_turn: Path<C>,
}

impl<C: Coord> UiHomunculusPath<C> {
    const W: f32 = 0.05;
    const H: f32 = 0.75;
    
    fn new() -> UiHomunculusPath<Unit> {
        UiHomunculusPath {
            outline: outline(0),

            ss_fwd: corner_fwd(0),
            ss_ul: corner_ul(0),
            ss_ur: corner_ur(0),
            ss_ll: corner_ll(0),
            ss_lr: corner_lr(0),

            mo_ul: corner_ul(1),
            mo_ur: corner_ur(1),
            mo_ll: corner_ll(0),
            mo_lr: corner_lr(0),
            mo_tail: corner_tail(1),

            u_turn: corner_fwd(1),
        }
    }

    fn transform<D: Coord>(&self, to_canvas: &Affine2d) -> UiHomunculusPath<D> {
        UiHomunculusPath {
            outline: self.outline.transform(to_canvas),

            ss_fwd: self.ss_fwd.transform(to_canvas),
            ss_ul: self.ss_ul.transform(to_canvas),
            ss_ur: self.ss_ur.transform(to_canvas),
            ss_ll: self.ss_ll.transform(to_canvas),
            ss_lr: self.ss_lr.transform(to_canvas),

            mo_ul: self.mo_ul.transform(to_canvas),
            mo_ur: self.mo_ur.transform(to_canvas),
            mo_ll: self.mo_ll.transform(to_canvas),
            mo_lr: self.mo_lr.transform(to_canvas),
            mo_tail: self.mo_tail.transform(to_canvas),

            u_turn: self.u_turn.transform(to_canvas),
        }
    }
}

impl Coord for UiHomunculus {}

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

fn corner_fwd(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let h0 = 1. - w0;

    let p = 0.6;

    let x0 = p * 0.5 + (1. - p) * w0;
    let h1 = p * h0 + (1. - p) * h;

    Path::<Unit>::move_to(0.5, h0)
        //.bezier2_to((0. + w0, 1. - w0), (0. - w0, 0.75))
        .line_to(x0, h1)
        .close_poly(1. - x0, h1).into()
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

fn corner_tail(i: usize) -> Path::<Unit> {
    let w = UiHomunculusPath::<Unit>::W;
    let h = UiHomunculusPath::<Unit>::H;
    let w0 = i as f32 * w;
    let h0 = w0;

    let p = 0.6;

    let x0 = (1. - p) * 0.5 + p * w0;
    let h1 = (1. - p) * h0 + p * h;

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
                hom.emoji(id, emoji, fun(item.get()));
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
