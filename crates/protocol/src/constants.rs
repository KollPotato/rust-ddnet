pub const MAX_CLIENTS: i32 = 16;
pub const SPEC_FREEVIEW: i32 = -1;
pub const FLAG_MISSING: i32 = -3;
pub const FLAG_ATSTAND: i32 = -2;
pub const FLAG_TAKEN: i32 = -1;
pub const VERSION: &'static str = "0.6 626fce9a778df4d4";
pub const DDNET_VERSION: i32 = 17021;

pub mod emote {
    pub const NORMAL: i32 = 0;
    pub const PAIN: i32 = 1;
    pub const HAPPY: i32 = 2;
    pub const SURPRISE: i32 = 3;
    pub const ANGRY: i32 = 4;
    pub const BLINK: i32 = 5;
}

pub mod emoticon {
    pub const OOP: i32 = 0;
    pub const EXCLAMATION: i32 = 1;
    pub const HEARTS: i32 = 2;
    pub const DROP: i32 = 3;
    pub const DOTDOT: i32 = 4;
    pub const MUSIC: i32 = 5;
    pub const SORRY: i32 = 6;
    pub const GHOST: i32 = 7;
    pub const SUSHI: i32 = 8;
    pub const SPLATTEE: i32 = 9;
    pub const DEVILTEE: i32 = 10;
    pub const ZOMG: i32 = 11;
    pub const ZZZ: i32 = 12;
    pub const WTF: i32 = 13;
    pub const EYES: i32 = 14;
    pub const QUESTION: i32 = 15;
}

pub mod powerup {
    pub const HEALTH: i32 = 0;
    pub const ARMOR: i32 = 1;
    pub const WEAPON: i32 = 2;
    pub const NINJA: i32 = 3;
    pub const ARMOR_SHOTGUN: i32 = 4;
    pub const ARMOR_GRENADE: i32 = 5;
    pub const ARMOR_NINJA: i32 = 6;
    pub const ARMOR_LASER: i32 = 7;
}

pub mod authed {
    pub const NO: i32 = 0;
    pub const HELPER: i32 = 1;
    pub const MOD: i32 = 2;
    pub const ADMIN: i32 = 3;
}

pub mod weapon {
    pub const HAMMER: i32 = 0;
    pub const PISTOL: i32 = 1;
    pub const SHOTGUN: i32 = 2;
    pub const GRENADE: i32 = 3;
    pub const RIFLE: i32 = 4;
    pub const NINJA: i32 = 5;
}

pub mod connless {
    pub const INFO_FLAG_PASSWORD: i32 = 1;

    pub const REQUEST_LIST: [u8; 8] = *b"\xff\xff\xff\xffreq2";
    pub const LIST: [u8; 8] = *b"\xff\xff\xff\xfflis2";
    pub const REQUEST_COUNT: [u8; 8] = *b"\xff\xff\xff\xffcou2";
    pub const COUNT: [u8; 8] = *b"\xff\xff\xff\xffsiz2";
    pub const REQUEST_INFO: [u8; 8] = *b"\xff\xff\xff\xffgie3";
    pub const INFO: [u8; 8] = *b"\xff\xff\xff\xffinf3";
    pub const INFO_EXTENDED: [u8; 8] = *b"\xff\xff\xff\xffiext";
    pub const INFO_EXTENDED_MORE: [u8; 8] = *b"\xff\xff\xff\xffiex+";
    pub const HEARTBEAT: [u8; 8] = *b"\xff\xff\xff\xffbea2";
    pub const FORWARD_CHECK: [u8; 8] = *b"\xff\xff\xff\xfffw??";
    pub const FORWARD_RESPONSE: [u8; 8] = *b"\xff\xff\xff\xfffw!!";
    pub const FORWARD_OK: [u8; 8] = *b"\xff\xff\xff\xfffwok";
    pub const FORWARD_ERROR: [u8; 8] = *b"\xff\xff\xff\xfffwer";
}

pub mod game {
    use uuid::Uuid;

