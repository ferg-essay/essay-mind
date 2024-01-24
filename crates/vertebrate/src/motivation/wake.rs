use essay_ecs::{app::{App, Plugin}, core::ResMut};
use mind_ecs::Tick;

use crate::ticks::Seconds;

use super::motive::{Motive, MotiveTrait, Motives};

fn wake_update(mut wake: ResMut<Motive<Wake>>) {
    wake.add(1.);
}

pub struct Wake;
impl MotiveTrait for Wake {}

pub struct WakePlugin;

impl Plugin for WakePlugin {
    fn build(&self, app: &mut App) {
        Motives::insert::<Wake>(app, Seconds(1.));

        app.system(Tick, wake_update);
    }
}
