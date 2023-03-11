use ticker::Ticker;

use crate::{Gram, Topos, gram, MindBuilder, attention::task::Task};

#[test]
fn test_task() {
    let mut system = MindBuilder::new();
    let detector = system.ticker(TestDetector::new());

    let task = Task::new(&mut system);

    let mut on_task = detector.sink(
        |d, msg| {
        d.on_task(msg.0);
    });

    task.on_task().to(&mut on_task);

    let gain_cue = system.external_source();

    gain_cue.source().to(&task.on_gain_cue());

    let mut system = system.build();
    let detector = detector.unwrap();
    let gain_cue = gain_cue.fiber();

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");
    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");

    detector.write(|x| x.input = 0.75);
    system.tick();
    assert_eq!(detector.write(|x| x.take()), "detect(0.75 in 0.5)");

    detector.write(|x| x.input = 0.25);
    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");

    gain_cue.send((gram("gain"), Topos::Nil));
    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "detect(0.25 in 0.25)");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "detect(0.25 in 0.25)");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "detect(0.25 in 0.25)");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "detect(0.25 in 0.25)");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");

    system.tick();
    assert_eq!(detector.write(|x| x.take()), "");
}

struct TestDetector {
    input: f32,
    threshold: f32,
    value: Vec<String>,
    on_task: bool,
}

impl TestDetector {
    fn new() -> Self {
        Self {
            input: 0.,
            threshold: 0.5,
            value: Vec::new(),
            on_task: false,
        }
    }

    fn on_task(&mut self, _gram: Gram) {
        self.threshold = 0.25;
    }

    fn take(&mut self) -> String {
        let value = self.value.join(", ");
        self.value.drain(..);

        value
    }
}

impl Ticker for TestDetector {
    fn tick(&mut self, _ctx: &mut ticker::Context) {
        if self.threshold <= self.input {
            self.value.push(format!("detect({} in {})", self.input, self.threshold));
        }

        self.threshold = 0.5;
    }
}