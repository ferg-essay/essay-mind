
pub struct TickerBuilderData {
    pub is_built : bool,

    pub fiber_id : i32,
}

impl TickerBuilderData {
    pub fn fiber_id(&mut self) -> i32 {
        self.fiber_id += 1;
        self.fiber_id
    }

    pub fn build(&mut self) {
        assert!(! self.is_built);
        self.is_built = true;
    }
}
