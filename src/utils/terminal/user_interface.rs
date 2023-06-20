use std::io::{self, Write};
use const_format::formatcp;
use std::env;
/*
    Definition of the user interface class, which reads the user input in the terminal
    as a separate thread, and sends the input to the main thread through a channel.
    The main thread then processes the input and sends the output to the user interface.
*/

pub const BANNER:&'static str = formatcp!("
 __    __     ______     __  __     ______     ______     ______     __     __   __     ______    
/\\  -./  \\   /\\  __ \\   /\\ \\/ /    /\\  __ \\   /\\  ___\\   /\\  ___\\   /\\ \\   /\\  -.\\ \\   /\\  __ \\   
\\ \\ \\-./\\ \\  \\ \\ \\/\\ \\  \\ \\  _ -.  \\ \\  __ \\  \\ \\ \\____  \\ \\ \\____  \\ \\ \\  \\ \\ \\-.  \\  \\ \\ \\/\\ \\  
 \\ \\_\\ \\ \\_\\  \\ \\_____\\  \\ \\_\\ \\_\\  \\ \\_\\ \\_\\  \\ \\_____\\  \\ \\_____\\  \\ \\_\\  \\ \\_\\\\ \\_\\  \\ \\_____\\ 
  \\/_/  \\/_/   \\/_____/   \\/_/\\/_/   \\/_/\\/_/   \\/_____/   \\/_____/   \\/_/   \\/_/ \\/_/   \\/_____/ 
   #version {} - Authors {} \n",env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_AUTHORS"));

pub const HELP:&str = r"

";

                                                                                              