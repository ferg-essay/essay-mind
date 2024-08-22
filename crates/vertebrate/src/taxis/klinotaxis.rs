// 
// interpeduncular klinotaxis: Hb.m and R.ip
//

use std::{any::type_name, marker::PhantomData};

use essay_ecs::{app::{App, Plugin}, core::{Res, ResMut}};
use mind_ecs::{AppTick, Tick};

use crate::{
    body::Body, motive::{Motive, MotiveTrait, Motives}, 
    hind_move::{_HindMove, _HindMovePlugin}, 
    mid_move::SeekInput, 
    striatum::{Gate, StriatumGate}, 
    taxis::chemotaxis::{Avoid, Seek}, 
    util::{DecayValue, Seconds}
};

pub struct Klinotaxis<I: SeekInput> {
    lateral: Vec<f32>,
    short_average: DecayValue,
    long_average: DecayValue,

    striatum: KlinotaxisStriatum<I>,

    marker: PhantomData<I>,
}

impl<I: SeekInput> Klinotaxis<I> {
    const THRESHOLD : f32 = 1e-3;
    const IS_TURN : bool = true;
    const IS_U_TURN : bool = true;

    fn new() -> Self {
        let mut values = Vec::new();

        for _ in 0..3 {
            // values.push(DecayValue::new(Seconds(0.2)));
            values.push(0.);
        }

        Self {
            lateral: values,
            short_average: DecayValue::new(Seconds(0.5)),
            long_average: DecayValue::new(Seconds(1.0)),

            striatum: KlinotaxisStriatum::new(),

            marker: PhantomData::default(),
        }
    }

    fn update(&mut self, head_cast: f32) {
        let bin = self.bin(head_cast);

        self.lateral[bin] = 0.;
        self.short_average.update();
        self.long_average.update();

        // self.striatum.update(tick);
    }

    fn update_signal(&mut self, head_cast: f32, value: f32) {
        let bin = self.bin(head_cast);

        self.lateral[bin] = value;
        self.short_average.add(value);
        self.long_average.add(value);

        // println!("AVG {} bin {} bins {:?}", (self.short_average.value() - self.long_average.value()), bin, self.lateral);
        // self.striatum.update(tick);
    }

    fn bin(&self, head_cast: f32) -> usize {
        let bin = (0.5 * (head_cast + 1.) * self.lateral.len() as f32) as usize;
        let bin = bin.min(self.lateral.len() - 1);

        bin
    }

    fn is_left_turn(&self) -> bool {
        if ! Self::IS_TURN {
            return false;
        }

        if self.lateral[0] < self.lateral[1] && self.lateral[2] < self.lateral[1] {
            false
        } else {
            (self.lateral[0] - self.lateral[2]) > Self::THRESHOLD
        }
    }

    fn is_right_turn(&self) -> bool {
        if ! Self::IS_TURN {
            return false;
        }

        if self.lateral[0] < self.lateral[1] && self.lateral[2] < self.lateral[1] {
            false
        } else {
            (self.lateral[2] - self.lateral[0]) > Self::THRESHOLD
        }
    }

    fn is_moving_away(&self) -> bool {
        if ! Self::IS_U_TURN {
            return false;
        }

        (self.long_average.value() - self.short_average.value()) > 1e-3
    }

    fn is_moving_toward(&self) -> bool {
        if ! Self::IS_U_TURN {
            return false;
        }

        (self.short_average.value() - self.long_average.value()) > 1e-3
    }
}

pub struct KlinotaxisStriatum<I: SeekInput> {
    ltd_buildup: DecayValue,
    ltd_decay: DecayValue,

    marker: PhantomData<I>,
}

impl<I: SeekInput> KlinotaxisStriatum<I> {
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

//pub trait TegInput : Send + Sync + 'static {
    // fn seek_dir(&self) -> Option<DirVector>;
//}

pub struct KlinotaxisPlugin<I: SeekInput, M: MotiveTrait> {
    marker: PhantomData<(I, M)>,
}

impl<I: SeekInput, M: MotiveTrait> KlinotaxisPlugin<I, M> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData::<(I, M)>::default(),
        }
    }
}

fn update_seek<I: SeekInput, M: MotiveTrait>(
    mut seek: ResMut<Klinotaxis<I>>,
    hind_move: ResMut<_HindMove>,
    input: Res<I>,
    motive: Res<Motive<M>>,
    tick: Res<AppTick>,
    body: Res<Body>,
    mut motive_seek: ResMut<Motive<Seek>>,
    mut motive_avoid: ResMut<Motive<Avoid>>,
) {
    if ! motive.is_active() {
        return;
    }

    let head_cast = body.head_cast();
    seek.update(head_cast);

    if let Some(goal) = input.seek_dir() {
        seek.update_signal(head_cast, goal.value());

        if seek.striatum.update(tick.get()) {
            motive_seek.set_max(1.);
        
            hind_move.forward(0.5);

            if seek.is_moving_away() {
                hind_move.u_turn(1.);
            } else {
                if seek.is_left_turn() && seek.is_right_turn() {

                } else if seek.is_left_turn() {
                    hind_move.left_brake(0.5);
                } else if seek.is_right_turn() {
                    hind_move.right_brake(0.5);
                }
            }
        } else {
            // avoid
            motive_avoid.set_max(1.);
            hind_move.forward(0.6);

            if seek.is_moving_toward() {
                hind_move.u_turn(1.)
            } else {
                if seek.is_left_turn() && seek.is_right_turn() {

                } else if seek.is_right_turn() {
                    hind_move.left_brake(0.5);
                } else if seek.is_left_turn() {
                    hind_move.right_brake(0.5);
                }
            }
        }
    }
}

impl<I: SeekInput, M: MotiveTrait> Plugin for KlinotaxisPlugin<I, M> {
    fn build(&self, app: &mut App) {
        assert!(app.contains_plugin::<_HindMovePlugin>(), "TegSeek requires HindMovePlugin");
        assert!(app.contains_resource::<I>(), "TegSeek requires resource {}", type_name::<I>());
        
        let seek = Klinotaxis::<I>::new();
        app.insert_resource(seek);

        app.system(Tick, update_seek::<I, M>);

        Motives::insert::<Seek>(app, Seconds(0.2));
        Motives::insert::<Avoid>(app, Seconds(0.2));

        StriatumGate::<SeekGate<I>>::init(app);
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
