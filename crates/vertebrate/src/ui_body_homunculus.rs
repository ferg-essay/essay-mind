use essay_ecs::prelude::*;
use essay_plot::{
    prelude::*, 
    artist::{GridColorOpt, ColorMaps, Norms, paths::Unit, PathStyle}
};
use essay_tensor::tf32;
use ui_graphics::{ui_plot::{UiPlot, UiFigurePlugin, UiFigure}, ui_layout::{BoxId, UiLayout, UiLayoutEvent}, UiCanvas};
use crate::{ui_world::{UiWorldPlugin, UiWorld}, body::Body};

#[derive(Component)]
pub struct UiHomunculus {
    id: BoxId,
    pos: Bounds<Canvas>,
    clip: Clip,
    bounds: Bounds<Unit>,

    paths_unit: UiHomunculusPath<Unit>,
    paths_canvas: UiHomunculusPath<Canvas>,
}

impl UiHomunculus {
    pub fn new(id: BoxId) -> Self {
        let paths_unit = UiHomunculusPath::<Unit>::new();
        let affine = Affine2d::eye();
        let paths_canvas = paths_unit.transform(&affine);

        Self {
            id,
            pos: Bounds::zero(),
            clip: Clip::None,
            bounds: Bounds::from([1., 1.]),

            paths_unit,
            paths_canvas,
        }
    }

    pub fn set_pos(&mut self, set_pos: &Bounds<Canvas>) {
        self.pos = set_pos.clone();
        self.clip = Clip::from(&self.pos);

        self.paths_canvas = self.paths_unit.transform(&self.to_canvas());
    }

    pub fn to_canvas(&self) -> Affine2d {
        self.bounds.affine_to(&self.pos)
    }

    pub fn clip(&self) -> &Clip {
        &self.clip
    }
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
        }
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
    mut ui: ResMut<UiCanvas>
) {
    let to_canvas = &ui_homunculus.to_canvas();

    let turn = (body.turn() + 0.5) % 1.;

    let peptides = tf32!([
        [if body.is_collide_left() { 1. } else { 0. }, 
        if body.is_collide_right() { 1. } else { 0. }],
        [0., 0.],
        // [if body.is_food_left(world.deref()) { 1. } else { 0. }, 
        // if body.is_food_right(world.deref()) { 1. } else { 0. }],
        //[ if body.is_sensor_food() { 1. } else { 0. }, body.arrest() ],
        [ body.speed().clamp(0., 1.), 0.],
        [ turn.clamp(0., 0.5) * 2., turn.clamp(0.5, 1.) * 2. - 1. ],
    ]);

    let paths = &ui_homunculus.paths_canvas;

    let mut style = PathStyle::new();
    style.edge_color("black");
    style.face_color(0xf0f0f0);

    ui.draw_path(&paths.outline, &style);

    style.edge_color("red");
    style.face_color("red");

    if body.is_collide_left() { 
        ui.draw_path(&paths.ss_ul, &style);
    }

    if body.is_collide_right() { 
        ui.draw_path(&paths.ss_ur, &style);
    }

    style.edge_color("sky");
    style.face_color("sky");

    let turn_left = turn.clamp(0., 0.5) * 2.;
    let turn_right = turn.clamp(0.5, 1.) * 2. - 1.;

    if turn_left < 1. {
        ui.draw_path(&paths.mo_lr, &style);
    }

    if turn_right > 0. {
        ui.draw_path(&paths.mo_ll, &style);
    }

    //ui_body.action_map.data(peptides.reshape([4, 2]));
}

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
