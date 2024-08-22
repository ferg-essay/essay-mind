// 
// tegmental motor: posterior tuberculum and Vta
//

use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    hind_move::{HindMove, HindMovePlugin},
    motive::{Motive, MotiveTrait, Motives}, 
    striatum::{Gate, Striatum2, StriatumGate}, 
    taxis::chemotaxis::{Avoid, Seek}, 
    util::{DecayValue, EgoVector, Seconds, Turn}
};

pub struct TegSeek<I: SeekInput> {
    ltd_buildup: DecayValue,
    ltd_decay: DecayValue,

    marker: PhantomData<I>,
}

impl<I: SeekInput> TegSeek<I> {
    const BUILDUP : f32 = 25.;

    fn new() -> Self {
        Self {
            ltd_buildup: DecayValue::new(Seconds(Self::BUILDUP)),
            ltd_decay: DecayValue::new(Seconds(1.5 * Self::BUILDUP)),
            marker: PhantomData::default(),
        }
    }

    fn update(&mut self, tick: &AppTick) -> bool {
        self.ltd_buildup.update_ticks(tick.ticks());
        self.ltd_decay.update_ticks(tick.ticks());
        
        // avoid timeout (adenosine in striatum) with hysteresis
        let is_seek = if self.ltd_decay.value() < 0.2 {
            true
        } else if self.ltd_decay.value() > 0.9 {
            false
        } else {
            // hysteresis
            self.ltd_buildup.value() > 0.05
        };

        if is_seek {
            self.ltd_buildup.add(1.);
            self.ltd_decay.set_max(self.ltd_buildup.value());
        } else {
            self.ltd_buildup.set(0.);
        }

        is_seek
    }
}

pub trait SeekInput : Send + Sync + 'static {
    fn seek_dir(&self) -> Option<EgoVector>;
}

pub struct MidSeekPlugin<I: SeekInput, M: MotiveTrait> {
    _striatum: Striatum2,
    marker: PhantomData<(I, M)>,
}

impl<I: SeekInput, M: MotiveTrait> MidSeekPlugin<I, M> {
    pub fn new() -> Self {
        Self {
            _striatum: Striatum2::default(),
            marker: PhantomData::<(I, M)>::default(),
        }
    }
}

fn update_seek<I: SeekInput, M: MotiveTrait>(
    mut seek: ResMut<TegSeek<I>>,
    mut hind_move: ResMut<HindMove>,
    input: Res<I>,
    motive: Res<Motive<M>>,
    tick: Res<AppTick>,
    mut motive_seek: ResMut<Motive<Seek>>,
    mut motive_avoid: ResMut<Motive<Avoid>>,
) {
    if ! motive.is_active() {
        return;
    }

    if let Some(dir) = input.seek_dir() {
        // seek until timeout
        if seek.update(tick.get()) {
            motive_seek.set_max(1.);
        
            hind_move.forward(0.5);

            hind_move.turn(dir.dir().to_turn());
        } else {
            // avoid
            motive_avoid.set_max(1.);
            hind_move.forward(0.6);

            let turn = dir.dir().to_turn();

            hind_move.turn(Turn::Unit(- turn.to_unit()));
        }
    }
}

impl<I: SeekInput, M: MotiveTrait> Plugin for MidSeekPlugin<I, M> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<HindMovePlugin>(), "MidSeek requires HindMovePlugin");
        assert!(app.contains_resource::<I>(), "MidSeek requires resource {}", type_name::<I>());
        
        let seek = TegSeek::<I>::new();
        app.insert_resource(seek);

        Motives::insert::<Seek>(app, Seconds(0.2));
        Motives::insert::<Avoid>(app, Seconds(0.2));

        StriatumGate::<SeekGate<I>>::init(app);

        app.system(Tick, update_seek::<I, M>);
    }
}

pub struct SeekGate<I: SeekInput> {
    marker: PhantomData<I>,
}

impl<I: SeekInput> Default for SeekGate<I> {
    fn default() -> Self {
        Self { marker: Default::default() }
    }
}

impl<I: SeekInput> Gate for SeekGate<I> {
    
}
