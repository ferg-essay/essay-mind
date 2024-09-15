use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}
};
use log::error;
use mind_ecs::Tick;

use crate::{
    body::{Body, BodyEat, BodyEatPlugin},
    util::{Seconds, Ticks, TimeoutValue} 
};

//
// HindEat corresponds to Phox2b correlates of adult tunicate Ciona brain
// HindEat includes R.nts and R.my but not R.pb
//

pub struct HindEat {
    is_eat_request: TimeoutValue<bool>,
    is_stop_request: TimeoutValue<bool>,

    is_eating: TimeoutValue<bool>,

    // Mouse gaping is a reflexive orofacial expression to expel food in
    // the mouth, functionally like spitting
    is_gaping: TimeoutValue<bool>,

    // Some animals like mice can't vomit
    is_vomiting: TimeoutValue<bool>,

    // Configuration

    // animals that can eat while moving, such as worms or swimming
    // filter feeders like manta rays
    allow_eat_while_move: bool,
}

impl HindEat {
    #[inline]
    pub fn is_eating(&self) -> bool {
        self.is_eating.value_or(false)
    } 

    #[inline]
    pub fn is_gaping(&self) -> bool {
        self.is_gaping.value_or(false)
    } 

    #[inline]
    pub fn is_vomiting(&self) -> bool {
        self.is_vomiting.value_or(false)
    } 

    #[inline]
    pub fn eat(&mut self) {
        self.is_eat_request.set(true);
    }

    #[inline]
    fn is_eat_request(&self) -> bool {
        self.is_eat_request.value_or(false)
    } 

    #[inline]
    pub fn stop(&mut self) {
        self.is_stop_request.set(true);
    }

    fn is_stop_request(&self) -> bool {
        self.is_stop_request.value_or(false)
    } 

    fn is_eat_allowed(&self, body: &Body) -> bool {
        ! self.allow_eat_while_move || body.speed() < 0.1
    } 

    fn pre_update(&mut self) {
        self.is_eating.update();
        self.is_gaping.update();
        self.is_vomiting.update();

        self.is_eat_request.update();
        self.is_stop_request.update();
    }
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            is_eat_request: TimeoutValue::new(Ticks(3)),
            is_stop_request: TimeoutValue::new(Seconds(1.)),
            is_eating: TimeoutValue::new(Seconds(2.)),
            is_gaping: TimeoutValue::new(Seconds(10.)),
            is_vomiting: TimeoutValue::new(Seconds(60.)),
            allow_eat_while_move: true,
        }
    }
}

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
) {
    hind_eat.pre_update();

    if hind_eat.is_stop_request() {
        hind_eat.is_eating.set(false);
    } else if hind_eat.is_eat_request() {
        if hind_eat.is_eat_allowed(body.get()) {
            hind_eat.is_eating.set(true);
        } else {
            error!("eating while moving");
        }
    }


    if body_eat.sickness() > 0. {
        // rodent lack vomiting
        hind_eat.is_vomiting.set(true);
    } else if body_eat.bitter() > 0. {
        // rodent gaping is in R.nts [cite]
        hind_eat.is_gaping.set(true);
    } else if hind_eat.is_eating() {
        body_eat.eat();
    }
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        app.init_resource::<HindEat>();

        app.system(Tick, update_hind_eat);
    }
}
