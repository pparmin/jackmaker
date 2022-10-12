// use std::{io::Read};
// use std::io;
// use jack::Client;

use std::io::Read;

use jackmaker::{Osc, OscForm, ShutdownHandler};

fn main() {
    // open client
    let (client, _status) = 
        jack::Client::new("jackmaker", jack::ClientOptions::NO_START_SERVER).expect("failed to open client");

    // register output audio port
    let port_out = client
        .register_port("sine_out", jack::AudioOut::default())
        .expect("failed to register port");   

    let sine = Osc {
        freq: 220.0,
        phase: 0.0,
        amp: 0.1,
        out: port_out,
        form: OscForm::Sine,
    };

    let client_active = client
        .activate_async(ShutdownHandler {}, sine)
        .expect("failed to activate client");

    client_active
        .as_client()
        .connect_ports_by_name("jackmaker:sine_out", "system:playback_1")
        .expect("failed to connect port to playback_1");
        
    client_active
        .as_client()
        .connect_ports_by_name("jackmaker:sine_out", "system:playback_2")
        .expect("failed to connect port to playback_1");

    // while let Some(_) = read_freq() {
    //         println!("main thread started...");
    //     }
    std::io::stdin().read_exact(&mut [0]).unwrap_or(());

    client_active
        .deactivate()
        .expect("failed to deactive client");
}

