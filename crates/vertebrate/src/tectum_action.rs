use essay_ecs::prelude::{App, Plugin, ResMut};
use mind_ecs::Tick;

use crate::striatum_action::{StriatumAction, ActionId};

pub struct TectumTurn {
    striatum: StriatumAction,
    left: ActionId,
    right: ActionId,

    action: Option<Turn>,
}

impl TectumTurn {
    fn new(_name: &str) -> Self {
        let mut striatum = StriatumAction::new();
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
            Turn::Left => { self.striatum.sense(self.left, value); }
            Turn::Right => { self.striatum.sense(self.right, value); }
        }
    }

    pub fn action(&self) -> Option<Turn> {
        self.action.clone()
    }

    pub fn action_efference(&mut self, turn: Turn) {
        let value = 1.;

        match turn {
            Turn::Left => { self.striatum.action_efference(self.left, value); }
            Turn::Right => { self.striatum.action_efference(self.right, value); }
        }
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

pub struct TectumLocomotion {
    approach: TectumTurn,
    repel: TectumTurn,
}

impl TectumLocomotion {
    pub fn new() -> Self {
        Self {
            approach: TectumTurn::new("approach"),
            repel: TectumTurn::new("repel"),
        }
    }

    pub fn approach(&mut self) -> &mut TectumTurn {
        &mut self.approach
    }

    pub fn repel(&mut self) -> &mut TectumTurn {
        &mut self.repel
    }

    pub fn update(&mut self) {
        self.approach.update();
        self.repel.update();
    }
}

impl Default for TectumLocomotion {
    fn default() -> Self {
        Self::new()
    }
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
