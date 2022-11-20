// use std::{io::Read};
// use std::io;
// use jack::Client;

use std::io::Read;

use jackmaker::{Osc, OscForm, ShutdownHandler};
use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // open client
    let (client, _status) = jack::Client::new("jackmaker", jack::ClientOptions::NO_START_SERVER)
        .expect("failed to open client");

    // register output audio port
    let port_out = client
        .register_port("sine_out", jack::AudioOut::default())
        .expect("failed to register port");

    let sine = Osc {
        freq: 220.0,
        phase: 0.0,
        amp: 0.1,
        out: port_out,
        form: OscForm::Mouse,
    };
    match sine.form {

        // Since winit will only build a window on the main thread, mouse modulation currently has to be handled in main.rs
        // All other audio creation forms currently implemented in lib.rs will be handled through the library file.
        // This may change in a future redesign, but as I want to move ahead with the actual task of working with audio,
        // I have decided to use the current implementation for the time being.
        OscForm::Mouse => calc_mouse_pos(),
        _ => (),
    }

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
        .expect("failed to connect port to playback_2");

    std::io::stdin().read_exact(&mut [0]).unwrap_or(());

    client_active
        .deactivate()
        .expect("failed to deactive client");
}

fn calc_mouse_pos() {
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut _curser_position = PhysicalPosition::new(0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                _curser_position = position;
                println!("Current mouse position: {:?}", _curser_position);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            _ => (),
        }
    });
}
