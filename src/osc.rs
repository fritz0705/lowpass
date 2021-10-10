use std::convert::TryFrom;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct OscillatorState {
    pub y0: i32,
    pub y1: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Oscillator {
    pub zeta: i32,
    pub omega: i32,
}

fn mult_div(a: i32, b: i32, c: i32) -> i32 {
    match a.checked_mul(b) {
        Some(ab) => ab / c,
        None => (a / c).saturating_mul(b),
    }
}

#[inline]
fn add_div(a: i32, b: i32, c: i32) -> i32 {
    i32::try_from((i64::from(a) + i64::from(b)) / (i64::from(c)))
        .unwrap_or(i32::MAX)
}

pub fn mult3x(y: i32, omega: i32, zeta: i32) -> i32 {
    let y = i64::from(y);
    let omega = i64::from(omega);
    let zeta = i64::from(zeta);
    let res = y.checked_mul(omega)
        .and_then(|yomega| yomega.checked_mul(zeta))
        .map(|yomegazeta| yomegazeta >> 32);
    match res {
        Some(yomegazeta) => i32::try_from(yomegazeta).unwrap_or(0),
        None => 0
    }
}

impl Oscillator {
    pub fn from_omega(omega: i32) -> Oscillator {
        Oscillator {
            omega: omega,
            zeta: 0,
        }
    }
    pub fn step(&self, state: OscillatorState, x0: i32) -> OscillatorState {
        // System of ODE
        //   { y0' = y1
        //   { y1' = x - zeta xi y1 - xi^2 y0
        // 4th order Runge--Kutta
        //
        let k1_0 = state.y1;
        let k1_1 = x0
            .wrapping_sub(mult3x(state.y1, self.omega, self.zeta))
            .wrapping_sub(mult3x(state.y0, self.omega, self.omega));
        let k2_0 = k1_0.wrapping_add(k1_1 / 2);
        let k2_1 = x0
            .wrapping_sub(mult3x(state.y1, self.omega, self.zeta))
            .wrapping_sub(mult3x(k1_1 / 2, self.omega, self.zeta))
            .wrapping_sub(mult3x(state.y0, self.omega, self.omega))
            .wrapping_sub(mult3x(k1_0 / 2, self.omega, self.omega));
        let k3_0 = k1_0.wrapping_add(k2_1 / 2);
        let k3_1 = x0
            .wrapping_sub(mult3x(state.y1, self.omega, self.zeta))
            .wrapping_sub(mult3x(k2_1 / 2, self.omega, self.zeta))
            .wrapping_sub(mult3x(state.y0, self.omega, self.omega))
            .wrapping_sub(mult3x(k2_0 / 2, self.omega, self.omega));
        let k4_0 = k1_0.wrapping_add(k3_1);
        let k4_1 = x0
            .wrapping_sub(mult3x(state.y1, self.omega, self.zeta))
            .wrapping_sub(mult3x(k3_1, self.omega, self.zeta))
            .wrapping_sub(mult3x(state.y0, self.omega, self.omega))
            .wrapping_sub(mult3x(k3_0, self.omega, self.omega));
        OscillatorState {
            y0: state
                .y0
                .wrapping_add(add_div(k1_0, k4_0, 3))
                .wrapping_add(add_div(k2_0, k3_0, 6)),
            y1: state.y1 + (add_div(k1_1, k4_1, 3)) + (add_div(k2_1, k3_1, 6)),
        }
    }

    pub fn many_steps(&self, mut state: OscillatorState, x0s: Vec<i32>) -> Vec<OscillatorState> {
        let mut res = Vec::with_capacity(x0s.len());
        for x0 in x0s {
            state = self.step(state, x0);
            res.push(state)
        }
        res
    }

    pub fn initial_state(&self) -> OscillatorState {
        OscillatorState {
            y0: i32::MAX / 5 * 4,
            y1: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RectangleState {
    pub y0: i32,
    pub osc: OscillatorState,
}

#[derive(Copy, Clone)]
pub struct Rectangle(Oscillator);

impl Rectangle {
    pub fn step(&self, state: RectangleState, x0: i32) -> RectangleState {
        let osc = self.0.step(state.osc, x0);
        RectangleState {
            osc: osc,
            y0: osc.y0.signum() * i32::MAX,
        }
    }

    pub fn initial_state(&self) -> RectangleState {
        RectangleState {
            y0: 0,
            osc: self.0.initial_state(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TriangleState {
    pub y0: i32,
    pub osc: OscillatorState,
}

#[derive(Copy, Clone)]
pub struct Triangle(Oscillator);

impl Triangle {
    pub fn from_omega(omega: i32) -> Self {
        Triangle(Oscillator {
            omega: omega,
            zeta: 0,
        })
    }

    pub fn step(&self, state: TriangleState, x0: i32) -> TriangleState {
        // XXX Magic constant XXX
        let amplitude = self.0.omega * 10430;
        let osc = self.0.step(state.osc, x0);
        TriangleState {
            y0: state.y0.saturating_add(osc.y0.signum() * amplitude),
            osc: osc,
        }
    }

    pub fn initial_state(&self) -> TriangleState {
        TriangleState {
            y0: 0,
            osc: self.0.initial_state(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct SawtoothState {
    pub y0: i32,
    pub triangle: TriangleState,
}

#[derive(Copy, Clone)]
pub struct Sawtooth(Triangle);

impl Sawtooth {
    pub fn from_omega(omega: i32) -> Self {
        Sawtooth(Triangle::from_omega(omega))
    }

    pub fn step(&self, state: SawtoothState, x0: i32) -> SawtoothState {
        let state = self.0.step(state.triangle, x0);
        SawtoothState {
            y0: state.y0 * state.osc.y0.signum(),
            triangle: state,
        }
    }

    pub fn initial_state(&self) -> SawtoothState {
        SawtoothState {
            y0: 0,
            triangle: self.0.initial_state(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct FM {
    pub carrier: Oscillator,
    pub modulator: Oscillator,
    pub index: i32,
}

#[derive(Copy, Clone)]
pub struct FMState {
    pub y0: i32,
    pub carrier: OscillatorState,
    pub modulator: OscillatorState,
}

impl FM {
    pub fn step(&self, state: FMState, x0: i32) -> FMState {
        let modulator_state = self.modulator.step(state.modulator, 0);
        let carrier_state = Oscillator {
            omega: self.carrier.omega + mult_div(modulator_state.y0 >> 15, self.index, 16),
            zeta: self.carrier.zeta,
        }
        .step(state.carrier, x0);
        FMState {
            y0: carrier_state.y0,
            carrier: carrier_state,
            modulator: modulator_state,
        }
    }

    pub fn initial_state(&self) -> FMState {
        FMState {
            y0: 0,
            carrier: self.carrier.initial_state(),
            modulator: self.modulator.initial_state(),
        }
    }
}
