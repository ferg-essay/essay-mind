use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use log::{log, Level};
use mind_ecs::Tick;

use crate::{
    body::{Body, BodyEat, BodyEatPlugin}, 
    util::{DecayValue, HalfLife, Command}, 
};

pub struct HindEat {
    is_eat: DecayValue,
    is_eat_while_move: bool,
    is_food_zone: bool,

    commands: Command<EatCommand>,
}

impl HindEat {
    pub const HALF_LIFE : HalfLife = HalfLife(0.4);

    fn is_eat(&self) -> bool {
        self.is_eat.value() > 0.25
    } 

    fn is_eat_allowed(&self, body: &Body) -> bool {
        ! self.is_eat_while_move || body.speed() < 0.1
    } 

    #[inline]
    pub fn eat(&self) {
        self.commands.send(EatCommand::Eat);
    }

    #[inline]
    pub fn is_stop(&self) -> bool {
        self.is_eat.value() < 0.1
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
}

impl Default for HindEat {
    fn default() -> Self {
        Self {  
            is_eat: DecayValue::new(HindEat::HALF_LIFE),
            is_eat_while_move: true,
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
    for command in hind_eat.commands() {
        println!("Cmd {:?}", command);

        match command {
            EatCommand::Eat => {
                hind_eat.get_mut().is_eat.set(1.);
            }
            EatCommand::Stop => {
                // hind_eat.get_mut().is_eat.set(0.);
            }
        }
    }

    if ! hind_eat.is_eat() {
        return;
    }

    println!("Try eat");

    if ! body_eat.is_food_zone() {
        // log!(Level::Debug, "eating without sensor");
        return;
    }

    if ! hind_eat.is_eat_allowed(body.get()) {
        log!(Level::Info, "eating while moving");
        println!("Eating while moving");
        return
    }

    body_eat.eat();
    // if world.isbody.is
}

pub struct HindEatPlugin;

impl Plugin for HindEatPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<BodyEatPlugin>(), "HindEatPlugin requires BodyEatPlugin");

        // app.init_resource::<Explore>();
        // app.event::<HindLocomotorEvent>();
        app.init_resource::<HindEat>();

        app.system(Tick, update_hind_eat);
    }
}
