pub struct Sequencer {
    current: u64,
}

impl Sequencer {
    pub fn new(start: u64) -> Sequencer {
        Sequencer { current: start }
    }

    pub fn get(&mut self) -> u64 {
        let ret = self.current;
        self.current += 1;
        ret
    }
}
