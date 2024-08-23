use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_move::{HindMove, HindMovePlugin},
    motive::{Motive, MotiveTrait, Motives}, 
    striatum::{StriatumTimeout, StriatumValue}, 
    taxis::{chemotaxis::{Avoid, Seek}, TaxisAvoid}, 
    util::{EgoVector, Seconds}
};

use super::MidMove;

// 
// midbrain tegmental seek: fish V.pt - posterior tuberculum to MLR
//
// [Barrior et al 2020] fish V.pt CSF contacting. V.pt turns and struggles but not 
//   forward, correlated with Hi, Hc
// [Beausejour et al 2020] lamprey V.pt to Ob.m feedback (DA)
// [Horstick et al 2020] fish V.pt has a Hb projection for turn bias
// [Imamura et al 2020] fish O.mc to V.pt
// [Kermen et al 2020] fish Oe (Nose) to V.pt skipping Ob, unclear if Ob to V.pt
// [Suryanarayana et al 2021] lamprey Ob.m to V.pt locomotor
//


pub struct MidSeek<I: SeekInput> {
    timeout: StriatumTimeout,

    marker: PhantomData<I>,
}

impl<I: SeekInput> MidSeek<I> {
    fn new() -> Self {
        Self {
            timeout: StriatumTimeout::new(),
            marker: PhantomData::default(),
        }
    }

    fn update(&mut self, tick: &AppTick) -> StriatumValue {
        self.timeout.active(tick)
    }
}

pub trait SeekInput : Send + Sync + 'static {
    fn seek_dir(&self) -> Option<EgoVector>;
}

pub struct MidSeekPlugin<I: SeekInput, M: MotiveTrait> {
    marker: PhantomData<(I, M)>,
}

impl<I: SeekInput, M: MotiveTrait> MidSeekPlugin<I, M> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData::<(I, M)>::default(),
        }
    }
}

fn update_seek<I: SeekInput, M: MotiveTrait>(
    mut seek: ResMut<MidSeek<I>>,
    mid_move: Res<MidMove>,
    mut hind_move: ResMut<HindMove>,
    mut avoid: ResMut<TaxisAvoid>,
    input: Res<I>,
    motive: Res<Motive<M>>,
    tick: Res<AppTick>,
    mut motive_seek: ResMut<Motive<Seek>>,
    mut _motive_avoid: ResMut<Motive<Avoid>>,
) {
    // only act if motivated, such as Foraging
    if ! motive.is_active() {
        return;
    }

    if let Some(dir) = input.seek_dir() {
        // seek until timeout
        match seek.update(tick.get()) {
            StriatumValue::Active => {
                // println!("Active");
                motive_seek.set_max(1.);
        
                mid_move.seek();

                hind_move.turn(dir.dir().to_turn());
            }
            StriatumValue::Avoid => {
                avoid.avoid();
            }
            StriatumValue::None => {
                // println!("None");
            }
        }
    }
}

impl<I: SeekInput, M: MotiveTrait> Plugin for MidSeekPlugin<I, M> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "MidSeek requires HindMovePlugin");
        assert!(app.contains_resource::<I>(), "MidSeek requires resource {}", type_name::<I>());
        
        let seek = MidSeek::<I>::new();
        app.insert_resource(seek);

        Motives::insert::<Seek>(app, Seconds(0.2));
        Motives::insert::<Avoid>(app, Seconds(0.2));

        app.system(Tick, update_seek::<I, M>);
    }
}
