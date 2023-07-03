use std::{
    sync::{mpsc::{
        channel,
        Sender,
        Receiver,
    },
    Arc
    },
    thread,
    time::Duration,
    net::{UdpSocket, IpAddr},

};
use cpal::{
    Data,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use crate::utils::{
    networking::{
        packet::Packet,
        networkhandler::NetworkHandler,
    },
    config::Config,
};
use crate::client::{
    microphone::Microphone,
    speaker::Speaker,
    ui::ui::Ui,
};


pub fn start(){

    let (networking_send_tx, networking_send_rx) = channel::<Packet>();
    let (networking_recv_tx, networking_recv_rx) = channel::<Packet>();
    let (ui_input_tx, ui_input_rx) = channel::<String>();
    let (ui_display_tx, ui_display_rx) = channel::<String>();
    let (microphone_recv_tx, microphone_recv_rx) = channel::<Vec<u8>>();
    let (microphone_send_tx, microphone_send_rx) = channel::<Vec<u8>>();
    let (speaker_recv_tx, speaker_recv_rx) = channel::<Vec<u8>>();
    let (speaker_send_tx, speaker_send_rx) = channel::<Vec<u8>>();

    let config: Config = Config::new();
    let socket: UdpSocket = UdpSocket::bind(config.server_addr+":23232").expect("Unable to bind to socket");
    let network_handler_rx =Arc::new(NetworkHandler::new(socket));
    let network_handler_tx = network_handler_rx.clone();

    let microphone: Microphone = Microphone::new(microphone_recv_rx, microphone_send_tx);
    let speaker:Speaker = Speaker::new(speaker_recv_rx, speaker_send_tx);

    let ui = Ui::new(ui_input_tx, ui_display_rx);

        //start all the threads
    let networking_upstream_thread = thread::spawn(move || {
            //start the networking upstream thread
        loop{
            let packet = networking_send_rx.recv();
            //network_handler_rx.send(packet);
        }
    });
    let networking_downstream_thread = thread::spawn(move || {
        //start the networking downstream thread
        loop{
            let packet = network_handler_tx.recv();
            networking_recv_tx.send(packet);
        }
    });
    let ui_input_thread = thread::spawn(move || {
        //start the ui input thread
        loop{
            let input = ui_input_rx.recv();
        }
    });
    let ui_display_thread = thread::spawn(move || {
        //start the ui display thread
        loop{
            let display = String::from("test");
            ui_display_tx.send(display);
        }
    });
    let microphone_recv_thread: Result<cpal::Stream, cpal::BuildStreamError> = microphone.selected_microphone.build_output_stream(
        &microphone.supported_config,
        move |_data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            log::debug!("microphone stream called");
        },
        move |err| {
            log::error!("an error occurred on microphone stream: {}", err);
        },
        None // None=blocking, Some(Duration)=timeout
    );
    let speaker_send_thread: Result<cpal::Stream, cpal::BuildStreamError> = speaker.selected_speaker.build_input_stream(
        &speaker.supported_config,
        move |_data: & [f32], _: &cpal::InputCallbackInfo| {
            
        },
        move |err| {
            log::error!("an error occurred on speaker stream: {}", err);
        },
        None,

         // None=blocking, Some(Duration)=timeout
    );

    microphone_recv_thread.unwrap().play()
    .expect("failed to record microphone");

    speaker_send_thread.unwrap().play()
    .expect("failed to play speaker");

    //join all the threads
    networking_upstream_thread.join().unwrap();
    networking_downstream_thread.join().unwrap();
    ui_input_thread.join().unwrap();
    ui_display_thread.join().unwrap();
}
