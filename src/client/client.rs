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
    net::UdpSocket,

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

    let config = Config::new();
    let socket = UdpSocket::bind(config.server_addr.clone()).expect("Unable to bind to socket");
    let network_handler_rx =Arc::new(NetworkHandler::new(socket));
    let network_handler_tx = network_handler_rx.clone();
    let microphone = Microphone::new(microphone_recv_rx, microphone_send_tx);

    let ui = Ui::new(ui_input_tx, ui_display_rx);

        //start all the threads
    let networking_upstream_thread = thread::spawn(move || {
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
            let input = ui_input_rx.recv().unwrap();
            ui_display_tx.send(input).unwrap();
        }
    });
    let ui_display_thread = thread::spawn(move || {
        //start the ui display thread
        loop{
            let to_display = ui_display_rx.recv().unwrap();
        }
    });
    let microphone_recv_thread = thread::spawn(move || {
        //start the microphone recv thread
    });
    let microphone_send_thread = thread::spawn(|| {
        //start the microphone send thread
    });

    //join all the threads
    networking_upstream_thread.join().unwrap();
    networking_downstream_thread.join().unwrap();
    ui_input_thread.join().unwrap();
    ui_display_thread.join().unwrap();
    microphone_recv_thread.join().unwrap();
    microphone_send_thread.join().unwrap();
}
