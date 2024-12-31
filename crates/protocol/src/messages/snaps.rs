use crate::enums::*;
use packer::{Decoder, DecoderError};
use std::mem::transmute;
use std::slice::from_ref;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub enum SnapTypeId {
    Ordinal(u16),
    Uuid(Uuid),
}

impl From<u16> for SnapTypeId {
    fn from(value: u16) -> Self {
        Self::Ordinal(value)
    }
}

impl From<Uuid> for SnapTypeId {
    fn from(value: Uuid) -> Self {
        Self::Uuid(value)
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tick(pub i32);

pub const PLAYERFLAG_PLAYING: i32 = 1 << 0;
pub const PLAYERFLAG_IN_MENU: i32 = 1 << 1;
pub const PLAYERFLAG_CHATTING: i32 = 1 << 2;
pub const PLAYERFLAG_SCOREBOARD: i32 = 1 << 3;
pub const PLAYERFLAG_AIM: i32 = 1 << 4;

pub const GAMEFLAG_TEAMS: i32 = 1 << 0;
pub const GAMEFLAG_FLAGS: i32 = 1 << 1;

pub const GAMESTATEFLAG_GAMEOVER: i32 = 1 << 0;
pub const GAMESTATEFLAG_SUDDENDEATH: i32 = 1 << 1;
pub const GAMESTATEFLAG_PAUSED: i32 = 1 << 2;
pub const GAMESTATEFLAG_RACETIME: i32 = 1 << 3;

pub const CHARACTERFLAG_SOLO: i32 = 1 << 0;
pub const CHARACTERFLAG_JETPACK: i32 = 1 << 1;
pub const CHARACTERFLAG_COLLISION_DISABLED: i32 = 1 << 2;
pub const CHARACTERFLAG_ENDLESS_HOOK: i32 = 1 << 3;
pub const CHARACTERFLAG_ENDLESS_JUMP: i32 = 1 << 4;
pub const CHARACTERFLAG_SUPER: i32 = 1 << 5;
pub const CHARACTERFLAG_HAMMER_HIT_DISABLED: i32 = 1 << 6;
pub const CHARACTERFLAG_SHOTGUN_HIT_DISABLED: i32 = 1 << 7;
pub const CHARACTERFLAG_GRENADE_HIT_DISABLED: i32 = 1 << 8;
pub const CHARACTERFLAG_LASER_HIT_DISABLED: i32 = 1 << 9;
pub const CHARACTERFLAG_HOOK_HIT_DISABLED: i32 = 1 << 10;
pub const CHARACTERFLAG_TELEGUN_GUN: i32 = 1 << 11;
pub const CHARACTERFLAG_TELEGUN_GRENADE: i32 = 1 << 12;
pub const CHARACTERFLAG_TELEGUN_LASER: i32 = 1 << 13;
pub const CHARACTERFLAG_WEAPON_HAMMER: i32 = 1 << 14;
pub const CHARACTERFLAG_WEAPON_GUN: i32 = 1 << 15;
pub const CHARACTERFLAG_WEAPON_SHOTGUN: i32 = 1 << 16;
pub const CHARACTERFLAG_WEAPON_GRENADE: i32 = 1 << 17;
pub const CHARACTERFLAG_WEAPON_LASER: i32 = 1 << 18;
pub const CHARACTERFLAG_WEAPON_NINJA: i32 = 1 << 19;
pub const CHARACTERFLAG_MOVEMENTS_DISABLED: i32 = 1 << 20;
pub const CHARACTERFLAG_IN_FREEZE: i32 = 1 << 21;
pub const CHARACTERFLAG_PRACTICE_MODE: i32 = 1 << 22;

pub const GAMEINFOFLAG_TIMESCORE: i32 = 1 << 0;
pub const GAMEINFOFLAG_GAMETYPE_RACE: i32 = 1 << 1;
pub const GAMEINFOFLAG_GAMETYPE_FASTCAP: i32 = 1 << 2;
pub const GAMEINFOFLAG_GAMETYPE_FNG: i32 = 1 << 3;
pub const GAMEINFOFLAG_GAMETYPE_DDRACE: i32 = 1 << 4;
pub const GAMEINFOFLAG_GAMETYPE_DDNET: i32 = 1 << 5;
pub const GAMEINFOFLAG_GAMETYPE_BLOCK_WORLDS: i32 = 1 << 6;
pub const GAMEINFOFLAG_GAMETYPE_VANILLA: i32 = 1 << 7;
pub const GAMEINFOFLAG_GAMETYPE_PLUS: i32 = 1 << 8;
pub const GAMEINFOFLAG_FLAG_STARTS_RACE: i32 = 1 << 9;
pub const GAMEINFOFLAG_RACE: i32 = 1 << 10;
pub const GAMEINFOFLAG_UNLIMITED_AMMO: i32 = 1 << 11;
pub const GAMEINFOFLAG_DDRACE_RECORD_MESSAGE: i32 = 1 << 12;
pub const GAMEINFOFLAG_RACE_RECORD_MESSAGE: i32 = 1 << 13;
pub const GAMEINFOFLAG_ALLOW_EYE_WHEEL: i32 = 1 << 14;
pub const GAMEINFOFLAG_ALLOW_HOOK_COLL: i32 = 1 << 15;
pub const GAMEINFOFLAG_ALLOW_ZOOM: i32 = 1 << 16;
pub const GAMEINFOFLAG_BUG_DDRACE_GHOST: i32 = 1 << 17;
pub const GAMEINFOFLAG_BUG_DDRACE_INPUT: i32 = 1 << 18;
pub const GAMEINFOFLAG_BUG_FNG_LASER_RANGE: i32 = 1 << 19;
pub const GAMEINFOFLAG_BUG_VANILLA_BOUNCE: i32 = 1 << 20;
pub const GAMEINFOFLAG_PREDICT_FNG: i32 = 1 << 21;
pub const GAMEINFOFLAG_PREDICT_DDRACE: i32 = 1 << 22;
pub const GAMEINFOFLAG_PREDICT_DDRACE_TILES: i32 = 1 << 23;
pub const GAMEINFOFLAG_PREDICT_VANILLA: i32 = 1 << 24;
pub const GAMEINFOFLAG_ENTITIES_DDNET: i32 = 1 << 25;
pub const GAMEINFOFLAG_ENTITIES_DDRACE: i32 = 1 << 26;
pub const GAMEINFOFLAG_ENTITIES_RACE: i32 = 1 << 27;
pub const GAMEINFOFLAG_ENTITIES_FNG: i32 = 1 << 28;
pub const GAMEINFOFLAG_ENTITIES_VANILLA: i32 = 1 << 29;
pub const GAMEINFOFLAG_DONT_MASK_ENTITIES: i32 = 1 << 30;
pub const GAMEINFOFLAG_ENTITIES_BW: i32 = 1 << 31;

pub const GAMEINFOFLAG2_ALLOW_X_SKINS: i32 = 1 << 0;
pub const GAMEINFOFLAG2_GAMETYPE_CITY: i32 = 1 << 1;
pub const GAMEINFOFLAG2_GAMETYPE_FDDRACE: i32 = 1 << 2;
pub const GAMEINFOFLAG2_ENTITIES_FDDRACE: i32 = 1 << 3;
pub const GAMEINFOFLAG2_HUD_HEALTH_ARMOR: i32 = 1 << 4;
pub const GAMEINFOFLAG2_HUD_AMMO: i32 = 1 << 5;
pub const GAMEINFOFLAG2_HUD_DDRACE: i32 = 1 << 6;
pub const GAMEINFOFLAG2_NO_WEAK_HOOK: i32 = 1 << 7;

pub const EXPLAYERFLAG_AFK: i32 = 1 << 0;
pub const EXPLAYERFLAG_PAUSED: i32 = 1 << 1;
pub const EXPLAYERFLAG_SPEC: i32 = 1 << 2;

pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT0: i32 = 1 << 0;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT1: i32 = 1 << 1;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT2: i32 = 1 << 2;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT3: i32 = 1 << 3;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT4: i32 = 1 << 4;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT5: i32 = 1 << 5;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT6: i32 = 1 << 6;
pub const LEGACYPROJECTILEFLAG_CLIENTID_BIT7: i32 = 1 << 7;
pub const LEGACYPROJECTILEFLAG_NO_OWNER: i32 = 1 << 8;
pub const LEGACYPROJECTILEFLAG_IS_DDNET: i32 = 1 << 9;
pub const LEGACYPROJECTILEFLAG_BOUNCE_HORIZONTAL: i32 = 1 << 10;
pub const LEGACYPROJECTILEFLAG_BOUNCE_VERTICAL: i32 = 1 << 11;
pub const LEGACYPROJECTILEFLAG_EXPLOSIVE: i32 = 1 << 12;
pub const LEGACYPROJECTILEFLAG_FREEZE: i32 = 1 << 13;

pub const PROJECTILEFLAG_BOUNCE_HORIZONTAL: i32 = 1 << 0;
pub const PROJECTILEFLAG_BOUNCE_VERTICAL: i32 = 1 << 1;
pub const PROJECTILEFLAG_EXPLOSIVE: i32 = 1 << 2;
pub const PROJECTILEFLAG_FREEZE: i32 = 1 << 3;
pub const PROJECTILEFLAG_NORMALIZE_VEL: i32 = 1 << 4;

pub const PLAYER_INPUT: u16 = 1;
pub const PROJECTILE: u16 = 2;
pub const LASER: u16 = 3;
pub const PICKUP: u16 = 4;
pub const FLAG: u16 = 5;
pub const GAME_INFO: u16 = 6;
pub const GAME_DATA: u16 = 7;
pub const CHARACTER_CORE: u16 = 8;
pub const CHARACTER: u16 = 9;
pub const PLAYER_INFO: u16 = 10;
pub const CLIENT_INFO: u16 = 11;
pub const SPECTATOR_INFO: u16 = 12;
pub const MY_OWN_OBJECT: Uuid = Uuid::from_u128(0x0dc77a02_bfee_3a53_ac8e_0bb0241bd722);
pub const DDNET_CHARACTER: Uuid = Uuid::from_u128(0x76ce455b_f9eb_3a48_add7_e04b941d045c);
pub const DDNET_PLAYER: Uuid = Uuid::from_u128(0x22ca938d_1380_3e2b_9e7b_d2558ea6be11);
pub const GAME_INFO_EX: Uuid = Uuid::from_u128(0x933dea6a_da79_30ea_a98f_8af03689a945);
pub const DDRACE_PROJECTILE: Uuid = Uuid::from_u128(0x0e6db85c_2b61_386f_bbf2_d0d0471b9272);
pub const DDNET_LASER: Uuid = Uuid::from_u128(0x29de68a2_6928_31b8_8360_a2307e0d844f);
pub const DDNET_PROJECTILE: Uuid = Uuid::from_u128(0x6550fbce_f317_3b31_8ffe_d2b37f3ab40e);
pub const DDNET_PICKUP: Uuid = Uuid::from_u128(0xea5e4a51_58fb_3684_96e4_e0d267f4ca65);
pub const COMMON: u16 = 13;
pub const EXPLOSION: u16 = 14;
pub const SPAWN: u16 = 15;
pub const HAMMER_HIT: u16 = 16;
pub const DEATH: u16 = 17;
pub const SOUND_GLOBAL: u16 = 18;
pub const SOUND_WORLD: u16 = 19;
pub const DAMAGE_IND: u16 = 20;
pub const MY_OWN_EVENT: Uuid = Uuid::from_u128(0x0c4fd27d_47e3_3871_a226_9f417486a311);
pub const SPEC_CHAR: Uuid = Uuid::from_u128(0x4b801c74_e24c_3ce0_b92c_b754d02cfc8a);
pub const SWITCH_STATE: Uuid = Uuid::from_u128(0xec15e669_ce11_3367_ae8e_b90e5b27b9d5);
pub const ENTITY_EX: Uuid = Uuid::from_u128(0x2de9aec3_32e4_3986_8f7e_e7459da7f535);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerInputSnap {
    pub direction: i32,
    pub target_x: i32,
    pub target_y: i32,
    pub jump: i32,
    pub fire: i32,
    pub hook: i32,
    pub player_flags: i32,
    pub wanted_weapon: i32,
    pub next_weapon: i32,
    pub prev_weapon: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub x: i32,
    pub y: i32,
    pub vel_x: i32,
    pub vel_y: i32,
    pub weapon: Weapon,
    pub start_tick: Tick,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Laser {
    pub x: i32,
    pub y: i32,
    pub from_x: i32,
    pub from_y: i32,
    pub start_tick: Tick,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pickup {
    pub x: i32,
    pub y: i32,
    pub type_: i32,
    pub subtype: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FlagSnap {
    pub x: i32,
    pub y: i32,
    pub team: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameInfoSnap {
    pub game_flags: i32,
    pub game_state_flags: i32,
    pub round_start_tick: Tick,
    pub warmup_timer: i32,
    pub score_limit: i32,
    pub time_limit: i32,
    pub round_num: i32,
    pub round_current: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameData {
    pub teamscore_red: i32,
    pub teamscore_blue: i32,
    pub flag_carrier_red: i32,
    pub flag_carrier_blue: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharacterCoreSnap {
    pub tick: i32,
    pub x: i32,
    pub y: i32,
    pub vel_x: i32,
    pub vel_y: i32,
    pub angle: i32,
    pub direction: i32,
    pub jumped: i32,
    pub hooked_player: i32,
    pub hook_state: i32,
    pub hook_tick: Tick,
    pub hook_x: i32,
    pub hook_y: i32,
    pub hook_dx: i32,
    pub hook_dy: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CharacterSnap {
    pub character_core: CharacterCoreSnap,
    pub player_flags: i32,
    pub health: i32,
    pub armor: i32,
    pub ammo_count: i32,
    pub weapon: Weapon,
    pub emote: Emote,
    pub attack_tick: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    pub local: i32,
    pub client_id: i32,
    pub team: Team,
    pub score: i32,
    pub latency: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClientInfoSnap {
    pub name: [i32; 4],
    pub clan: [i32; 3],
    pub country: i32,
    pub skin: [i32; 6],
    pub use_custom_color: i32,
    pub color_body: i32,
    pub color_feet: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpectatorInfo {
    pub spectator_id: i32,
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MyOwnObject {
    pub test: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DDNetCharacterSnap {
    pub flags: i32,
    pub freeze_end: Tick,
    pub jumps: i32,
    pub tele_checkpoint: i32,
    pub strong_weak_id: i32,
    pub jumped_total: i32,
    pub ninja_activation_tick: Tick,
    pub freeze_start: Tick,
    pub target_x: i32,
    pub target_y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DDNetPlayerSnap {
    pub flags: i32,
    pub auth_level: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameInfoEx {
    pub flags: i32,
    pub version: i32,
    pub flags2: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DdraceProjectile {
    pub x: i32,
    pub y: i32,
    pub angle: i32,
    pub data: i32,
    pub type_: Weapon,
    pub start_tick: Tick,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DdnetLaser {
    pub to_x: i32,
    pub to_y: i32,
    pub from_x: i32,
    pub from_y: i32,
    pub start_tick: Tick,
    pub owner: i32,
    pub type_: i32,
    pub switch_number: i32,
    pub subtype: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DdnetProjectile {
    pub x: i32,
    pub y: i32,
    pub vel_x: i32,
    pub vel_y: i32,
    pub type_: Weapon,
    pub start_tick: Tick,
    pub owner: i32,
    pub switch_number: i32,
    pub tune_zone: i32,
    pub flags: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DDNetPickupSnap {
    pub x: i32,
    pub y: i32,
    pub type_: i32,
    pub subtype: i32,
    pub switch_number: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CommonSnap {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Explosion {
    pub common: CommonSnap,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Spawn {
    pub common: CommonSnap,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HammerHitSnap {
    pub common: CommonSnap,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeathSnap {
    pub common: CommonSnap,
    pub client_id: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SoundGlobal {
    pub common: CommonSnap,
    pub sound_id: Sound,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SoundWorld {
    pub common: CommonSnap,
    pub sound_id: Sound,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DamageInd {
    pub common: CommonSnap,
    pub angle: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MyOwnEvent {
    pub test: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpecChar {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SwitchState {
    pub highest_switch_number: i32,
    pub status: [i32; 8],
    pub switch_numbers: [i32; 4],
    pub end_ticks: [i32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntityEx {
    pub switch_number: i32,
    pub layer: i32,
    pub entity_class: i32,
}

impl PlayerInputSnap {
    pub fn decode(decoder: &mut Decoder) -> Result<PlayerInputSnap, DecoderError> {
        Ok(PlayerInputSnap {
            direction: decoder.read_int()?,
            target_x: decoder.read_int()?,
            target_y: decoder.read_int()?,
            jump: decoder.read_int()?,
            fire: decoder.read_int()?,
            hook: decoder.read_int()?,
            player_flags: decoder.read_int()?,
            wanted_weapon: decoder.read_int()?,
            next_weapon: decoder.read_int()?,
            prev_weapon: decoder.read_int()?,
        })
    }
}

impl Projectile {
    pub fn decode(unpacker: &mut Decoder) -> Result<Projectile, DecoderError> {
        Ok(Projectile {
            x: unpacker.read_int()?,
            y: unpacker.read_int()?,
            vel_x: unpacker.read_int()?,
            vel_y: unpacker.read_int()?,
            weapon: Weapon::from_i32(unpacker.read_int()?).ok_or(DecoderError::UnknownId)?,
            start_tick: Tick(unpacker.read_int()?),
        })
    }
}

impl Laser {
    pub fn decode(unpacker: &mut Decoder) -> Result<Laser, DecoderError> {
        Ok(Laser {
            x: unpacker.read_int()?,
            y: unpacker.read_int()?,
            from_x: unpacker.read_int()?,
            from_y: unpacker.read_int()?,
            start_tick: Tick(unpacker.read_int()?),
        })
    }
}

impl Pickup {
    pub fn decode(unpacker: &mut Decoder) -> Result<Pickup, DecoderError> {
        Ok(Pickup {
            x: unpacker.read_int()?,
            y: unpacker.read_int()?,
            type_: unpacker.read_int()?,
            subtype: unpacker.read_int()?,
        })
    }
}

impl FlagSnap {
    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        Ok(Self {
            x: decoder.read_int()?,
            y: decoder.read_int()?,
            team: decoder.read_int()?,
        })
    }
}

impl GameInfoSnap {
    pub fn decode(decoder: &mut Decoder) -> Result<GameInfoSnap, DecoderError> {
        Ok(GameInfoSnap {
            game_flags: decoder.read_int()?,
            game_state_flags: decoder.read_int()?,
            round_start_tick: Tick(decoder.read_int()?),
            warmup_timer: decoder.read_int()?,
            score_limit: decoder.read_int()?,
            time_limit: decoder.read_int()?,
            round_num: decoder.read_int()?,
            round_current: decoder.read_int()?,
        })
    }
}

impl GameData {
    pub fn decode(p: &mut Decoder) -> Result<GameData, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<GameData, DecoderError> {
        Ok(GameData {
            teamscore_red: _p.read_int()?,
            teamscore_blue: _p.read_int()?,
            flag_carrier_red: _p.read_int()?,
            flag_carrier_blue: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl CharacterCoreSnap {
    pub fn decode(_p: &mut Decoder) -> Result<CharacterCoreSnap, DecoderError> {
        Ok(CharacterCoreSnap {
            tick: _p.read_int()?,
            x: _p.read_int()?,
            y: _p.read_int()?,
            vel_x: _p.read_int()?,
            vel_y: _p.read_int()?,
            angle: _p.read_int()?,
            direction: _p.read_int()?,
            jumped: _p.read_int()?,
            hooked_player: _p.read_int()?,
            hook_state: _p.read_int()?,
            hook_tick: Tick(_p.read_int()?),
            hook_x: _p.read_int()?,
            hook_y: _p.read_int()?,
            hook_dx: _p.read_int()?,
            hook_dy: _p.read_int()?,
        })
    }

    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl CharacterSnap {
    pub fn decode(_p: &mut Decoder) -> Result<CharacterSnap, DecoderError> {
        Ok(CharacterSnap {
            character_core: CharacterCoreSnap::decode(_p)?,
            player_flags: _p.read_int()?,
            health: _p.read_int()?,
            armor: _p.read_int()?,
            ammo_count: _p.read_int()?,
            weapon: Weapon::from_i32(_p.read_int()?).ok_or(DecoderError::UnknownId)?,
            emote: Emote::from_i32(_p.read_int()?).ok_or(DecoderError::UnknownId)?,
            attack_tick: _p.read_int()?,
        })
    }

    pub fn encode(&self) -> &[i32] {
        self.character_core.encode();

        unsafe { transmute(from_ref(self)) }
    }
}

impl PlayerInfo {
    pub fn decode(p: &mut Decoder) -> Result<PlayerInfo, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<PlayerInfo, DecoderError> {
        Ok(PlayerInfo {
            local: _p.read_int()?,
            client_id: _p.read_int()?,
            team: Team::from_i32(_p.read_int()?).ok_or(DecoderError::Other)?,
            score: _p.read_int()?,
            latency: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl ClientInfoSnap {
    pub fn decode(p: &mut Decoder) -> Result<ClientInfoSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<ClientInfoSnap, DecoderError> {
        Ok(ClientInfoSnap {
            name: [
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
            ],
            clan: [_p.read_int()?, _p.read_int()?, _p.read_int()?],
            country: _p.read_int()?,
            skin: [
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
            ],
            use_custom_color: _p.read_int()?,
            color_body: _p.read_int()?,
            color_feet: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl SpectatorInfo {
    pub fn decode(p: &mut Decoder) -> Result<SpectatorInfo, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<SpectatorInfo, DecoderError> {
        Ok(SpectatorInfo {
            spectator_id: _p.read_int()?,
            x: _p.read_int()?,
            y: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl DDNetCharacterSnap {
    pub fn decode(p: &mut Decoder) -> Result<DDNetCharacterSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DDNetCharacterSnap, DecoderError> {
        Ok(DDNetCharacterSnap {
            flags: _p.read_int()?,
            freeze_end: Tick(_p.read_int()?),
            jumps: _p.read_int()?,
            tele_checkpoint: _p.read_int()?,
            strong_weak_id: _p.read_int()?,
            jumped_total: _p.read_int()?,
            ninja_activation_tick: Tick(_p.read_int()?),
            freeze_start: Tick(_p.read_int()?),
            target_x: _p.read_int()?,
            target_y: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl DDNetPlayerSnap {
    pub fn decode(p: &mut Decoder) -> Result<DDNetPlayerSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DDNetPlayerSnap, DecoderError> {
        Ok(DDNetPlayerSnap {
            flags: _p.read_int()?,
            auth_level: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl GameInfoEx {
    pub fn decode(p: &mut Decoder) -> Result<GameInfoEx, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<GameInfoEx, DecoderError> {
        Ok(GameInfoEx {
            flags: _p.read_int()?,
            version: _p.read_int()?,
            flags2: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl DdraceProjectile {
    pub fn decode(p: &mut Decoder) -> Result<DdraceProjectile, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DdraceProjectile, DecoderError> {
        Ok(DdraceProjectile {
            x: _p.read_int()?,
            y: _p.read_int()?,
            angle: _p.read_int()?,
            data: _p.read_int()?,
            type_: Weapon::from_i32(_p.read_int()?).ok_or(DecoderError::Other)?,
            start_tick: Tick(_p.read_int()?),
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl DdnetLaser {
    pub fn decode(p: &mut Decoder) -> Result<DdnetLaser, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DdnetLaser, DecoderError> {
        Ok(DdnetLaser {
            to_x: _p.read_int()?,
            to_y: _p.read_int()?,
            from_x: _p.read_int()?,
            from_y: _p.read_int()?,
            start_tick: Tick(_p.read_int()?),
            owner: _p.read_int()?,
            type_: _p.read_int()?,
            switch_number: _p.read_int()?,
            subtype: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl DdnetProjectile {
    pub fn decode(p: &mut Decoder) -> Result<DdnetProjectile, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DdnetProjectile, DecoderError> {
        Ok(DdnetProjectile {
            x: _p.read_int()?,
            y: _p.read_int()?,
            vel_x: _p.read_int()?,
            vel_y: _p.read_int()?,
            type_: Weapon::from_i32(_p.read_int()?).ok_or(DecoderError::Other)?,
            start_tick: Tick(_p.read_int()?),
            owner: _p.read_int()?,
            switch_number: _p.read_int()?,
            tune_zone: _p.read_int()?,
            flags: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}
impl DDNetPickupSnap {
    pub fn decode(p: &mut Decoder) -> Result<DDNetPickupSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DDNetPickupSnap, DecoderError> {
        Ok(DDNetPickupSnap {
            x: _p.read_int()?,
            y: _p.read_int()?,
            type_: _p.read_int()?,
            subtype: _p.read_int()?,
            switch_number: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl CommonSnap {
    pub fn decode(p: &mut Decoder) -> Result<CommonSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<CommonSnap, DecoderError> {
        Ok(CommonSnap {
            x: _p.read_int()?,
            y: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl Explosion {
    pub fn decode(p: &mut Decoder) -> Result<Explosion, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<Explosion, DecoderError> {
        Ok(Explosion {
            common: CommonSnap::decode_inner(_p)?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl Spawn {
    pub fn decode(p: &mut Decoder) -> Result<Spawn, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<Spawn, DecoderError> {
        Ok(Spawn {
            common: CommonSnap::decode_inner(_p)?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl HammerHitSnap {
    pub fn decode(p: &mut Decoder) -> Result<HammerHitSnap, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<HammerHitSnap, DecoderError> {
        Ok(HammerHitSnap {
            common: CommonSnap::decode_inner(_p)?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl DeathSnap {
    pub fn decode(decoder: &mut Decoder) -> Result<DeathSnap, DecoderError> {
        Ok(DeathSnap {
            common: CommonSnap::decode_inner(decoder)?,
            client_id: decoder.read_int()?,
        })
    }

    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl SoundGlobal {
    pub fn decode(p: &mut Decoder) -> Result<SoundGlobal, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<SoundGlobal, DecoderError> {
        Ok(SoundGlobal {
            common: CommonSnap::decode_inner(_p)?,
            sound_id: Sound::from_i32(_p.read_int()?).ok_or(DecoderError::Other)?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl SoundWorld {
    pub fn decode(p: &mut Decoder) -> Result<SoundWorld, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }

    pub fn decode_inner(_p: &mut Decoder) -> Result<SoundWorld, DecoderError> {
        Ok(SoundWorld {
            common: CommonSnap::decode_inner(_p)?,
            sound_id: Sound::from_i32(_p.read_int()?).ok_or(DecoderError::OutOfBounds)?,
        })
    }

    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl DamageInd {
    pub fn decode(p: &mut Decoder) -> Result<DamageInd, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<DamageInd, DecoderError> {
        Ok(DamageInd {
            common: CommonSnap::decode_inner(_p)?,
            angle: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        self.common.encode();
        unsafe { transmute(from_ref(self)) }
    }
}

impl MyOwnEvent {
    pub fn decode(p: &mut Decoder) -> Result<MyOwnEvent, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<MyOwnEvent, DecoderError> {
        Ok(MyOwnEvent {
            test: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl SpecChar {
    pub fn decode(p: &mut Decoder) -> Result<SpecChar, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<SpecChar, DecoderError> {
        Ok(SpecChar {
            x: _p.read_int()?,
            y: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl SwitchState {
    pub fn decode(p: &mut Decoder) -> Result<SwitchState, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<SwitchState, DecoderError> {
        Ok(SwitchState {
            highest_switch_number: _p.read_int()?,
            status: [
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
            ],
            switch_numbers: [
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
            ],
            end_ticks: [
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
                _p.read_int()?,
            ],
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl EntityEx {
    pub fn decode(p: &mut Decoder) -> Result<EntityEx, DecoderError> {
        let result = Self::decode_inner(p)?;

        Ok(result)
    }
    pub fn decode_inner(_p: &mut Decoder) -> Result<EntityEx, DecoderError> {
        Ok(EntityEx {
            switch_number: _p.read_int()?,
            layer: _p.read_int()?,
            entity_class: _p.read_int()?,
        })
    }
    pub fn encode(&self) -> &[i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

pub fn get_object_size(type_: u16) -> Option<u32> {
    Some(match type_ {
        PLAYER_INPUT => 10,
        PROJECTILE => 6,
        LASER => 5,
        PICKUP => 4,
        FLAG => 3,
        GAME_INFO => 8,
        GAME_DATA => 4,
        CHARACTER_CORE => 15,
        CHARACTER => 22,
        PLAYER_INFO => 5,
        CLIENT_INFO => 17,
        SPECTATOR_INFO => 3,
        COMMON => 2,
        EXPLOSION => 2,
        SPAWN => 2,
        HAMMER_HIT => 2,
        DEATH => 3,
        SOUND_GLOBAL => 3,
        SOUND_WORLD => 3,
        DAMAGE_IND => 3,
        _ => return None,
    })
}
