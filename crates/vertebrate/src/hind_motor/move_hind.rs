use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::{body::{Body, BodyPlugin}, util::{Seconds, Ticks, Turn}};

use super::{move_search::LevyWalk, move_startle::Mauthner};

pub struct HindLocomotion {
    search: Option<LevyWalk>,
    startle: Option<Mauthner>,

    action: Action,
    
    ss_forward: f32,
    ss_left: f32,
    ss_right: f32,

    // output for UI
    is_freeze: bool,

    mo_forward: f32,
    mo_left: f32,
    mo_right: f32,
}

impl HindLocomotion {
    // transition between slow twitch and fast twitch
    pub const MID : f32 = 0.5;
    pub const MCELL : f32 = 1.0;

    fn new() -> Self {
        Self {
            search: None,
            startle: None,

            action: Action::none(),

            ss_forward: 0.0,
            ss_left: 0.0,
            ss_right: 0.0,

            is_freeze: false,

            mo_forward: 0.0,
            mo_left: 0.0,
            mo_right: 0.0,
        }
    }

    fn update(&mut self, body: &mut Body) {
        self.clear();

        self.action.update();

        if let Some(startle) = &mut self.startle {
            startle.update(body);

            self.ss_forward = startle.ss_forward().max(self.ss_forward);
            self.ss_left = startle.ss_left().max(self.ss_left);
            self.ss_right = startle.ss_right().max(self.ss_right);
        }

        if let Some(search) = &mut self.search {
            if ! self.action.is_active() {
                self.action = search.next_action();
            }
        }

        if self.action.is_active() {
            let Action { speed, turn, time, factor, .. } = self.action;

            body.action(speed, turn, factor, time);
        }
    }

    fn clear(&mut self) {
        self.ss_forward = 0.;
        self.ss_left = 0.;
        self.ss_right = 0.;

        self.is_freeze = false;

        self.mo_forward = 0.;
        self.mo_left = 0.;
        self.mo_right = 0.;
    }

    //
    // UI output
    //

    #[inline]
    pub fn ss_forward(&self) -> f32 {
        self.ss_forward
    }

    #[inline]
    pub fn ss_left(&self) -> f32 {
        self.ss_left
    }

    #[inline]
    pub fn ss_right(&self) -> f32 {
        self.ss_right
    }

    #[inline]
    pub fn is_freeze(&self) -> bool {
        self.is_freeze
    }

    #[inline]
    pub fn mo_forward(&self) -> f32 {
        self.mo_forward
    }

    #[inline]
    pub fn mo_left(&self) -> f32 {
        self.mo_left
    }

    #[inline]
    pub fn mo_right(&self) -> f32 {
        self.mo_right
    }
}

#[derive(Clone, Debug)]
pub(super) struct Action {
    kind: ActionKind,
    factor: f32,
    time: f32,
    speed: f32,
    turn: Turn,
}

impl Action {
    pub(super) fn new(kind: ActionKind, speed: f32, turn: Turn, time: impl Into<Seconds>, factor: f32) -> Self {
        Self {
            kind,
            time: time.into().0,
            factor,
            speed,
            turn,
        }
    }

    pub(super) fn none() -> Self {
        Action::new(ActionKind::None, 0.,Turn::Unit(0.), 1., 0.)
    }

    fn update(&mut self) {
        self.time -= 1. / Ticks::TICKS_PER_SECOND as f32;
    }

    fn is_active(&self) -> bool {
        self.time >= 1.0e-6
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ActionKind {
    None,
    Roam,
    Seek,
}

fn update_hind_move(
    mut hind_move: ResMut<HindLocomotion>,
    mut body: ResMut<Body>
) {
    hind_move.update(body.get_mut());
}

pub struct HindMovePlugin;

impl Plugin for HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotionPlugin requires BodyPlugin");

        let mut hind_move = HindLocomotion::new();
        hind_move.search = Some(LevyWalk::new());
        hind_move.startle = Some(Mauthner::new());

        app.insert_resource(hind_move);

        app.system(Tick, update_hind_move);
    }
}
