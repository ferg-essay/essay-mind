use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use log::{log, Level};
use mind_ecs::Tick;

use crate::{
    body::{Body, BodyEat, BodyEatPlugin},
    util::{DecayValue, HalfLife, Command} 
};

pub struct HindEat {
    is_eat: DecayValue,
    is_eat_enable: DecayValue,
    allow_eat_while_move: bool,
    is_food_zone: bool,

    commands: Command<EatCommand>,
}

impl HindEat {
    pub const HALF_LIFE : HalfLife = HalfLife(2.);

    pub fn is_eat(&self) -> bool {
        self.is_eat.value() > 0.25
    } 

    fn is_eat_enable(&self) -> bool {
        self.is_eat_enable.value() > 0.25
    } 

    fn is_eat_allowed(&self, body: &Body) -> bool {
        ! self.allow_eat_while_move || body.speed() < 0.1
    } 

    #[inline]
    pub fn eat(&self) {
        self.commands.send(EatCommand::Eat);
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.is_eat_enable.value() < 0.1
    } 

    #[inline]
    pub fn stop(&self) {
        self.commands.send(EatCommand::Stop);
    }

    #[inline]
    pub fn is_food_zone(&self) -> bool {
        self.is_food_zone
    } 

    #[inline]
    fn commands(&mut self) -> Vec<EatCommand> {
        self.commands.drain()
    }

    fn pre_update(&mut self) {
        self.is_eat.update();
        self.is_eat_enable.update();
    }
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            is_eat_enable: DecayValue::new(HindEat::HALF_LIFE),
            is_eat: DecayValue::new(HindEat::HALF_LIFE),
            allow_eat_while_move: true,
            is_food_zone: false,
            commands: Command::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EatCommand {
    Eat,
    Stop,
}

fn update_hind_eat(
    mut hind_eat: ResMut<HindEat>,
    mut body_eat: ResMut<BodyEat>,
    body: Res<Body>,
) {
    hind_eat.pre_update();

    for command in hind_eat.commands() {
        match command {
            EatCommand::Eat => {
                hind_eat.get_mut().is_eat_enable.set(1.);
            }
            EatCommand::Stop => {
                // hind_eat.get_mut().is_eat.set(0.);
            }
        }
    }

    if ! hind_eat.is_eat_enable() {
        return;
    }

    if ! body_eat.is_food_zone() {
        // log!(Level::Debug, "eating without sensor");
        return;
    }

    if ! hind_eat.is_eat_allowed(body.get()) {
        log!(Level::Info, "eating while moving");
        return
    }

    body_eat.eat();
    // hind_eat.get_mut().is_eat.set_max(1.);
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        app.init_resource::<HindEat>();

        app.system(Tick, update_hind_eat);
    }
}
