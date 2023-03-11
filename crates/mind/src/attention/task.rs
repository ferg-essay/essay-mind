use ticker::Ticker;

use crate::{Gram, Topos, MindBuilder, TickerBuilder, Sink, Source, Fiber, gram};

pub struct Task {
    ticker: TickerBuilder<TaskInner>,
    gain_cue: Sink,
    on_task: Source,
}

impl Task {
    pub fn new(system: &mut MindBuilder) -> Self {
        let mut ticker = system.ticker(TaskInner::new());

        let gain_cue = ticker.sink(|t, msg| {
            t.gain_cue(msg.0, msg.1);
        });

        let on_task = ticker.source(
            |t, fiber| {
            t.on_task = fiber;
        });

        Self {
            ticker: ticker,
            gain_cue: gain_cue,
            on_task: on_task,
        }
    }

    pub fn on_gain_cue(&self) -> Sink {
        self.gain_cue.clone()
    }

    pub fn on_task(&self) -> Source {
        self.on_task.clone()
    }

}

struct TaskInner {
    on_task: Fiber,
    ticks: u32,
}

impl TaskInner {
    fn new() -> Self {
        Self {
            on_task: Default::default(),
            ticks: 0,
        }
    }
    pub fn gain_cue(&mut self, _gram: Gram, _topos: Topos) {
        self.ticks = 4;
    }
}

impl Ticker for TaskInner {
    fn tick(&mut self, ctx: &mut ticker::Context) {
        if self.ticks > 0 {
            self.ticks -= 1;
            self.on_task.send((gram("task"), Topos::Nil));
        }
    }
}