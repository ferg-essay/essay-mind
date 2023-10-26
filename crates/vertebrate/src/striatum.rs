use essay_tensor::Tensor;

use crate::action::{StriatumSnr, StriatumId};

pub struct StriatumStn {
    direct: StriatumDirect,
    indirect: StriatumIndirect,

    dopamine: Dopamine,
}

impl StriatumStn {
    pub fn new() -> Self {
        Self {
            direct: StriatumDirect::new(),
            indirect: StriatumIndirect::new(),
            dopamine: Dopamine::None,
        }
    }

    pub fn direct(&self) -> &StriatumDirect {
        &self.direct
    }

    pub fn direct_mut(&mut self) -> &mut StriatumDirect {
        &mut self.direct
    }

    pub fn indirect(&self) -> &StriatumIndirect {
        &self.indirect
    }

    pub fn indirect_mut(&mut self) -> &mut StriatumIndirect {
        &mut self.indirect
    }

    pub fn update(
        &mut self, 
        dopamine: Dopamine,
        snr_d: &mut dyn StriatumSnr,
        snr_i: &mut dyn StriatumSnr,
    ) {
        self.dopamine = dopamine;

        let select_d = self.direct_mut().update(dopamine);
        let select_i = self.indirect_mut().update(dopamine);

        if let Some(selected_d) = select_d {
            if let Some(selected_i) = select_i {
                if selected_i.value <= selected_d.value {
                    snr_d.attend(selected_d.id, selected_d.value);
                } else {
                    snr_i.attend(selected_i.id, selected_i.value);
                }
            } else {
                snr_d.attend(selected_d.id, selected_d.value);
            }
        } else if let Some(selected_i) = select_i {
            snr_i.attend(selected_i.id, selected_i.value);
        }
    }
}

pub struct StriatumDirect {
    actions: Vec<StriatumItem>,
    selected: Option<Selected>,
    threshold: f32,
}

impl StriatumDirect {
    pub const DECAY : f32 = 0.75;
    pub const INHIBIT : f32 = 0.2;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            selected: None,
            threshold: 0.5,
        }
    }

    pub fn add_action(
        &mut self, 
        name: impl AsRef<str>,
    ) -> StriatumId {
        let id = StriatumId::new(self.actions.len());

        self.actions.push(StriatumItem::new(id, name.as_ref()));

        id
    }

    pub fn sense(&mut self, id: StriatumId, sense: Sense) {
        self.actions[id.i()].sense = sense;
    }

    pub fn attend(&mut self, id: StriatumId, value: Sense) {
        self.actions[id.i()].attention = value.value();
    }

    fn update(
        &mut self, 
        da: Dopamine,
    ) -> Option<Selected> {
        let d1 = da.d1();

        let mut best_sense = 0.;
        let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let sense = item.sense.value();
            let attention = item.attention;
            item.sense = Sense::None; // sense may also be accumulative
            item.attention = item.attention * Self::DECAY;

            let noise = random() * 0.01;
            let scaled_sense = d1 * (sense * (1. + attention) + noise);

            if best_sense < scaled_sense {
                second = best_sense;
                best_sense = scaled_sense;
                best = Some(Selected::new(item.id, scaled_sense.clamp(0., 1.)));
            }
        }

        // TODO: reference for this heuristic (fixed block)
        let selected = if self.threshold <= best_sense - second {
            //if let Some(id) = best {
            //    self.actions[id.0].dopamine = 1.;
            //}

            best
        } else {
            None
        };

        self.selected = selected;

        self.selected.clone()
    }
}

pub struct StriatumIndirect {
    actions: Vec<StriatumItem>,
    selected: Option<Selected>,
    threshold: f32,
}

impl StriatumIndirect {
    pub const DECAY : f32 = 0.75;
    pub const INHIBIT : f32 = 0.2;

    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            selected: None,
            threshold: 0.1,
        }
    }

    pub fn add_action(
        &mut self, 
        name: impl AsRef<str>,
    ) -> StriatumId {
        let id = StriatumId::new(self.actions.len());

        self.actions.push(StriatumItem::new(id, name.as_ref()));

        id
    }

    /*
    pub fn sense(&mut self, id: IndirectId, sense: Sense) {
        println!("SenseId {:?} {:?}", id, sense);
        self.actions[id.i()].sense = sense;
    }
    */
    pub fn sense(&mut self, sense: Sense) {
        if self.actions.len() > 0 {
            self.actions[0].sense = sense;
        }
    }

    pub fn attend(&mut self, id: IndirectId, value: Sense) {
        self.actions[id.i()].attention = value.value();
    }

    fn update(
        &mut self, 
        da: Dopamine,
    ) -> Option<Selected> {
        let d2 = da.d2();

        let mut best_sense = 0.;
        let mut second = 0.;
        let mut best = None;

        for item in &mut self.actions {
            let sense = item.sense.value();
            let action = item.attention;
            item.sense = Sense::None; // sense may also be accumulative
            item.attention = item.attention * Self::DECAY;

            let noise = random() * 0.01;
            let scaled_sense = d2 * (sense * (1. + action) + noise);

            if best_sense < scaled_sense {
                second = best_sense;
                best_sense = scaled_sense;
                best = Some(Selected::new(item.id, scaled_sense));
            }
        }

        // TODO: reference for the fixed block heuristic
        // or use a softmax
        let selected = if self.threshold <= best_sense - second {
            //if let Some(id) = best {
            //    self.actions[id.0].dopamine = 1.;
            //}

            best
        } else {
            None
        };

        self.selected = selected;

        self.selected.clone()
    }
}

fn random() -> f32 {
    Tensor::random_uniform([1], ())[0]
}

struct StriatumItem {
    id: StriatumId,
    _name: String,
    sense: Sense,
    attention: f32,
}

impl StriatumItem {
    fn new(
        id: StriatumId,
        name: &str,
    ) -> StriatumItem {
        Self {
            id,
            _name: String::from(name),
            sense: Sense::None,
            attention: Sense::None.value(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dopamine {
    None,
    Low,
    High,
    Top
}

impl Dopamine {
    fn d1(&self) -> f32 {
        match self {
            Dopamine::None => 0.,
            Dopamine::Low => 0.25,
            Dopamine::High => 1.,
            Dopamine::Top => 1.5,
        }
    }

    fn d2(&self) -> f32 {
        match self {
            Dopamine::None => 0.,
            Dopamine::Low => 1.,
            Dopamine::High => 0.25,
            Dopamine::Top => 0.,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sense {
    None,
    Low,
    High,
    Top
}

impl Sense {
    fn value(&self) -> f32 {
        match self {
            Sense::None => 0.,
            Sense::Low => 1.,
            Sense::High => 1.,
            Sense::Top => 1.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectId(usize);

impl DirectId {
    pub fn new(id: usize) -> Self {
        DirectId(id)
    }

    pub fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndirectId(usize);

impl IndirectId {
    pub fn new(id: usize) -> Self {
        IndirectId(id)
    }

    pub fn i(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
struct Selected {
    id: StriatumId,
    value: f32,
}

impl Selected {
    fn new(id: StriatumId, value: f32) -> Self {
        Self {
            id,
            value,
        }
    }
}


