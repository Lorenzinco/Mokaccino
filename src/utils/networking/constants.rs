static PACKET_MAX_LENGTH: u16 = 4096;

static SERVER_TIMEOUT: u8 = 30;
static CLIENT_TIMEOUT: u8 = 15;
static PING_INTERVAL: u8 = 4;

// *-*-*-* COMMANDS *-*-*-*
static OPCODE_PING: u8 = 0;
// CLIENT -> SERVER
static OPCODE_LOGIN: u8 = 10;
static OPCODE_LOGOUT: u8 = 11;
static OPCODE_WHEREIS: u8 = 12;
// SERVER -> CLIENT
static OPCODE_LOGGED: u8 = 20;
static OPCODE_HEREIS: u8 = 21;
// CLIENT <-> CLIENT
static OPCODE_CONNECT: u8 = 30;
static OPCODE_ACCEPT: u8 = 31;
static OPCODE_DISCONNECT: u8 = 32;
static OPCODE_MESSAGE: u8 = 33;
static OPCODE_VOICE: u8 = 34;