    pub const SV_MOTD: u8 = 1;
    pub const SV_BROADCAST: u8 = 2;
    pub const SV_CHAT: u8 = 3;
    pub const SV_KILL_MESSAGE: u8 = 4;
    pub const SV_SOUND_GLOBAL: u8 = 5;
    pub const SV_TUNE_PARAMS: u8 = 6;
    pub const SV_EXTRA_PROJECTILE: u8 = 7;
    pub const SV_READY_TO_ENTER: u8 = 8;
    pub const SV_WEAPON_PICKUP: u8 = 9;
    pub const SV_EMOTICON: u8 = 10;
    pub const SV_VOTE_CLEAR_OPTIONS: u8 = 11;
    pub const SV_VOTE_OPTION_LIST_ADD: u8 = 12;
    pub const SV_VOTE_OPTION_ADD: u8 = 13;
    pub const SV_VOTE_OPTION_REMOVE: u8 = 14;
    pub const SV_VOTE_SET: u8 = 15;
    pub const SV_VOTE_STATUS: u8 = 16;
    pub const CL_SAY: u8 = 17;
    pub const CL_SET_TEAM: u8 = 18;
    pub const CL_SET_SPECTATOR_MODE: u8 = 19;
    pub const CL_START_INFO: u8 = 20;
    pub const CL_CHANGE_INFO: u8 = 21;
    pub const CL_KILL: u8 = 22;
    pub const CL_EMOTICON: u8 = 23;
    pub const CL_VOTE: u8 = 24;
    pub const CL_CALL_VOTE: u8 = 25;

    pub const CL_IS_DDNET_LEGACY: i32 = 26;
    pub const SV_DDRACE_TIME_LEGACY: i32 = 27;
    pub const SV_RECORD_LEGACY: i32 = 28;

    pub const SV_TEAMS_STATE_LEGACY: i32 = 30;
    pub const CL_SHOW_OTHERS_LEGACY: i32 = 31;
    pub const SV_MY_OWN_MESSAGE: Uuid = Uuid::from_u128(0x1231e484_f607_3722_a89a_bd85db46f5d2);
    pub const CL_SHOW_DISTANCE: Uuid = Uuid::from_u128(0x53bb28af_4252_3ac9_8fd3_6ccbc2a603e3);
    pub const CL_SHOW_OTHERS: Uuid = Uuid::from_u128(0x7f264cdd_71a2_3962_bbce_0f94bbd81913);
    pub const SV_TEAMS_STATE: Uuid = Uuid::from_u128(0xa091961a_95e8_3744_bb60_5eac9bd563c6);
    pub const SV_DDRACE_TIME: Uuid = Uuid::from_u128(0x5dde8b3c_6f6f_37ac_a72a_bb341fe76de5);
    pub const SV_RECORD: Uuid = Uuid::from_u128(0x804f149f_9b53_3b0a_897f_59663a1c4eb9);
    pub const SV_KILL_MSG_TEAM: Uuid = Uuid::from_u128(0xee610b6f_909f_311e_93f7_11a95f55a086);
}

pub mod control {
    pub const KEEP_ALIVE: u8 = 0;
    pub const CONNECT: u8 = 1;
    pub const CONNECT_ACCEPT: u8 = 2;
    pub const ACCEPT: u8 = 3;
    pub const CLOSE: u8 = 4;
}

pub mod system {
    use uuid::Uuid;

