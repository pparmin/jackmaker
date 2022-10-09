use jack::AudioOut;
use std::f32::consts::TAU;
use std::{io, process};
use std::str::FromStr;


//const M_PI: f64 = 3.14159265358979323846;

#[derive(Debug)]
pub enum OscForm {
    Sine,
    Saw,
    Sqr,
    Tri,
}

pub struct Osc {
    pub freq: f32,
    pub phase: f32,
    pub amp: f32,
    pub out: jack::Port<AudioOut>,
    pub form: OscForm,
}

impl jack::ProcessHandler for Osc {
    fn process(&mut self, client: &jack::Client, ps: &jack::ProcessScope) -> jack::Control {
        let sr = client.sample_rate() as f32;
        let out = self.out.as_mut_slice(ps);

        for o in out.iter_mut() {
            match &self.form {
                OscForm::Sine => {
                    *o = self.amp * (TAU * self.phase).sin();

                    self.phase += self.freq / sr;
                    while self.phase >= 1.0 {
                        self.phase -= 1.0;
                    }
                }
                _ => {
                    println!("Generating a signal for {:?} is currently not implemented", self.form);
                }
            }
        }
    jack::Control::Continue
    }
}

pub struct ShutdownHandler { }

impl jack::NotificationHandler for ShutdownHandler {
    fn shutdown(&mut self, _status: jack::ClientStatus, _reason: &str) {
        eprintln!("jack server is shutting down: {}", _reason);
        process::exit(1);
    }
}
  

/// Attempt to read a frequency from standard in. Will block until there is
/// user input. `None` is returned if there was an error reading from standard
/// in, or the retrieved string wasn't a compatible u16 integer.
pub fn read_freq() -> Option<f64> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => u16::from_str(user_input.trim()).ok().map(|n| n as f64),
        Err(_) => None,
    }
}