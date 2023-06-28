use std::{
    sync::mpsc::{
        channel,
        Sender,
        Receiver,
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
    voice::Voice,
    ui::ui::Ui,
};



pub struct Client{
    pub networking_send_tx: Sender<Packet>,
    pub networking_recv_rx: Receiver<Packet>,
    pub ui_input_rx: Receiver<String>,
    pub ui_display_tx: Sender<String>,
    pub voice_recv_tx: Sender<Vec<u8>>,
    pub voice_send_rx: Receiver<Vec<u8>>,
    pub network_handler: NetworkHandler,
    pub config: Config,
    pub voice: Voice,
    pub ui: Ui
}

impl Client{
    
    pub fn new() -> Client{
        let (networking_send_tx, networking_send_rx) = channel::<Packet>();
        let (networking_recv_tx, networking_recv_rx) = channel::<Packet>();
        let (ui_input_tx, ui_input_rx) = channel::<String>();
        let (ui_display_tx, ui_display_rx) = channel::<String>();
        let (voice_recv_tx, voice_recv_rx) = channel::<Vec<u8>>();
        let (voice_send_tx, voice_send_rx) = channel::<Vec<u8>>();

        let config = Config::new();
        let socket = UdpSocket::bind(config.server_addr.clone()).expect("Unable to bind to socket");
        let network_handler = NetworkHandler::new(socket, networking_send_rx, networking_recv_tx);

        let voice = Voice::new(voice_recv_rx, voice_send_tx);

        let ui = Ui::new(ui_input_tx, ui_display_rx);

        Client{
            networking_send_tx,
            networking_recv_rx,
            ui_input_rx,
            ui_display_tx,
            voice_recv_tx,
            voice_send_rx,
            network_handler,
            config,
            voice,
            ui,
        }
    }

    pub fn start(&mut self){

        //start all the threads
        let networking_upstream_thread = thread::spawn(|| {
            //start the networking upstream thread
        });
        let networking_downstream_thread = thread::spawn(|| {
            //start the networking downstream thread
        });
        let ui_input_thread = thread::spawn(|| {
            //start the ui input thread
        });
        let ui_display_thread = thread::spawn(|| {
            //start the ui display thread
        });
        let voice_recv_thread = thread::spawn(|| {
            //start the voice recv thread
        });
        let voice_send_thread = thread::spawn(|| {
            //start the voice send thread
        });

        //join all the threads
        networking_upstream_thread.join().unwrap();
        networking_downstream_thread.join().unwrap();
        ui_input_thread.join().unwrap();
        ui_display_thread.join().unwrap();
        voice_recv_thread.join().unwrap();
        voice_send_thread.join().unwrap();
    }
}