    pub const INFO: u8 = 1;
    pub const MAP_CHANGE: u8 = 2;
    pub const MAP_DATA: u8 = 3;
    pub const CON_READY: u8 = 4;
    pub const SNAP: u8 = 5;
    pub const EMPTY_SNAP: u8 = 6;
    pub const SINGLE_SNAP: u8 = 7;
    pub const SMALL_SNAP: u8 = 8;
    pub const INPUT_TIMING: u8 = 9;
    pub const RCON_AUTH_STATUS: u8 = 10;
    pub const RCON_LINE: u8 = 11;
    pub const AUTH_CHALLANGE: u8 = 12;
    pub const AUTH_RESULT: u8 = 13;
    pub const READY: u8 = 14;
    pub const ENTER_GAME: u8 = 15;
    pub const INPUT: u8 = 16;
    pub const RCON_CMD: u8 = 17;
    pub const RCON_AUTH: u8 = 18;
    pub const REQUEST_MAP_DATA: u8 = 19;
    pub const AUTH_START: u8 = 20;
    pub const AUTH_RESPONSE: u8 = 21;
    pub const PING: u8 = 22;
    pub const PING_REPLY: u8 = 23;
    pub const ERROR: u8 = 24;
    pub const RCON_COMMAND_ADD: u8 = 25;
    pub const RCON_COMMAND_REMOVE: u8 = 26;

    pub const WHAT_IS: Uuid = Uuid::from_u128(0x245e5097_9fe0_39d6_bf7d_9a29e1691e4c);
    pub const IT_IS: Uuid = Uuid::from_u128(0x6954847e_2e87_3603_b562_36da29ed1aca);
    pub const I_DONT_KNOW: Uuid = Uuid::from_u128(0x416911b5_7973_33bf_8d52_7bf01e519cf0);
    pub const RCON_TYPE: Uuid = Uuid::from_u128(0x12810e1f_a1db_3378_b4fb_164ed6505926);
    pub const MAP_DETAILS: Uuid = Uuid::from_u128(0xf9117b3c_8039_3416_9fc0_aef2bcb75c03);
    pub const CAPABILITIES: Uuid = Uuid::from_u128(0xf621a5a1_f585_3775_8e73_41beee79f2b2);
    pub const CLIENT_VERSION: Uuid = Uuid::from_u128(0x8c001304_8461_3e47_8787_f672b3835bd4);
    pub const PING_EX: Uuid = Uuid::from_u128(0xbcb43bf5_427c_36d8_b5b8_7975c8c06aa1);
    pub const PONG_EX: Uuid = Uuid::from_u128(0xd8295530_14a7_3a0a_b02e_b2cee08d2033);
    pub const CHECKSUM_REQUEST: Uuid = Uuid::from_u128(0x60a7cef1_2ecc_3ed4_b138_00fd0c8f5994);
    pub const CHECKSUM_RESPONSE: Uuid = Uuid::from_u128(0x88fc61ec_5a3c_3fc3_8dfa_fd3b715db9e0);
    pub const CHECKSUM_ERROR: Uuid = Uuid::from_u128(0x090960d1_4000_3fd5_9670_4976ae702a6a);
}

pub const CHUNK_HEADER_SIZE: usize = 2;
pub const CHUNK_HEADER_SIZE_VITAL: usize = 3;
pub const HEADER_SIZE: usize = 3;
pub const MAX_PACKETSIZE: usize = 1400;
pub const PADDING_SIZE_CONNLESS: usize = 3;
pub const TOKEN_SIZE: usize = 4;

pub const MAX_PAYLOAD: usize = 1390;

pub const PACKET_FLAG_CONTROL: u8 = 1 << 0;
pub const PACKET_FLAG_CONNLESS: u8 = 1 << 1;
pub const PACKET_FLAG_REQUEST_RESEND: u8 = 1 << 2;
pub const PACKET_FLAG_COMPRESSION: u8 = 1 << 3;

pub const CHUNK_FLAG_RESEND: u8 = 1 << 1;
pub const CHUNK_FLAG_VITAL: u8 = 1 << 0;

pub const CTRLMSG_CLOSE_REASON_LENGTH: usize = 127;
pub const CTRLMSG_TOKEN_MAGIC: &[u8; 4] = b"TKEN";
pub const CHUNK_FLAGS_BITS: u32 = 2;
pub const CHUNK_SIZE_BITS: u32 = 10;
pub const PACKET_FLAGS_BITS: u32 = 4;
pub const SEQUENCE_BITS: u32 = 10;
pub const SEQUENCE_MODULUS: u16 = 1 << SEQUENCE_BITS;
