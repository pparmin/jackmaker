use std::f32::consts::TAU;
use std::{io, process};
use std::str::FromStr;

use jack::AudioOut;

#[derive(Debug)]
pub enum OscForm {
    Sine,
    Saw,
    Sqr,
    Tri,
    Mouse,
}

#[derive(Default, Debug)]
pub struct Coordinate {
    pub pos_x: f32,
    pub pos_y: f32,
}

pub struct Osc {
    pub freq: f32,
    pub phase: f32,
    pub amp: f32,
    pub out: jack::Port<AudioOut>,
    pub form: OscForm,
    pub receiver: crossbeam_channel::Receiver<Coordinate>,
}

impl jack::ProcessHandler for Osc {
    fn process(&mut self, client: &jack::Client, ps: &jack::ProcessScope) -> jack::Control {
        let sr = client.sample_rate() as f32;
        let out = self.out.as_mut_slice(ps);

        for o in out.iter_mut() {
            self.phase += self.freq / sr;
            while self.phase >= 1.0 {
                self.phase -= 1.0;
            }

            match &self.form {
                OscForm::Sine => {
                    *o = self.amp * (TAU * self.phase).sin();
                }
                OscForm::Saw => {
                    *o = self.amp * (2.0 * self.phase);
                }
                OscForm::Sqr => {
                    if self.phase < 0.5 {
                        *o = self.amp * -1.0;
                    } else if self.phase >= 0.5 {
                        *o = self.amp * 1.0;
                    }                
                }
                OscForm::Tri => {
                    if self.phase < 0.5 {
                        *o = self.amp * (4.0 * self.phase - 1.0);
                    } else if self.phase >= 0.5 {
                        *o = self.amp *  (4.0 * (1.0 - self.phase) - 1.0);
                    }
                }
                OscForm::Mouse => {
                    let coordinate = self.receiver.recv().expect("Error while receiving coordinate in audio thread");
                    println!("Received mouse coordinate: {:?}", coordinate)
                }
            }
            println!("current sample: {}", o);
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