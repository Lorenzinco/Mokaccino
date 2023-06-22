// da vedere come implementare i comandi bene
// use const_format::formatcp;
use crate::utils::terminal::cli;
use crate::client::network::peerConnection::PeerConnection;

pub const MESSAGE_HELP: &str = "Avaliable commands:
    \x1b[96mconnect\x1b[95m <username>\x1b[0m - connects to a user
    \x1b[96mhelp\x1b[95m <command>\x1b[0m - shows help for a command
    \x1b[96mexit\x1b[0m - exits the program";

pub fn help(args: Vec<&str>) {
    let output = match args.len() {
        0 => MESSAGE_HELP,
        1 => {
            match args[0] {
                "connect" => "\x1b[96mconnect \x1b[95m<username>\x1b[0m: attempt to connect to another user.",
                _ => "Unknown command. Type \"\x1b[96mhelp\x1b[0m\" to see a list of commands."
            }
        },
        _ => "Too many arguments. Type \"\x1b[96mhelp \x1b[95m<command>\x1b[0m\" to read how to use a command."
    };
    cli::output(output);
}

pub fn connect(args: Vec<&str>, username: &str, peer: &mut Option<PeerConnection>) {
    let output = match args.len() {
        1 => "Please specify an username.",
        2 => {
            if args[1] == username {
                "You can't connect to yourself."
            } else {
                "Attempting to connect..."
            }
        },
        _ => "Too many arguments."
    };
    cli::output(output);
} 