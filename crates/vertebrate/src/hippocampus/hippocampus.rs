use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;
use util::random::Rand64;

use crate::{taxis::{TaxisAvoid, TaxisAvoidPlugin}, util::Ticks};

use super::{Sequence128, Sequence128Builder};

pub struct Hippocampus {
    seq_builder: Sequence128Builder,
    fiber_ticks: Ticks,

    seq: Option<Sequence128>,
    ticks: usize,
}

impl Hippocampus {
    fn new(seq_builder: Sequence128Builder) -> Self {
        Self {
            seq_builder,
            fiber_ticks: Ticks(0),
            seq: None,
            ticks: 0,
        }
    }

    pub fn avoid(&mut self) {
        if self.seq.is_none() {
            self.seq = Some(self.seq_builder.next());
        }
    }

    pub fn is_active(&self) -> bool {
        if let Some(seq) = self.seq {
            ! seq.is_zero()
        } else {
            false
        }
    }

    fn update(&mut self) {
        if let Some(seq) = &mut self.seq {
            if self.ticks > 0 {
                self.ticks -= 1;
            } else {
                seq.next();

                if seq.is_zero() {
                    self.seq = None;
                } else {
                    self.ticks = self.fiber_ticks.ticks();
                }
            }
        }
    }
}

fn update_hippocampus(
    mut ehc: ResMut<Hippocampus>,
    mut avoid: ResMut<TaxisAvoid>,
) {
    ehc.update();

    if ehc.is_active() {
        avoid.avoid();
    }
}

pub struct HippocampusPlugin {
    rand: Rand64,
    digits: usize,
    radix: usize,
    seq: usize,
}

impl HippocampusPlugin {
    pub fn new() -> Self {
        let rand = if cfg!(test) {
            Rand64(42)
        } else {
            Rand64::new()
        };

        Self {
            rand,
            digits: 5,
            radix: 4,
            seq: 2,
        }
    }

    pub fn digits(&mut self, digits: usize) -> &mut Self {
        assert!(digits <= 21);

        self.digits = digits;

        self
    }

    pub fn radix(&mut self, radix: usize) -> &mut Self {
        assert!(1 < radix && radix <= 6);

        self.radix = radix;

        self
    }

    pub fn seq(&mut self, seq: usize) -> &mut Self {
        assert!(seq < 6);
        assert!(seq < self.radix);

        self.seq = seq;

        self
    }
}

impl Plugin for HippocampusPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<TaxisAvoidPlugin>(), "MidMove requires HindMove");
        let seq_builder = Sequence128Builder::new(
            self.rand.clone(), self.digits, self.radix, self.seq
        );

        app.insert_resource(Hippocampus::new(seq_builder));

        app.system(Tick, update_hippocampus);
    }
}
