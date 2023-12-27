use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::{prelude::*, core::Local};
use mind_ecs::Tick;
use crate::action::Turn;
use crate::body::touch::Touch;
use crate::locomotor::mid_explore::MidExplore;
use crate::tectum::{TectumPlugin, TectumLocomotionStn};
use crate::body::{ActionFactory, Body, BodyPlugin, Action};
use crate::util::{Angle, DirVector};
use util::random::{random_pareto, random, random_normal};
// use crate::{body::{Action, Body}, util::{DirVector, Angle}};


pub struct TaxisPons {
    left60: TaxisTurn,
    right60: TaxisTurn,
    u_turn: TaxisTurn,

    action: TaxisAction,
    explore: Explore,
}

impl TaxisPons {
    const _CPG_TIME : f32 = 1.;

    fn pre_update(&mut self) {
        self.action = TaxisAction::None;
        self.explore.pre_update();
    }

    fn event(&mut self, event: &TaxisEvent) {
        match event {
            // strong avoid events
            TaxisEvent::StrongAvoidLeft => {
                self.action = self.action.avoid_left();
            }
            TaxisEvent::StrongAvoidRight => {
                self.action = self.action.avoid_right();
            }
            TaxisEvent::StrongAvoidBoth => {
                self.action = self.action.avoid_left();
                self.action = self.action.avoid_right();
            }

            // explore events
            TaxisEvent::Avoid => self.explore.avoid(),
            TaxisEvent::AvoidUTurn => self.explore.avoid_turn(),
            TaxisEvent::Normal => self.explore.normal(),
            TaxisEvent::AvoidVector(vector) => {
                self.explore.add_avoid(*vector)
            },
        }
    }

    fn update(&mut self, body: &mut Body) {
        match self.action {
            TaxisAction::None => {
                self.explore.update(body);
            },
            TaxisAction::StrongAvoidLeft => {
                body.locomotion_mut().action(self.right60.action(1.));
            },
            TaxisAction::StrongAvoidRight => {
                body.locomotion_mut().action(self.left60.action(1.));
            },
            TaxisAction::StrongAvoidBoth => {
                body.locomotion_mut().action(self.u_turn.action(1.));
            },
        }
    }
}

impl FromStore for TaxisPons {
    fn init(_store: &mut Store) -> Self {
        TaxisPons {
            left60: TaxisTurn::new(Angle::Deg(-60.), Angle::Deg(15.)),
            right60: TaxisTurn::new(Angle::Deg(60.), Angle::Deg(15.)),
            u_turn: TaxisTurn::new(Angle::Deg(180.), Angle::Deg(60.)),

            explore: Explore::new(),
            action: TaxisAction::None,
        }
    }
}

fn update_touch(
    mut locomotor: ResMut<TaxisPons>,
    mut touch: InEvent<Touch>,
) {
    for touch in touch.iter() {
        match touch {
            Touch::CollideLeft => {
            },
            Touch::CollideRight => {
            },
        }
    }
    /*
    if body.is_collide_left() {
        tectum.away().turn(Turn::Right, 1.);
    }

    if body.is_collide_right() {
        tectum.away().turn(Turn::Left, 1.);
    }
    */
}

fn update_taxis_pons(
    mut body: ResMut<Body>, 
    mut touch_events: InEvent<Touch>,
    mut taxis_events: InEvent<TaxisEvent>,
    mut taxis_pons: ResMut<TaxisPons>, 
) {
    taxis_pons.pre_update();

    for touch in touch_events.iter() {
        match touch {
            Touch::CollideLeft => {
                taxis_pons.event(&TaxisEvent::StrongAvoidLeft);
            },
            Touch::CollideRight => {
                taxis_pons.event(&TaxisEvent::StrongAvoidRight);
            },
        }
    }

    for event in taxis_events.iter() {
        taxis_pons.event(event);
    }

    taxis_pons.update(body.get_mut());
}

#[derive(Clone, Copy, Debug, Event)]
pub enum TaxisEvent {
    Avoid,
    AvoidUTurn,
    Normal,
    AvoidVector(DirVector),
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,
}

enum TaxisAction {
    None,
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,
}

impl TaxisAction {
    fn avoid_left(&self) -> Self {
        match self {
            TaxisAction::None => TaxisAction::StrongAvoidLeft,
            TaxisAction::StrongAvoidLeft => TaxisAction::StrongAvoidLeft,
            TaxisAction::StrongAvoidRight => TaxisAction::StrongAvoidBoth,
            TaxisAction::StrongAvoidBoth => TaxisAction::StrongAvoidBoth,
        }
    }

    fn avoid_right(&self) -> Self {
        match self {
            TaxisAction::None => TaxisAction::StrongAvoidRight,
            TaxisAction::StrongAvoidLeft => TaxisAction::StrongAvoidBoth,
            TaxisAction::StrongAvoidRight => TaxisAction::StrongAvoidRight,
            TaxisAction::StrongAvoidBoth => TaxisAction::StrongAvoidBoth,
        }
    }
}
struct Explore {
    alpha: f32,
    low: f32,
    high: f32,

