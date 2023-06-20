
pub const HELP:&str = "Avaliable commands:
    \x1b[96mhelp\x1b[95m <command>\x1b[0m - shows help for a command
    \x1b[96mconnect\x1b[95m <username>\x1b[0m - connects to a user, you have to fetch information about the user from the server first
    \x1b[96mlogin\x1b[0m - connects to the network server, information about your username can be set in the file config.json
    \x1b[96mexit\x1b[0m - exits the program";

pub fn help(args: Vec<&str>)->String{
    let mut help = String::new();
    match args.len(){
        1 => {
            help = String::from(HELP);
        },
        2 => {
            match args[1]{
                "connect" => {
                    help = String::from("type connect <username> to connect to a user,
                     to do this you have to fetch information about the user from the server first.");
                }
                _ => {
                    help = String::from("Unknown command, type help to see a list of commands,
                    and then type help <command> to see more information about a command.");
                }
                //TODO: add more commands
            }
        },
        _ => {
            help = String::from("Too many arguments, type help <command> to see more information about a command.");
        }
    }
    help
}

pub fn exit(){
    print!("\r\x1b[2K");
    println!("\x1b[95mExiting...\x1b[0m");
    println!("Bye!");
    std::process::exit(0);
}