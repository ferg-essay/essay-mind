use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}, 
    prelude::Event
};
use mind_ecs::Tick;

use crate::{
    hind_eat::HindEat, hind_move::{HindMove, HindMovePlugin}, motive::{Dwell, Motive, Wake}, util::Command
};

pub struct MidMove {
    commands: Command<MidMoveEvent>,
}

impl MidMove {
    #[inline]
    pub fn eat(&self) {
        self.commands.send(MidMoveEvent::Eat);
    }

    #[inline]
    pub fn dwell(&self) {
        self.commands.send(MidMoveEvent::Dwell);
    }

    #[inline]
    pub fn roam(&self) {
        self.commands.send(MidMoveEvent::Roam);
    }

    #[inline]
    pub fn seek(&self) {
        self.commands.send(MidMoveEvent::Seek);
    }

    #[inline]
    fn commands(&mut self) -> Vec<MidMoveEvent> {
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
        _dwell: &Motive<Dwell>,
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        for event in self.commands() {
            match event {
                MidMoveEvent::Eat => {
                    self.on_eat(hind_move, hind_eat);
                },
                MidMoveEvent::Roam => {
                    self.on_roam(hind_move, hind_eat);
                }
                MidMoveEvent::Seek => {
                    self.on_seek(hind_move, hind_eat);
                }
                MidMoveEvent::Dwell => {
                    self.on_dwell(hind_move, hind_eat);
                }
            }
        }
    }

    fn on_roam(
        &mut self, 
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if hind_eat.is_stop() {
            hind_move.roam();
        }
    }

    fn on_seek(
        &mut self, 
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if hind_eat.is_stop() {
            hind_move.seek();
        }
    }

    fn on_dwell(
        &mut self, 
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before dwell
        if hind_eat.is_stop() {
            hind_move.roam(); // dwell
        }
    }

    fn on_eat(
        &mut self, 
        hind_move: &HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for movement to stop before eat
        if hind_move.is_stop() {
            hind_eat.eat();
        } else {
            hind_move.stop();
        }
    }
}

impl Default for MidMove {
    fn default() -> Self {
        Self { 
            commands: Command::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Event)]
enum MidMoveEvent {
    Eat,
    Roam,
    Dwell,
    Seek,
}

fn update_mid_motor(
    mut mid_motor: ResMut<MidMove>,
    hind_eat: Res<HindEat>, 
    mut hind_move: ResMut<HindMove>, 
    wake: Res<Motive<Wake>>,
    dwell: Res<Motive<Dwell>>,
) {
    mid_motor.pre_update();

    if wake.is_active() {
        mid_motor.update(dwell.get(), hind_move.get_mut(), hind_eat.get());
    } else {
        mid_motor.clear();
    }
}

pub struct MidMovePlugin;

impl Plugin for MidMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "MidMove requires HindMove");

        app.init_resource::<MidMove>();
        app.event::<MidMoveEvent>();

        app.system(Tick, update_mid_motor);
    }
}
