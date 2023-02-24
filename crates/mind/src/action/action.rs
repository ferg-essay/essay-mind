use crate::{TickerBuilder, MindBuilder};

pub struct ActionBuilder {
    ticker: TickerBuilder<Action>,    
}

impl ActionBuilder {
    pub fn new(system: &mut MindBuilder) -> Self {
        Self {
            ticker: system.node(Action {}),
        }
    }
}

struct Action {

}