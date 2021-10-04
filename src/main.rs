use lowpass::osc::*;
use std::io::{self, Write};

#[derive(Copy, Clone)]
struct State {
    base_osc: FM,
    base_state: FMState
}

fn incr12(omega: i32) -> i32 {
    (omega * 34716) / 32768
}

impl State {
    fn init() -> State{
        let base_osc = FM {
            carrier: Oscillator {
                omega: incr12(incr12(incr12(incr12(incr12(incr12(incr12(incr12(incr12(incr12(incr12(incr12(3775)))))))))))),
                zeta: 1
            },
            modulator: Oscillator::from_omega(3000),
            index: 1024
        };
        State {
            base_state: base_osc.initial_state(),
            base_osc: base_osc
        }
    }

    fn output(self) -> i32 {
        self.base_state.y0
    }

    fn step(mut self) -> State {
        self.base_state = self.base_osc.step(self.base_state, 0);
        self
    }
}

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut state = State::init();

    loop {
        state = state.step();
        match handle.write_all(&state.output().to_le_bytes()) {
            Ok(_) => (),
            Err(_) => break
        }
    }
}
