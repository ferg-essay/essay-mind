use essay_ecs::core::Store;
use essay_ecs::core::store::FromStore;
use essay_ecs::prelude::*;
use mind_ecs::Tick;
use crate::body::touch::Touch;
use crate::body::{Action, Body, BodyAction, BodyPlugin};
use crate::util::{Angle, DirVector};
use util::random::{random_pareto, random, random_normal};


pub struct TaxisPons {
    left60: TaxisTurn,
    right60: TaxisTurn,
    left120: TaxisTurn,
    right120: TaxisTurn,

    action: TaxisAction,
    explore: Explore,

    is_first: bool,
}

impl TaxisPons {
    const _CPG_TIME : f32 = 1.;

    pub fn avoid_left(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.avoid_left(),
            TaxisAction::StrongAvoidLeft => 1.,
            TaxisAction::StrongAvoidRight => 0.,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    pub fn avoid_right(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.avoid_right(),
            TaxisAction::StrongAvoidLeft => 0.,
            TaxisAction::StrongAvoidRight => 1.,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    pub fn avoid_forward(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.avoid_forward(),
            TaxisAction::StrongAvoidLeft => 0.,
            TaxisAction::StrongAvoidRight => 0.,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    pub fn forward_delta(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.forward_delta(),
            TaxisAction::StrongAvoidLeft => 0.5,
            TaxisAction::StrongAvoidRight => 0.5,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    pub fn left_delta(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.left_delta(),
            TaxisAction::StrongAvoidLeft => 1.,
            TaxisAction::StrongAvoidRight => 0.5,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    pub fn right_delta(&self) -> f32 {
        match self.action {
            TaxisAction::None => self.explore.right_delta(),
            TaxisAction::StrongAvoidLeft => 0.5,
            TaxisAction::StrongAvoidRight => 1.,
            TaxisAction::StrongAvoidBoth => 1.,
        }
    }

    fn pre_update(&mut self) {
        self.is_first = true;
        self.action = TaxisAction::None;
        self.explore.pre_update();
    }

    fn event(&mut self, event: &TaxisEvent) {
        //if self.is_first {
        //    self.is_first = false;
        //    self.explore.pre_update();
        //}

        match event {
            // collision/escape - strong avoid events
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

            // gradient taxis
            TaxisEvent::AvoidVector(vector) => {
                self.explore.add_avoid(*vector)
            },
            
            TaxisEvent::ApproachVector(vector) => {
                self.explore.add_approach(*vector)
            },

            // explore/speed modes
            TaxisEvent::Approach => self.explore.approach(),
            TaxisEvent::Avoid => self.explore.avoid(),
            TaxisEvent::AvoidUTurn => self.explore.avoid_turn(),
            TaxisEvent::Normal => self.explore.normal(),
            TaxisEvent::Roam => self.explore.roam(),
            TaxisEvent::Dwell => self.explore.dwell(),

            // display events
            TaxisEvent::ApproachDisplay(_) => {},
            TaxisEvent::AvoidDisplay(_) => {},
        }
    }

    fn update(&mut self, body: &mut Body) {
        match self.action {
            TaxisAction::None => {
                self.explore.update(body);
            },
            TaxisAction::StrongAvoidLeft => {
                body.locomotion_mut().avoid(self.right60.action(1.));
            },
            TaxisAction::StrongAvoidRight => {
                body.locomotion_mut().avoid(self.left60.action(1.));
            },
            TaxisAction::StrongAvoidBoth => {
                if random_normal() < 0. {
                    body.locomotion_mut().avoid(self.left120.action(1.));
                } else {
                    body.locomotion_mut().avoid(self.right120.action(1.));
                }
            },
        }
    }
}

impl FromStore for TaxisPons {
    fn init(_store: &mut Store) -> Self {
        TaxisPons {
            left60: TaxisTurn::new(Angle::Deg(-60.), Angle::Deg(15.)),
            right60: TaxisTurn::new(Angle::Deg(60.), Angle::Deg(15.)),

            left120: TaxisTurn::new(Angle::Deg(-120.), Angle::Deg(60.)),
            right120: TaxisTurn::new(Angle::Deg(120.), Angle::Deg(60.)),

            explore: Explore::new(),
            action: TaxisAction::None,

            is_first: true,
        }
    }
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

        match event {
            TaxisEvent::ApproachDisplay(vector) => {
                body.set_approach_dir(*vector);
            }
            _ => {},
        }
    }

    taxis_pons.update(body.get_mut());
}

#[derive(Clone, Copy, Debug, Event)]
pub enum TaxisEvent {
    // escape/collision
    StrongAvoidLeft,
    StrongAvoidRight,
    StrongAvoidBoth,

    // gradient taxis
    ApproachVector(DirVector),
    AvoidVector(DirVector),

    ApproachDisplay(DirVector),
    AvoidDisplay(DirVector),

    // speed modes
    Approach,
    Avoid,
    AvoidUTurn,
    Normal,
    Roam,
    Dwell,
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
    speed: f32,

    alpha: f32,
    low: f32,
    high: f32,

    turn_mean: f32,
    turn_std: f32,

    approach_left: f32,
    approach_right: f32,
    approach_forward: f32,
    approach_dir: DirVector,

    avoid_left: f32,
    avoid_right: f32,
    avoid_forward: f32,
    avoid_dir: DirVector,

    is_turn: bool,

    action: BodyAction,
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
            speed: 1.,

            low: Self::LOW,
            high: Self::HIGH,
            alpha: Self::ALPHA,

            turn_mean: Self::TURN_MEAN,
            turn_std: Self::TURN_STD,

            approach_left: 0.,
            approach_right: 0.,
            approach_forward: 0.,
            approach_dir: DirVector::zero(),

            avoid_left: 0.,
            avoid_right: 0.,
            avoid_forward: 0.,
            avoid_dir: DirVector::zero(),

            is_turn: false,

            action: BodyAction::Roam,
        }
    }

    fn pre_update(&mut self) {
        self.normal();

        self.approach_left = 0.;
        self.approach_right = 0.;
        self.approach_forward = 0.;
        self.approach_dir = DirVector::zero();

        self.avoid_left = 0.;
        self.avoid_right = 0.;
        self.avoid_forward = 0.;
        self.avoid_dir = DirVector::zero();
    }

    fn normal(&mut self) {
        self.roam();
    }

    fn approach(&mut self) {
        self.dwell();
    }

    fn roam(&mut self) {
        self.speed = 1.;

        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;

        self.action = BodyAction::Roam;
    }

    fn dwell(&mut self) {
        self.speed = 0.5;

        self.low = Self::LOW;
        self.high = Self::HIGH.min(2. * Self::LOW);
        self.alpha = Self::ALPHA;

        self.turn_mean = Self::TURN_MEAN;
        self.turn_std = Self::TURN_STD;

        self.action = BodyAction::Dwell;
    }

    fn add_avoid(&mut self, avoid_dir: DirVector) {
        if avoid_dir.value() > 0.05 {
            let offset = 2. * avoid_dir.sin(); // * avoid_dir.value();

            self.avoid_left = offset.clamp(0., 1.);
            self.avoid_right = (- offset).clamp(0., 1.);
            self.avoid_forward = avoid_dir.cos().clamp(0., 1.);
            self.avoid_dir = avoid_dir;
        }
    }

    fn add_approach(&mut self, approach_dir: DirVector) {
        if approach_dir.value() > 0.01 {
            let offset = 2. * approach_dir.dx(); // * approach_dir.value();

            self.approach_left = (- offset).clamp(0., 1.);
            self.approach_right = offset.clamp(0., 1.);
            self.approach_forward = - approach_dir.dy().clamp(-1., 0.);
            self.approach_dir = approach_dir;

            self.action = BodyAction::Seek;
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

    fn forward_delta(&self) -> f32 {
        0.5 * (self.avoid_forward - self.approach_forward + 1.)
    }

    fn left_delta(&self) -> f32 {
        0.5 * (self.avoid_left - self.approach_left + 1.)
    }

    fn right_delta(&self) -> f32 {
        0.5 * (self.avoid_right - self.approach_right + 1.)
    }

    fn avoid_turn(&mut self) {
        self.low = Self::LOW;
        self.high = 0.5 * Self::HIGH;
        self.alpha = 1.;

        self.turn_mean = 2. * Self::TURN_MEAN;
        self.turn_std = 3. * Self::TURN_STD;
    }

    fn _prefer(&mut self) {
        self.low = Self::LOW;
        self.high = Self::HIGH;
        self.alpha = Self::ALPHA;
    }

    fn _is_turn(&self) -> bool {
        self.is_turn
    }

    fn update(
        &mut self,
        body: &mut Body
    ) {
        body.set_action(self.action);

        if ! body.locomotion().is_idle() {
            return;
        }

        let random = random();
        let mut mean = self.turn_mean;
        let mut std = self.turn_std;

        let avoid_forward = self.avoid_forward + self.approach_forward;
        if avoid_forward > 0. && random_normal().abs() < avoid_forward {
            mean = 2. * Self::TURN_MEAN;
            std = 3. * Self::TURN_STD;
        }

        let angle = mean + (random_normal() * std).clamp(-2. * std, 2. * std);

        // bounded pareto as approximation of Levy walk
        let low = self.low;
        let high = self.high; // 4.;
        let alpha = self.alpha;

        let f = 4.;
        let approach_left = (1. - f * self.approach_right - self.avoid_left).max(1.0e-6);
        let approach_right = (1. - f * self.approach_left - self.avoid_right).max(1.0e-6);
        let p_left = approach_left / (approach_left + approach_right).max(0.01);

        let speed = self.speed;

        // semi-brownian
        if self.is_turn {
            self.is_turn = false;

            let len = random_pareto(low, high, alpha);

            let action = Action::new(len, speed, Angle::Unit(0.));

            body.locomotion_mut().action(action);
        } else if random <= p_left {
            self.is_turn = true;

            let action = Action::new(1., speed, Angle::Deg(- angle));
            body.locomotion_mut().action(action);
            // tectum.toward().action_copy(Turn::Left)
        } else {
            self.is_turn = true;

            let action = Action::new(1., speed, Angle::Deg(angle));
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

pub struct TaxisPonsPlugin;

impl Plugin for TaxisPonsPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyPlugin>(), "MesLocomotorPlugin requires BodyPlugin");
        // assert!(app.contains_plugin::<TectumPlugin>(), "MesLocomotorPlugin requires TectumPlugin");

        // app.init_resource::<Explore>();
        app.event::<TaxisEvent>();
        app.init_resource::<TaxisPons>();

        app.system(Tick, update_taxis_pons);
    }
}
