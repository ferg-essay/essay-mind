use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_brain::{HindMove, HindMovePlugin}, 
    hippocampus::Engram64, 
    mid_brain::MidSeekPlugin, 
    motive::{Motive, MotiveAvoid, MotiveTrait, Motives}, 
    subpallium::{StriatumTimeout, StriatumValue}, 
    taxis::chemotaxis::{Avoid, Seek}, 
    util::{Seconds, Ticks}
};

use super::{MidMove, SeekInput};

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


fn update_seek<I: SeekInput, C: SeekContext, M: MotiveTrait>(
    mut seek: ResMut<MidSeekContext<I, C>>,
    mid_move: Res<MidMove>,
    mut hind_move: ResMut<HindMove>,
    mut avoid: ResMut<MotiveAvoid>,
    input: Res<I>,
    context: Res<C>,
    motive: Res<Motive<M>>,
    tick: Res<AppTick>,
    mut motive_seek: ResMut<Motive<Seek>>,
) {
    // only act if motivated, such as Foraging
    if ! motive.is_active() {
        return;
    }

    if let Some(dir) = input.seek_dir() {
        let context = context.context();

        // seek until timeout
        match seek.update(context, tick.get()) {
            StriatumValue::Active => {
                motive_seek.set_max(1.);
        
                mid_move.seek();

                hind_move.turn(dir.dir().to_turn().to_unit() * 0.5);
            }
            StriatumValue::Avoid => {
                avoid.avoid();
            }
            StriatumValue::None => {
            }
        }
    }
}

pub struct MidSeekContext<I: SeekInput, C: SeekContext> {
    decay: Ticks,

    items: Vec<Item>,

    marker: PhantomData<fn(I, C)>,
}

impl<I: SeekInput, C: SeekContext> MidSeekContext<I, C> {
    fn new() -> Self {
        Self {
            decay: Seconds(120.).into(),
            items: Vec::new(),
            marker: PhantomData::default(),
        }
    }

    fn decay(mut self, value: impl Into<Ticks>) -> Self {
        self.decay = value.into();

        self
    }

    fn update(&mut self, context: Engram64, tick: &AppTick) -> StriatumValue {
        if let Some(item) = self.items.iter_mut().find(|i| i.context == context) {
            if item.retain(tick) {
                let value = item.timeout.active(tick);

                return value;
            }
        }

        self.items.retain_mut(|i| i.retain(tick));

        let mut item = Item {
            context,
            timeout: StriatumTimeout::new().decay(self.decay),
        };

        let value = item.timeout.active(tick);

        self.items.push(item);

        value
    }
}

struct Item {
    context: Engram64,
    timeout: StriatumTimeout,
}

impl Item {
    fn retain(&mut self, tick: &AppTick) -> bool {
        let is_valid = self.timeout.is_valid(tick);

        is_valid
    }
}

pub trait SeekContext : Send + Sync + 'static {
    fn context(&self) -> Engram64;
}

pub struct MidSeekContextPlugin<I: SeekInput, C: SeekContext, M: MotiveTrait> {
    decay: Ticks,
    marker: PhantomData<fn(I, C, M)>,
}

impl<I: SeekInput, C: SeekContext, M: MotiveTrait> MidSeekContextPlugin<I, C, M> {
    pub fn new() -> Self {
        Self {
            decay: Seconds(120.).into(),
            marker: PhantomData::<fn(I, C, M)>::default(),
        }
    }

    pub fn decay(mut self, value: impl Into<Ticks>) -> Self {
        self.decay = value.into();

        self
    }
}

impl<I: SeekInput, C: SeekContext, M: MotiveTrait> Plugin for MidSeekContextPlugin<I, C, M> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "MidSeekContext requires HindMovePlugin");
        assert!(! app.contains_plugin::<MidSeekPlugin<I, M>>(), "MidSeekContext cannot also have MidSeek");
        assert!(app.contains_resource::<I>(), "MidSeekContext requires seek resource {}", type_name::<I>());
        assert!(app.contains_resource::<C>(), "MidSeekContext requires context resource {}", type_name::<C>());
        
        let seek = MidSeekContext::<I, C>::new().decay(self.decay);
        app.insert_resource(seek);

        Motives::insert::<Seek>(app, Seconds(0.2));
        Motives::insert::<Avoid>(app, Seconds(0.2));

        app.system(Tick, update_seek::<I, C, M>);
    }
}
