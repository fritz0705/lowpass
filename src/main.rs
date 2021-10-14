use lowpass::osc::*;
use std::io::{self, Write};

#[derive(Copy, Clone)]
struct State {
    osc: Oscillator,
    state: OscillatorState,
}

impl State {
    fn init() -> State {
        let osc = Oscillator {
            omega: 3775,
            zeta: 100,
        };
        State {
            osc: osc,
            state: osc.initial_state(),
        }
    }

    fn output(self) -> (i32, i32) {
        (self.state.y0, self.state.y1)
    }

    fn step(mut self) -> State {
        self.state = self.osc.step(self.state, 0);
        if self.state.y1.unsigned_abs() == 0 {
            self.state = self.osc.initial_state();
        }
        self
    }
}

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut state = State::init();

    loop {
        state = state.step();
        let (y0, y1) = state.output();
        match handle.write_all(&y0.to_le_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        }
        match handle.write_all(&y1.to_le_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
