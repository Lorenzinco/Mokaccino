//macros for the communication protocol

pub enum Command {
    //client to server
    Login,
    Whereis(String),
    Ping,
    Logout,
    Logged,
    Hereis(String),
    Pong,
    //Client to client
    Connect(String),
    Message(String),
    Voice(String)
}

pub fn get_op_code(cmd:Command)->u8{
    match cmd{
        Command::Login => 0,
        Command::Connect(_) => 1,
        Command::Message(_) => 2,
        Command::Logout => 3,
        Command::Whereis(_) => 4,
        Command::Ping => 5,
        Command::Pong => 6,
        Command::Logged => 7,
        Command::Hereis(_) => 8,
        Command::Voice(_) => 9
    }
}