use ticker::{Ticker};

use crate::{MindBuilder, TickerBuilder, MindMessage, Topos, gram::Gram, Source, Sink};

#[cfg(test)]
mod tests;

struct ActionGroup {
    ticker: TickerBuilder<ActionTicker>,
    actions: Vec<Sink>,

    on_action_copy: Source,
}

struct ActionTicker {
    
}

impl ActionGroup {
    fn new(mind: &mut MindBuilder) -> Self {
        let inner = ActionTicker {};
        let mut ticker = mind.ticker(inner);
        // todo() add theta

        //let mut ptr = ticker.ptr();

        /*
        let to_enhance = ticker.fiber(move |(s, msg)| {
            ptr.borrow().enhance(s, msg);
        });
        */

        todo!()
        /* 
        ActionGroup {
            actions: Vec::new(),

            on_action_copy: ticker.fiber(),

            ticker: ticker,
        }
        */
    }

    fn push<A:Action>(&mut self, action: A) {
        //let mut ptr = self.ticker.ptr();
        action.on_complete().to(&self.ticker.sink(move |t: &mut ActionTicker, msg| {
            t.complete(msg.0, msg.1);
        }));
        let item = ActionItem {};


    }
}

trait Action {
    fn on_complete(&self)->Source;
}
struct ActionItem {

}

impl ActionItem {
    fn test(&self, source: usize, msg: MindMessage) {
        
    }
}

impl ActionTicker {
    fn enhance(&self, index: usize, msg: MindMessage) {
        
    }

    fn complete(&self, gram: Gram, topos: Topos) {
        
    }
}


impl Ticker for ActionTicker {
    fn tick(&mut self, ticks: u64) {

    }
}