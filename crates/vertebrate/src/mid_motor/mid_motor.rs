use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}, prelude::Event};
use mind_ecs::Tick;

use crate::{core_motive::{Dwell, Motive}, hind_motor::{HindEat, HindMove}, util::Command};

pub struct MidMotor {
    commands: Command<MidMotorEvent>,
}

impl MidMotor {
    #[inline]
    pub fn eat(&self) {
        self.commands.send(MidMotorEvent::Eat);
    }

    #[inline]
    pub fn explore(&self) {
        self.commands.send(MidMotorEvent::Explore);
    }

    #[inline]
    fn commands(&mut self) -> Vec<MidMotorEvent> {
        self.commands.drain()
    }

    fn on_roam(
        &mut self, 
        hind_motor: &HindMove,
        hind_eat: &HindEat,
    ) {
        if hind_eat.is_stop() {
            hind_motor.roam();
        } else {
            hind_eat.stop();
        }
    }

    fn on_dwell(
        &mut self, 
        hind_motor: &HindMove,
        hind_eat: &HindEat,
    ) {
        if hind_eat.is_stop() {
            hind_motor.dwell();
        } else {
            hind_eat.stop();
        }
    }

    fn on_eat(
        &mut self, 
        hind_motor: &HindMove,
        hind_eat: &HindEat,
    ) {
        if hind_motor.is_stop() {
            hind_eat.eat();
        } else {
            hind_motor.stop();
        }
    }
}

impl Default for MidMotor {
    fn default() -> Self {
        Self { 
            commands: Command::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Event)]
enum MidMotorEvent {
    Eat,
    Explore,
}

fn update_mid_motor(
    mut mid_motor: ResMut<MidMotor>,
    hind_eat: Res<HindEat>, 
    hind_move: Res<HindMove>, 
    dwell: Res<Motive<Dwell>>,
) {
    for event in mid_motor.commands() {
        match event {
            MidMotorEvent::Eat => {
                mid_motor.on_eat(hind_move.get(), hind_eat.get());
            },
            MidMotorEvent::Explore => {
                if dwell.is_active() {
                    mid_motor.on_dwell(hind_move.get(), hind_eat.get());
                } else {
                    mid_motor.on_roam(hind_move.get(), hind_eat.get());
                }
            },
        }
    }
}

pub struct MidMotorPlugin;

impl Plugin for MidMotorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MidMotor>();
        app.event::<MidMotorEvent>();

        app.system(Tick, update_mid_motor);
    }
}
