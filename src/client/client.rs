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
    traits::{DeviceTrait, HostTrait, StreamTrait}, Sample,
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
    let (microphone_recv_tx, microphone_recv_rx) = channel::<Vec<f32>>();
    let (microphone_send_tx, microphone_send_rx) = channel::<Vec<f32>>();
    let (speaker_recv_tx, speaker_recv_rx) = channel::<Vec<f32>>();
    let (speaker_send_tx, speaker_send_rx) = channel::<Vec<f32>>();

    let config: Config = Config::new();
    let socket: UdpSocket = UdpSocket::bind(config.server_addr+":23232").expect("Unable to bind to socket");
    let network_handler_rx =Arc::new(NetworkHandler::new(socket));
    let network_handler_tx = network_handler_rx.clone();

    let microphone: Microphone = Microphone::new();
    let speaker:Speaker = Speaker::new();

    let ui = Ui::new(ui_input_tx, ui_display_rx);

        //start all the threads
    /*let networking_upstream_thread = thread::spawn(move || {
            //start the networking upstream thread
        loop{
            let packet = networking_send_rx.recv().unwrap();
            network_handler_rx.send(packet);
        }
    });
    let networking_downstream_thread = thread::spawn(move || {
        //start the networking downstream thread
        loop{
            let packet = network_handler_tx.recv();
            networking_recv_tx.send(packet).unwrap();
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
            ui_display_tx.send(display).unwrap();
        }
    });
    */
    let microphone_recv_thread: Result<cpal::Stream, cpal::BuildStreamError> = microphone.selected_microphone.build_input_stream(
        &microphone.config,
        move |data: & [f32], _: &cpal::InputCallbackInfo| {
            println!("microphone stream called");
            speaker_send_tx.send(data.to_vec()).expect("failed to send microphone data");
        },
        move |err| {
            log::error!("an error occurred on microphone stream: {}", err);
        },
        None // None=blocking, Some(Duration)=timeout
    );
    let speaker_send_thread: Result<cpal::Stream, cpal::BuildStreamError> = speaker.selected_speaker.build_output_stream(
        &speaker.config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            println!("speaker stream called");
            let vector: Vec<f32> = speaker_send_rx.recv().expect("failed to receive speaker data");
            for (i, sample) in data.iter_mut().enumerate() {
                println!("{:?}",vector[i]);
                *sample = vector[i];
            }
        },
        move |err| {
            log::error!("an error occurred on speaker stream: {}", err);
        },
        None,

         // None=blocking, Some(Duration)=timeout
    );

    //microphone_recv_thread.expect("Cant start recording audio").play()
    //    .expect("failed to play microphone");

    //speaker_send_thread.expect("Cant start playng back audio").play()
    //    .expect("failed to play speaker");

    //join all the threads
    //sleep 10 seconds
    thread::sleep(Duration::from_secs(10));
    /*
    
    networking_upstream_thread.join().unwrap();
    networking_downstream_thread.join().unwrap();
    ui_input_thread.join().unwrap();
    ui_display_thread.join().unwrap();
     */
}
