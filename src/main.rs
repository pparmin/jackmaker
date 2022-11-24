use std::io::Read;
// use crossbeam_channel::{unbounded, bounded, select};

use jackmaker::{Osc, OscForm, ShutdownHandler, Coordinate};
use winit::{
    dpi::PhysicalPosition,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {

    let (mod_sender, mod_receiver) = crossbeam_channel::unbounded::<Coordinate>();
    
    // Move audio playback into another thread
    std::thread::spawn(move || {
        // open client
        let (client, _status) = jack::Client::new("jackmaker", jack::ClientOptions::NO_START_SERVER)
            .expect("failed to open client");

        // register output audio port
        let port_out = client
            .register_port("osc_out", jack::AudioOut::default())
            .expect("failed to register port");

        let oscil = Osc {
            freq: 220.0,
            phase: 0.0,
            amp: 0.1,
            out: port_out,
            form: OscForm::Mouse,
            receiver: mod_receiver,
        };

        let client_active = client
        .activate_async(ShutdownHandler {}, oscil)
        .expect("failed to activate client");

        client_active
            .as_client()
            .connect_ports_by_name("jackmaker:osc_out", "system:playback_1")
            .expect("failed to connect port to playback_1");

        client_active
            .as_client()
            .connect_ports_by_name("jackmaker:osc_out", "system:playback_2")
            .expect("failed to connect port to playback_2");

        std::io::stdin().read_exact(&mut [0]).unwrap_or(());

        client_active
            .deactivate()
            .expect("failed to deactive client");

    });

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut _curser_position = PhysicalPosition::new(0.0, 0.0);

    // Run GUI event loop in main thread
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                _curser_position = position;
                //println!("Current mouse position: {:?}", _curser_position);
                let coordinate = Coordinate { 
                    pos_x: position.x as f32,
                    pos_y: position.y as f32,
                 };
                mod_sender.send(coordinate).expect("Error while sending coordinates to audio thread");
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

