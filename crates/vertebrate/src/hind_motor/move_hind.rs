use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::{body::{Body, BodyPlugin}, core_motive::{Motive, Wake}, util::{Seconds, Ticks, Turn}};

use super::{move_search::LevyWalk, move_startle::Startle};

pub struct HindLocomotion {
    search: Option<LevyWalk>,
    startle: Option<Startle>,

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

    fn update(&mut self, body: &mut Body, wake: &Motive<Wake>) {
        self.clear();

        self.action.update();

        if let Some(startle) = &mut self.startle {
            startle.update(body);

            self.ss_forward = startle.ss_forward().max(self.ss_forward);
            self.ss_left = startle.ss_left().max(self.ss_left);
            self.ss_right = startle.ss_right().max(self.ss_right);

            if self.action.allow_startle() {
                if let Some(action) = startle.next_action() {
                    self.action = action;
                }
            }
        }

        if ! wake.is_active() {
            return;
        }

        if let Some(search) = &mut self.search {
            if ! self.action.is_active() {
                self.action = search.next_action();
            }
        }

        if self.action.is_active() {
            let Action { speed, turn, timeout, elapsed, .. } = self.action;

            // println!("Turn {:?} {:?}", self.action.kind, turn);

            let turn_per_tick = Turn::Unit(turn.to_unit() / timeout.ticks().max(1) as f32);

            body.action(
                speed, 
                turn_per_tick,
                Ticks(timeout.ticks() - elapsed.ticks())
            );

            self.mo_forward = speed;
            let turn = turn.to_unit();

            let turn_value = match self.action.kind {
                ActionKind::Startle => 0.5 + 2. * turn.abs(),
                _ => (2. * turn.abs()).min(0.49),
            };

            if turn < -1.0e-3 {
                self.mo_left = turn_value;
            } else if turn > 1.0e-3 {
                self.mo_right = turn_value;

            }
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
    speed: f32,
    turn: Turn,
    timeout: Ticks,
    elapsed: Ticks,
}

impl Action {
    pub(super) fn new(kind: ActionKind, speed: f32, turn: Turn, time: impl Into<Ticks>) -> Self {
        let timeout = time.into();
        let time = timeout.ticks();

        Self {
            kind,
            speed,
            turn,
            timeout,
            elapsed: Ticks(0),
        }
    }

    pub(super) fn none() -> Self {
        Action::new(ActionKind::None, 0., Turn::Unit(0.), Seconds(1.))
    }

    fn update(&mut self) {
        self.elapsed = Ticks(self.elapsed.ticks() + 1);
    }

    fn is_active(&self) -> bool {
        self.elapsed.ticks() < self.timeout.ticks()
    }

    fn allow_startle(&self) -> bool {
        if ! self.is_active() {
            return true;
        } else {
            match self.kind {
                ActionKind::None => true,
                ActionKind::Roam => true,
                ActionKind::Seek => true,
                ActionKind::Startle => false,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ActionKind {
    None,
    Roam,
    Seek,
    Startle,
}

fn update_hind_move(
    mut hind_move: ResMut<HindLocomotion>,
    mut body: ResMut<Body>,
    wake: Res<Motive<Wake>>,
) {
    hind_move.update(body.get_mut(), wake.get());
}

pub struct HindMovePlugin;

impl Plugin for HindMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "HindLocomotionPlugin requires BodyPlugin");

        let mut hind_move = HindLocomotion::new();
        hind_move.search = Some(LevyWalk::new());
        hind_move.startle = Some(Startle::new());

        app.insert_resource(hind_move);

        app.system(Tick, update_hind_move);
    }
}
