use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}, prelude::Event};
use mind_ecs::Tick;

use crate::{core_motive::{Dwell, Motive, Wake}, hind_motor::{HindEat, HindLevyMove, HindLevyPlugin}, util::Command};

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

    fn pre_update(&mut self) {
    }

    fn clear(
        &mut self,
    ) {
        self.commands();
    }

    fn update(
        &mut self,
        dwell: &Motive<Dwell>,
        hind_move: &HindLevyMove,
        hind_eat: &HindEat,
    ) {
        for event in self.commands() {
            match event {
                MidMotorEvent::Eat => {
                    self.on_eat(hind_move, hind_eat);
                },
                MidMotorEvent::Explore => {
                    if dwell.is_active() {
                        self.on_dwell(hind_move, hind_eat);
                    } else {
                        self.on_roam(hind_move, hind_eat);
                    }
                }
            }
        }
    }

    fn on_roam(
        &mut self, 
        hind_motor: &HindLevyMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if hind_eat.is_stop() {
            hind_motor.roam();
        }
    }

    fn on_dwell(
        &mut self, 
        hind_motor: &HindLevyMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before dwell
        if hind_eat.is_stop() {
            hind_motor.dwell();
        }
    }

    fn on_eat(
        &mut self, 
        hind_motor: &HindLevyMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for movement to stop before eat
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
    hind_move: Res<HindLevyMove>, 
    wake: Res<Motive<Wake>>,
    dwell: Res<Motive<Dwell>>,
) {
    mid_motor.pre_update();

    if wake.is_active() {
        mid_motor.update(dwell.get(), hind_move.get(), hind_eat.get());
    } else {
        mid_motor.clear();
    }
}

pub struct MidMotorPlugin;

impl Plugin for MidMotorPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindLevyPlugin>(), "MidMotor requires HindLevy");

        app.init_resource::<MidMotor>();
        app.event::<MidMotorEvent>();

        app.system(Tick, update_mid_motor);
    }
}
