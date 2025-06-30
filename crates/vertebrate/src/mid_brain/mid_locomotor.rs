use essay_ecs::{
    app::{App, Plugin}, 
    core::{Res, ResMut}, 
    prelude::Event
};
use mind_ecs::Tick;

use crate::{
    hind_brain::{HindEat, HindMove, HindMovePlugin, ArtrR2, Serotonin}, 
    hypothalamus::{Dwell, Motive, Wake}, 
    util::Command
};

fn update_mid_motor(
    mut mid_motor: ResMut<MidLocomotor>,
    mut hind_eat: ResMut<HindEat>, 
    mut serotonin_eat: ResMut<Serotonin<HindEat>>, 
    mut serotonin_search: ResMut<Serotonin<ArtrR2>>, 
    mut hind_move: ResMut<HindMove>, 
    wake: Res<Motive<Wake>>,
    dwell: Res<Motive<Dwell>>,
) {
    mid_motor.pre_update();

    if wake.is_active() {
        mid_motor.update(
            dwell.get(), 
            hind_move.get_mut(), 
            hind_eat.get_mut(),
            serotonin_eat.get_mut(),
            serotonin_search.get_mut(),
        );
    } else {
        mid_motor.clear();
    }
}

pub struct MidLocomotor {
    commands: Command<MidLocomotorEvent>,
}

impl MidLocomotor {
    #[inline]
    pub fn eat(&self) {
        self.commands.send(MidLocomotorEvent::Eat);
    }

    #[inline]
    pub fn roam(&self) {
        self.commands.send(MidLocomotorEvent::Roam);
    }

    #[inline]
    pub fn avoid(&self) {
        self.commands.send(MidLocomotorEvent::Avoid);
    }

    #[inline]
    pub fn seek(&self) {
        self.commands.send(MidLocomotorEvent::Seek);
    }

    #[inline]
    fn commands(&mut self) -> Vec<MidLocomotorEvent> {
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
        hind_eat: &mut HindEat,
        serotonin_eat: &mut Serotonin<HindEat>,
        serotonin_search: &mut Serotonin<ArtrR2>,
    ) {
        for event in self.commands() {
            match event {
                MidLocomotorEvent::Eat => {
                    self.on_eat(hind_move, serotonin_eat);
                },
                MidLocomotorEvent::Roam => {
                    self.on_roam(hind_eat, hind_move, serotonin_search);
                }
                MidLocomotorEvent::Avoid => {
                    self.on_avoid(hind_move, hind_eat);
                }
                MidLocomotorEvent::Seek => {
                    self.on_seek(hind_move, hind_eat);
                }
            }
        }
    }

    fn on_roam(
        &mut self, 
        hind_eat: &HindEat,
        hind_move: &mut HindMove,
        serotonin_search: &mut Serotonin<ArtrR2>,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if ! hind_eat.is_eating() {
            hind_move.roam();
            serotonin_search.excite(1.);
        }
    }

    fn on_avoid(
        &mut self, 
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if ! hind_eat.is_eating() {
            hind_move.avoid();
        }
    }

    fn on_seek(
        &mut self, 
        hind_move: &mut HindMove,
        hind_eat: &HindEat,
    ) {
        // H.stn managed transition waits for eat to stop before roam
        if ! hind_eat.is_eating() {
            hind_move.seek();
        }
    }

    fn on_eat(
        &mut self, 
        hind_move: &mut HindMove,
        serotonin_eat: &mut Serotonin<HindEat>,
    ) {
        // H.stn managed transition waits for movement to stop before eat
        if hind_move.is_stop() {
            serotonin_eat.excite(1.);
        } else {
            hind_move.halt();
        }
    }
}

impl Default for MidLocomotor {
    fn default() -> Self {
        Self { 
            commands: Command::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Event)]
enum MidLocomotorEvent {
    Eat,
    Roam,
    Avoid,
    Seek,
}

pub struct MidMovePlugin;

impl Plugin for MidMovePlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "MidMove requires HindMove");

        app.init_resource::<MidLocomotor>();
        app.event::<MidLocomotorEvent>();

        app.system(Tick, update_mid_motor);
    }
}
