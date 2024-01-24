use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::Tick;

use crate::ticks::Seconds;

use super::{motive::{Motive, MotiveTrait, Motives}, Wake};

fn roam_update(
    mut roam: ResMut<Motive<Roam>>,
    wake: Res<Motive<Wake>>,
) {
    roam.set_max(wake.value());
}

pub struct Roam;
impl MotiveTrait for Roam {}

pub struct Dwell;
impl MotiveTrait for Dwell {}

pub struct ExplorePlugin;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        Motives::insert::<Roam>(app, Seconds(5.));

        app.system(Tick, roam_update);
    }
}
