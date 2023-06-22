pub const PACKET_MAX_LENGTH: usize = 1472;


pub const SERVER_TIMEOUT: u8 = 30;
pub const CLIENT_TIMEOUT: u8 = 15;
pub const PING_INTERVAL: u8 = 3;


// °º¤ø,¸¸,ø¤º°`°º¤ø COMMANDS ø¤°º¤ø,¸¸,ø¤º°`°º¤ø,¸
pub const OPCODE_PING: u8 = 0;
// CLIENT -> SERVER
pub const OPCODE_LOGIN: u8 = 10;
pub const OPCODE_LOGOUT: u8 = 11;
pub const OPCODE_WHEREIS: u8 = 12; // whereis <username>
// SERVER -> CLIENT
pub const OPCODE_LOGGED: u8 = 20;
pub const OPCODE_NOTFOUND: u8 = 21;
pub const OPCODE_HEREIS: u8 = 22; // hereis <username>@<ip>:<port>
// CLIENT <-> CLIENT
pub const OPCODE_CONNECT: u8 = 30; // connect <my_username>
pub const OPCODE_ACCEPT: u8 = 31; // accept <your_username>
pub const OPCODE_DISCONNECT: u8 = 32; // disconnect <my_username>
pub const OPCODE_MESSAGE: u8 = 33; // message <text>
pub const OPCODE_VOICE: u8 = 34; // voice <audio>