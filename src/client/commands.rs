// da vedere come implementare i comandi bene
use const_format::formatcp;
use crate::client::network::peer::Peer;

pub const MESSAGE_HELP: &str = "Avaliable commands:
    \x1b[96mconnect\x1b[95m <username>\x1b[0m - connects to a user
    \x1b[96mhelp\x1b[95m <command>\x1b[0m - shows help for a command
    \x1b[96mexit\x1b[0m - exits the program";

pub fn help(args: Vec<&str>) -> &str {
    match args.len() {
        0 => MESSAGE_HELP,
        1 => {
            match args[0] {
                "connect" => "\x1b[96mconnect \x1b[95m<username>\x1b[0m: attempt to connect to another user.",
                _ => "Unknown command. Type \"\x1b[96mhelp\x1b[0m\" to see a list of commands."
            }
        },
        _ => "Too many arguments. Type \"\x1b[96mhelp \x1b[95m<command>\x1b[0m\" to read how to use a command."
    }
}

pub fn connect(args: Vec<&str>,peer: Peer) -> &str{
    match args.len() {
        1 => "Missing argument. Type \"help <command>\" to read how to use a command.",
        2 => {
            if args[1] == peer.username{
                "You can't connect to yourself."
            } else {
                "Connecting..."
            }
        },
        _ => "Too many arguments. Type \"help <command>\" to read how to use a command."
    }
} 