use essay_ecs::{prelude::*, core::Local};
use essay_tensor::Tensor;
use mind_ecs::Tick;

use crate::{body::Body, world::{World, OdorType}};

fn mbon(
    mut mbon: ResMut<Mbon>,
    body: ResMut<Body>,
    world: Res<World>
) {
    let is_food = body.is_sensor_food();

    match body.odor_turn(world.get()) {
        Some((odor, angle)) => {
            mbon.update_odor(Some(odor), is_food);
        },
        None => {
            mbon.update_odor(None, is_food);
        },
    }
}

pub struct Mbon {
    ltd: Option<OdorType>,
    ltp: Option<OdorType>,

    current: Option<OdorType>,
    count: usize,
    is_food: bool,

    inhibit: f32,
}

impl Mbon {
    const THRESHOLD: usize = 10;

    fn new() -> Self {
        Self {
            ltd: None,
            ltp: None,
            current: None,
            count: 0,
            is_food: false,

            inhibit: 0.,
        }
    }

    pub fn is_inhibit(&self) -> bool {
        self.inhibit > 0.5
    }

    fn update_odor(&mut self, odor: Option<OdorType>, is_food: bool) {
        let is_ltp = true;
        let is_learn = false;

        if is_ltp {
            self.update_odor_ltp(odor, is_food, is_learn);
        } else {
            self.update_odor_ltd(odor, is_food, is_learn);
        }
    }

    fn update_odor_ltd(&mut self, odor: Option<OdorType>, is_food: bool, is_learn: bool) {
            match odor {
            Some(odor) => {
                if self.current.is_some() && self.current != Some(odor) {
                    // reset counters if odor changes
                    self.is_food = false;
                    self.count = 0;
                }

                self.current = Some(odor);
                if is_food {
                    self.is_food = true;

                    if self.ltd == self.current {
                        self.ltd = None;
                    }
                }
                self.count += 1;

                if self.ltd == Some(odor) {
                    if is_learn {
                        self.inhibit = 1.;
                    }
                }
            },
            None => {
                if self.current.is_some() {
                    if self.count > Self::THRESHOLD && ! self.is_food {
                        self.ltd = self.current;
                    }

                    self.current = None;
                    self.is_food = false;
                    self.count = 0;
                }

                self.inhibit = 0.;
            }
        }        
    }

    fn update_odor_ltp(&mut self, odor: Option<OdorType>, is_food: bool, is_learn: bool) {
        self.inhibit = 1.;

        if let Some(odor) = odor {
            if is_food {
                self.ltp = Some(odor);
            }

            if self.ltp == Some(odor) {
                if is_learn {
                    self.inhibit = 0.;
                }
            }
        }
    }
}

fn command_muscle_dopamine(
    mut body: ResMut<Body>, 
    world: Res<World>, 
    mbon: Res<Mbon>, 
    mut da: Local<DopaminePair>
) {
    let left_touch = body.is_touch_left();
    let right_touch = body.is_touch_right();

    //let mut left_odor = body.is_food_left(world.get());
    //let mut right_odor = body.is_food_right(world.get());
    let mut left_odor = false;
    let mut right_odor = false;

    // "where" path
    if let Some((_, angle)) = body.odor_turn(world.get()) {
        if ! mbon.is_inhibit() {
            if angle.to_unit() <= 0.5 {
                left_odor = true;
            } else {
                right_odor = true;
            }
        }
    }

    // update habituation
    //left_food = habituate.update_left(left_food);
    //right_food = habituate.update_right(right_food);

    // touch priority over food
    if left_touch || right_touch {
        left_odor = false;
        right_odor = false;
    }

    // touch crosses, food is straight
    let left = right_touch || left_odor;
    let right = left_touch || right_odor;

    // DA as short-term memory of previous direction
    da.left = (da.left - DopaminePair::DECAY).max(0.);
    da.right = (da.right - DopaminePair::DECAY).max(0.);

    if left && right && da.left <= 0. && da.right <= 0. {
        if Tensor::random_uniform([1], ())[0] < 0.5 {
            body.set_muscle_left(1.);
            da.left = if da.left <= 0. { 1. } else { da.left };
        } else {
            body.set_muscle_right(1.);
            da.right = if da.right <= 0. { 1. } else { da.right };
        }
    }

    // inhibition from opposite da
    if left && da.right <= 0. {
        body.set_muscle_left(1.);
        da.left = if da.left <= 0. { 1. } else { da.left };
    }

    // inhibition from opposite da
    if right && da.left <= 0. {
        body.set_muscle_right(1.);
        da.right = if da.right <= 0. { 1. } else { da.right };
    }
}

struct DopaminePair {
    left: f32,
    right: f32,
}

impl DopaminePair {
    pub const DECAY: f32 = 0.025;
}

impl Default for DopaminePair {
    fn default() -> Self {
        Self { left: Default::default(), right: Default::default() }
    }
}

fn food_arrest_update(mut body: ResMut<Body>) {
    if body.is_sensor_food() {
        body.arrest(1.);
    }
}

pub struct SlugControlPlugin;

impl Plugin for SlugControlPlugin {
    fn build(&self, app: &mut App) {
        //app.event::<DirCommand>();
        app.insert_resource(Mbon::new());

        app.system(Tick, mbon);
        //app.system(Update, touch_sense);
        // muscle control with dopamine memory to resolve
        // conflicts
        app.system(Tick, command_muscle_dopamine);

        app.system(Tick, food_arrest_update);
    }
}