    turn_mean: f32,
    turn_std: f32,

    avoid_left: f32,
    avoid_right: f32,
    avoid_forward: f32,

    is_turn: bool,
}

impl Explore {
    const LOW : f32 = 1.;
    const HIGH : f32 = 5.;
    const ALPHA : f32 = 2.;

    const TURN_MEAN : f32 = 60.;
    const TURN_STD : f32 = 15.;

    const _CPG_TIME : f32 = 1.;

    fn new() -> Self {
        Explore {
            low: Self::LOW,
            high: Self::HIGH,
            alpha: Self::ALPHA,

            turn_mean: Self::TURN_MEAN,
            turn_std: Self::TURN_STD,

            avoid_left: 0.,
            avoid_right: 0.,
            avoid_forward: 0.,

            is_turn: false,
        }
    }

    fn normal(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;
    }

    fn add_avoid(&mut self, avoid_dir: DirVector) {
        if avoid_dir.value() > 0.05 {
            let offset = 2. * avoid_dir.sin(); //  * avoid_dir.value();

            self.avoid_left = offset.clamp(0., 1.);
            self.avoid_right = (- offset).clamp(0., 1.);
            self.avoid_forward = avoid_dir.cos().clamp(0., 1.);
        } else {
            self.avoid_left = 0.;
            self.avoid_right = 0.;
            self.avoid_forward = 0.;
        }
    }

    fn avoid_forward(&self) -> f32 {
        self.avoid_forward
    }

    fn avoid_left(&self) -> f32 {
        self.avoid_left
    }

    fn avoid_right(&self) -> f32 {
        self.avoid_right
    }

    fn avoid(&mut self) {
        // prefer
        //self.low = 2. * Self::HIGH;
        //self.high = 2. * Self::HIGH;
        //self.alpha = 1.;

        //self.turn_mean = 2. * Self::TURN_MEAN;
        //self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = Self::ALPHA;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        // self.low = Self::LOW;
        // self.high = 2. * Self::HIGH;
        // self.alpha = 3.;

        // self.turn_mean = 2. * Self::TURN_MEAN;
        // self.turn_std = Self::TURN_STD;

        self.low = Self::HIGH;
        self.high = 2. * Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = 0.5 * Self::TURN_MEAN;
        self.turn_std = 0.5 * Self::TURN_STD;
    }

    fn avoid_turn(&mut self) {
        self.low = Self::LOW;
        self.high = 0.5 * Self::HIGH;
        self.alpha = 1.;

        self.turn_mean = 2. * Self::TURN_MEAN;
        self.turn_std = 3. * Self::TURN_STD;
    }

    fn prefer(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;
    }

    fn is_turn(&self) -> bool {
        self.is_turn
    }

    fn pre_update(&mut self) {
        self.normal();
    }

    fn update(
        &mut self,
        body: &mut Body
    ) {
        if ! body.locomotion().is_idle() {
            return;
        }

        let random = random();
        let mut mean = self.turn_mean;
        let mut std = self.turn_std;

        if self.avoid_forward > 0. && random_normal().abs() < self.avoid_forward {
            mean = 2. * Self::TURN_MEAN;
            std = 3. * Self::TURN_STD;
        }

        let angle = mean + (random_normal() * std).clamp(-2. * std, 2. * std);

        // bounded pareto as approximation of Levy walk
        let low = self.low;
        let high = self.high; // 4.;
        let alpha = self.alpha;

        let p_left = (1. - self.avoid_left) / (2. - self.avoid_left - self.avoid_right);

        // semi-brownian
        if self.is_turn {
            self.is_turn = false;

            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, 1., Angle::Unit(0.));

            body.locomotion_mut().action(action);
        } else if random <= p_left {
            self.is_turn = true;

            let action = Action::new(1., 1., Angle::Deg(angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Left)
        } else {
            self.is_turn = true;

            let action = Action::new(1., 1., Angle::Deg(-angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Right)
        }
    }
}

struct TaxisTurn {
    mean: Angle,
    std: Angle,
}

impl TaxisTurn {
    fn new(mean: Angle, std: Angle) -> Self {
        Self {
            mean,
            std
        }
    }

    fn action(&self, speed: f32) -> Action {
        let mean = self.mean.to_unit();
        let std = self.std.to_unit();

        let angle = mean + std * random_normal().clamp(-2., 2.);

        Action::new(1., speed, Angle::Unit(angle))
    }
}

pub struct LocomotorPonsPlugin;

impl Plugin for LocomotorPonsPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        // assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        // app.init_resource::<Explore>();
        app.init_resource::<TaxisPons>();

        app.system(Tick, update_taxis_pons);
    }
}
