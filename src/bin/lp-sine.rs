use lowpass::osc::*;
use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut args = std::env::args();
    let osc = Oscillator {
        omega: args.nth(1).ok_or(Box::<dyn Error>::from("Usage: lp-sine OMEGA"))?.parse()?,
        zeta: 0
    };
    let mut osc_s = osc.initial_state();

    loop {
        match handle.write_all(&osc_s.y0.to_le_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        }
        match handle.write_all(&osc_s.y1.to_le_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        }
        osc_s = osc.step(osc_s, 0);
    }
    Ok(())
}
