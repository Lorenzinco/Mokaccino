// da vedere come implementare i comandi bene

pub const MESSAGE_HELP: &str = "Avaliable commands:
    \x1b[96mconnect\x1b[95m <username>\x1b[0m - connects to a user
    \x1b[96mhelp\x1b[95m <command>\x1b[0m - shows help for a command
    \x1b[96mexit\x1b[0m - exits the program";

pub fn help(args: Vec<&str>) -> &str {
    match args.len() {
        1 => MESSAGE_HELP,
        2 => {
            match args[1] {
                "connect" => "connect <username>: attempt to connect to another user.",
                _ => "Unknown command. Type \"help\" to see a list of commands."
            }
        },
        _ => "Too many arguments. Type \"help <command>\" to read how to use a command."
    }
}