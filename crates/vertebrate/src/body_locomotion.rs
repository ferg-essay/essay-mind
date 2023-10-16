//use essay_ecs::prelude::*;
use essay_plot::prelude::*;

use essay_tensor::Tensor;

use crate::world::World;

// #[derive(Component)]
pub struct BodyLocomotion {
    pos: Point,

    dir: Angle,
    speed: f32,

    collide_left: bool,
    collide_right: bool,

    theta_ticks: usize,

    action_default: Action,
    action: Option<Action>,
    action_ticks: usize,
}

impl BodyLocomotion {
    pub fn new(pos: Point) -> Self {
        Self {
            theta_ticks: 10,

            pos,
            dir: Angle::Unit(0.),
            speed: 1.,

            collide_left: false,
            collide_right: false,

            action_default: Action::arrest(),
            action: None,
            action_ticks: 0,
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn pos_head(&self) -> Point {

        let Point(x, y) = self.pos;

        let (dy, dx) = self.dir.to_radians().sin_cos();

        // head location
        let head = Point(x + dx * 0.5, y + dy * 0.5);

        head
    }

    pub fn dir(&self) -> Angle {
        self.dir
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn action_default(&mut self, action: Action) {
        self.action_default = action;
    }

    pub fn action(&mut self, action: &ActionFactory) -> bool {
        if self.action.is_none() {
            let action = action.action();
            self.action_ticks = (self.theta_ticks as f32 * action.time).max(1.) as usize;
            self.action = Some(action);
            true
        } else {
            false
        }
    }

    pub fn is_theta(&self) -> bool {
        if let Some(action) = &self.action {
            action.speed > 0. && self.action_ticks == 1
        } else {
            false
        }
    }

    pub fn turn(&self) -> f32 {
        if let Some(action) = &self.action {
            action.turn.to_unit()
        } else {
            0.
        }
    }

    pub fn is_collide_left(&self) -> bool {
        self.collide_left
    }

    pub fn is_collide_right(&self) -> bool {
        self.collide_right
    }

    pub fn _speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    ///
    /// Update the animal's position
    /// 
    pub fn update(&mut self, world: &World) {
        let action = match &self.action {
            Some(action) => action,
            None => &self.action_default,
        };

        self.speed = action.speed;
        let turn = action.turn;

        if self.action_ticks > 0 {
            self.action_ticks -= 1;
        } else {
            self.action = None;
        }

        let speed = self.speed / self.theta_ticks as f32;

        let mut dir = self.dir.to_unit();
        let turn_unit = (turn.to_unit() + 0.5) % 1.0 - 0.5;
        dir += turn_unit / self.theta_ticks as f32;

        // random noise into direction
        if speed > 0. && random() < 0.2 {
            if random() < 0.5 {
                dir += 0.005;
            } else {
                dir -= 0.005;
            }
        }

        self.dir = Angle::Unit((dir + 1.) % 1.);

        let Point(mut x, mut y) = self.pos;

        let (dy, dx) = self.dir.to_radians().sin_cos();

        // head location
        let head = self.pos_head();

        // sensor ahead and to the side
        let sensor_left = (head.0 + dx * 0.1 - dy * 0.1, head.1 + dy * 0.1 + dx * 0.1);
        self.collide_left = world.is_collide(sensor_left);

        let sensor_right = (head.0 + dx * 0.1 + dy * 0.1, head.1 + dy * 0.1 - dx * 0.1);
        self.collide_right = world.is_collide(sensor_right);

        x = (1. - speed) * x + speed * head.0;
        y = (1. - speed) * y + speed * head.1;

        if ! world.is_collide((x, y)) {
            self.pos = Point(x, y);
        } else if ! self.collide_left && ! self.collide_right {
            self.collide_left = true;
            self.collide_right = true;
        }

        self.dir = Angle::Unit((dir + 1.) % 1.);
    }
}

pub struct Action {
    time: f32,
    speed: f32,
    turn: Angle,
}

impl Action {
    pub fn new(time: f32, speed: f32, turn: Angle) -> Self {
        Self {
            time,
            speed,
            turn,
        }
    }

    pub fn arrest() -> Self {
        Self::new(1., 0., Angle::Unit(0.))
    }

    pub fn forward() -> Self {
        Self::new(1., 1., Angle::Unit(0.))
    }
}

pub struct ActionFactory {
    speed_mean: f32,
    speed_std: f32,

    turn_mean: Angle,
    turn_std: Angle,
}

impl ActionFactory {
    pub fn new(speed: f32, turn: Angle) -> Self {
        Self {
            speed_mean: speed,
            speed_std: 0.,

            turn_mean: turn,
            turn_std: Angle::Unit(0.),
        }
    }

    pub fn action(&self) -> Action {
        Action::new(1., self.speed_mean, self.turn_mean)
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}
