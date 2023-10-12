use essay_ecs::prelude::{App, Plugin, ResMut};
use mind_ecs::Tick;

use crate::striatum_sense::{Striatum, ActionId};

pub struct TectumLocomotion {
    toward: TectumTurn,
    away: TectumTurn,
}

impl TectumLocomotion {
    pub fn new() -> Self {
        Self {
            toward: TectumTurn::new("toward"),
            away: TectumTurn::new("away"),
        }
    }

    pub fn toward(&mut self) -> &mut TectumTurn {
        &mut self.toward
    }

    pub fn away(&mut self) -> &mut TectumTurn {
        &mut self.away
    }

    pub fn update(&mut self) {
        self.toward.update();
        self.away.update();
    }
}

impl Default for TectumLocomotion {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TectumTurn {
    striatum: Striatum,
    left: ActionId,
    right: ActionId,

    action: Option<Turn>,
}

impl TectumTurn {
    fn new(_name: &str) -> Self {
        let mut striatum = Striatum::new();
        let left = striatum.add_action("turn-left");
        let right = striatum.add_action("turn-right");

        Self {
            striatum,
            left,
            right,
            action: None,
        }
    }

    pub fn turn(&mut self, turn: Turn, value: f32) {
        match turn {
            Turn::Left => { self.striatum.set_value(self.left, value); }
            Turn::Right => { self.striatum.set_value(self.right, value); }
        }
    }

    pub fn action(&self) -> Option<Turn> {
        self.action.clone()
    }

    fn update(&mut self) -> Option<Turn> {
        self.action = None;

        if let Some(id) = self.striatum.update() {
            if id == self.left {
                self.action = Some(Turn::Left);
            } else if id == self.right {
                self.action = Some(Turn::Right);
            } else {
                self.action = None;
            }
        }

        self.action()
    }
}

#[derive(Clone, Debug)]
pub enum Turn {
    Left,
    Right,
}

fn update_tectum(mut tectum: ResMut<TectumLocomotion>) {
    tectum.update();
}


pub struct TectumPlugin;

impl Plugin for TectumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TectumLocomotion>();

        app.system(Tick, update_tectum);
    }
}
