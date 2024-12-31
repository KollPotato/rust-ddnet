#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Emote {
    Normal,
    Pain,
    Happy,
    Surprise,
    Angry,
    Blink,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Powerup {
    Health,
    Armor,
    Weapon,
    Ninja,
    ArmorShotgun,
    ArmorGrenade,
    ArmorNinja,
    ArmorLaser,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Emoticon {
    Oop,
    Exclamation,
    Hearts,
    Drop,
    Dotdot,
    Music,
    Sorry,
    Ghost,
    Sushi,
    Splattee,
    Deviltee,
    Zomg,
    Zzz,
    Wtf,
    Eyes,
    Question,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Authed {
    No,
    Helper,
    Mod,
    Admin,
}

pub const ENTITYCLASS_PROJECTILE: i32 = 0;
pub const ENTITYCLASS_DOOR: i32 = 1;
pub const ENTITYCLASS_DRAGGER_WEAK: i32 = 2;
pub const ENTITYCLASS_DRAGGER_NORMAL: i32 = 3;
pub const ENTITYCLASS_DRAGGER_STRONG: i32 = 4;
pub const ENTITYCLASS_GUN_NORMAL: i32 = 5;
pub const ENTITYCLASS_GUN_EXPLOSIVE: i32 = 6;
pub const ENTITYCLASS_GUN_FREEZE: i32 = 7;
pub const ENTITYCLASS_GUN_UNFREEZE: i32 = 8;
pub const ENTITYCLASS_LIGHT: i32 = 9;
pub const ENTITYCLASS_PICKUP: i32 = 10;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum EntityClass {
    Projectile,
    Door,
    DraggerWeak,
    DraggerNormal,
    DraggerStrong,
    GunNormal,
    GunExplosive,
    GunFreeze,
    GunUnfreeze,
    Light,
    Pickup,
}

pub const LASERTYPE_RIFLE: i32 = 0;
pub const LASERTYPE_SHOTGUN: i32 = 1;
pub const LASERTYPE_DOOR: i32 = 2;
pub const LASERTYPE_FREEZE: i32 = 3;
pub const LASERTYPE_DRAGGER: i32 = 4;
pub const LASERTYPE_GUN: i32 = 5;
pub const LASERTYPE_PLASMA: i32 = 6;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum LaserType {
    Rifle,
    Shotgun,
    Door,
    Freeze,
    Dragger,
    Gun,
    Plasma,
}

pub const LASER_DRAGGER_TYPE_WEAK: i32 = 0;
pub const LASER_DRAGGER_TYPE_WEAK_NW: i32 = 1;
pub const LASER_DRAGGER_TYPE_NORMAL: i32 = 2;
pub const LASERDRAGGERTYPE_NORMAL_NW: i32 = 3;
pub const LASERDRAGGERTYPE_STRONG: i32 = 4;
pub const LASERDRAGGERTYPE_STRONG_NW: i32 = 5;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum LaserDraggerType {
    Weak,
    WeakNw,
    Normal,
    NormalNw,
    Strong,
    StrongNw,
}

pub const LASERGUNTYPE_UNFREEZE: i32 = 0;
pub const LASERGUNTYPE_EXPLOSIVE: i32 = 1;
pub const LASERGUNTYPE_FREEZE: i32 = 2;
pub const LASERGUNTYPE_EXPFREEZE: i32 = 3;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum LaserGunType {
    Unfreeze,
    Explosive,
    Freeze,
    Expfreeze,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Weapon {
    Hammer,
    Pistol,
    Shotgun,
    Grenade,
    Rifle,
    Ninja,
}

pub const TEAM_SPECTATORS: i32 = -1;
pub const TEAM_RED: i32 = 0;
pub const TEAM_BLUE: i32 = 1;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Team {
    Spectators = -1,
    Red,
    Blue,
}

pub const SOUND_GUN_FIRE: i32 = 0;
pub const SOUND_SHOTGUN_FIRE: i32 = 1;
pub const SOUND_GRENADE_FIRE: i32 = 2;
pub const SOUND_HAMMER_FIRE: i32 = 3;
pub const SOUND_HAMMER_HIT: i32 = 4;
pub const SOUND_NINJA_FIRE: i32 = 5;
pub const SOUND_GRENADE_EXPLODE: i32 = 6;
pub const SOUND_NINJA_HIT: i32 = 7;
pub const SOUND_RIFLE_FIRE: i32 = 8;
pub const SOUND_RIFLE_BOUNCE: i32 = 9;
pub const SOUND_WEAPON_SWITCH: i32 = 10;
pub const SOUND_PLAYER_PAIN_SHORT: i32 = 11;
pub const SOUND_PLAYER_PAIN_LONG: i32 = 12;
pub const SOUND_BODY_LAND: i32 = 13;
pub const SOUND_PLAYER_AIRJUMP: i32 = 14;
pub const SOUND_PLAYER_JUMP: i32 = 15;
pub const SOUND_PLAYER_DIE: i32 = 16;
pub const SOUND_PLAYER_SPAWN: i32 = 17;
pub const SOUND_PLAYER_SKID: i32 = 18;
pub const SOUND_TEE_CRY: i32 = 19;
pub const SOUND_HOOK_LOOP: i32 = 20;
pub const SOUND_HOOK_ATTACH_GROUND: i32 = 21;
pub const SOUND_HOOK_ATTACH_PLAYER: i32 = 22;
pub const SOUND_HOOK_NOATTACH: i32 = 23;
pub const SOUND_PICKUP_HEALTH: i32 = 24;
pub const SOUND_PICKUP_ARMOR: i32 = 25;
pub const SOUND_PICKUP_GRENADE: i32 = 26;
pub const SOUND_PICKUP_SHOTGUN: i32 = 27;
pub const SOUND_PICKUP_NINJA: i32 = 28;
pub const SOUND_WEAPON_SPAWN: i32 = 29;
pub const SOUND_WEAPON_NOAMMO: i32 = 30;
pub const SOUND_HIT: i32 = 31;
pub const SOUND_CHAT_SERVER: i32 = 32;
pub const SOUND_CHAT_CLIENT: i32 = 33;
pub const SOUND_CHAT_HIGHLIGHT: i32 = 34;
pub const SOUND_CTF_DROP: i32 = 35;
pub const SOUND_CTF_RETURN: i32 = 36;
pub const SOUND_CTF_GRAB_PL: i32 = 37;
pub const SOUND_CTF_GRAB_EN: i32 = 38;
pub const SOUND_CTF_CAPTURE: i32 = 39;
pub const SOUND_MENU: i32 = 40;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub enum Sound {
    GunFire,
    ShotgunFire,
    GrenadeFire,
    HammerFire,
    HammerHit,
    NinjaFire,
    GrenadeExplode,
    NinjaHit,
    RifleFire,
    RifleBounce,
    WeaponSwitch,
    PlayerPainShort,
    PlayerPainLong,
    BodyLand,
    PlayerAirjump,
    PlayerJump,
    PlayerDie,
    PlayerSpawn,
    PlayerSkid,
    TeeCry,
    HookLoop,
    HookAttachGround,
    HookAttachPlayer,
    HookNoattach,
    PickupHealth,
    PickupArmor,
    PickupGrenade,
    PickupShotgun,
    PickupNinja,
    WeaponSpawn,
    WeaponNoammo,
    Hit,
    ChatServer,
    ChatClient,
    ChatHighlight,
    CtfDrop,
    CtfReturn,
    CtfGrabPl,
    CtfGrabEn,
    CtfCapture,
    Menu,
}

impl Emote {
    pub fn from_i32(i: i32) -> Option<Emote> {
        use crate::constants::emote::*;
        use Emote::*;

        Some(match i {
            NORMAL => Normal,
            PAIN => Pain,
            HAPPY => Happy,
            SURPRISE => Surprise,
            ANGRY => Angry,
            BLINK => Blink,
            _ => return None,
        })
    }

    pub fn to_i32(self) -> i32 {
        use crate::constants::emote::*;
        use Emote::*;

        match self {
            Normal => NORMAL,
            Pain => PAIN,
            Happy => HAPPY,
            Surprise => SURPRISE,
            Angry => ANGRY,
            Blink => BLINK,
        }
    }
}

impl Powerup {
    pub fn from_i32(i: i32) -> Option<Powerup> {
        use crate::constants::powerup::*;
        use Powerup::*;

        Some(match i {
            HEALTH => Health,
            ARMOR => Armor,
            WEAPON => Weapon,
            NINJA => Ninja,
            ARMOR_SHOTGUN => ArmorShotgun,
            ARMOR_GRENADE => ArmorGrenade,
            ARMOR_NINJA => ArmorNinja,
            ARMOR_LASER => ArmorLaser,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use crate::constants::powerup::*;
        use Powerup::*;
        match self {
            Health => HEALTH,
            Armor => ARMOR,
            Weapon => WEAPON,
            Ninja => NINJA,
            ArmorShotgun => ARMOR_SHOTGUN,
            ArmorGrenade => ARMOR_GRENADE,
            ArmorNinja => ARMOR_NINJA,
            ArmorLaser => ARMOR_LASER,
        }
    }
}

impl Emoticon {
    pub fn from_i32(i: i32) -> Option<Emoticon> {
        use self::Emoticon::*;
        use crate::constants::emoticon::*;

        Some(match i {
            OOP => Oop,
            EXCLAMATION => Exclamation,
            HEARTS => Hearts,
            DROP => Drop,
            DOTDOT => Dotdot,
            MUSIC => Music,
            SORRY => Sorry,
            GHOST => Ghost,
            SUSHI => Sushi,
            SPLATTEE => Splattee,
            DEVILTEE => Deviltee,
            ZOMG => Zomg,
            ZZZ => Zzz,
            WTF => Wtf,
            EYES => Eyes,
            QUESTION => Question,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::Emoticon::*;
        use crate::constants::emoticon::*;

        match self {
            Oop => OOP,
            Exclamation => EXCLAMATION,
            Hearts => HEARTS,
            Drop => DROP,
            Dotdot => DOTDOT,
            Music => MUSIC,
            Sorry => SORRY,
            Ghost => GHOST,
            Sushi => SUSHI,
            Splattee => SPLATTEE,
            Deviltee => DEVILTEE,
            Zomg => ZOMG,
            Zzz => ZZZ,
            Wtf => WTF,
            Eyes => EYES,
            Question => QUESTION,
        }
    }
}

impl Authed {
    pub fn from_i32(i: i32) -> Option<Authed> {
        use self::Authed::*;
        use crate::constants::authed::*;
        Some(match i {
            NO => No,
            HELPER => Helper,
            MOD => Mod,
            ADMIN => Admin,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::Authed::*;
        use crate::constants::authed::*;
        match self {
            No => NO,
            Helper => HELPER,
            Mod => MOD,
            Admin => ADMIN,
        }
    }
}

impl EntityClass {
    pub fn from_i32(i: i32) -> Option<EntityClass> {
        use self::EntityClass::*;
        Some(match i {
            ENTITYCLASS_PROJECTILE => Projectile,
            ENTITYCLASS_DOOR => Door,
            ENTITYCLASS_DRAGGER_WEAK => DraggerWeak,
            ENTITYCLASS_DRAGGER_NORMAL => DraggerNormal,
            ENTITYCLASS_DRAGGER_STRONG => DraggerStrong,
            ENTITYCLASS_GUN_NORMAL => GunNormal,
            ENTITYCLASS_GUN_EXPLOSIVE => GunExplosive,
            ENTITYCLASS_GUN_FREEZE => GunFreeze,
            ENTITYCLASS_GUN_UNFREEZE => GunUnfreeze,
            ENTITYCLASS_LIGHT => Light,
            ENTITYCLASS_PICKUP => Pickup,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::EntityClass::*;
        match self {
            Projectile => ENTITYCLASS_PROJECTILE,
            Door => ENTITYCLASS_DOOR,
            DraggerWeak => ENTITYCLASS_DRAGGER_WEAK,
            DraggerNormal => ENTITYCLASS_DRAGGER_NORMAL,
            DraggerStrong => ENTITYCLASS_DRAGGER_STRONG,
            GunNormal => ENTITYCLASS_GUN_NORMAL,
            GunExplosive => ENTITYCLASS_GUN_EXPLOSIVE,
            GunFreeze => ENTITYCLASS_GUN_FREEZE,
            GunUnfreeze => ENTITYCLASS_GUN_UNFREEZE,
            Light => ENTITYCLASS_LIGHT,
            Pickup => ENTITYCLASS_PICKUP,
        }
    }
}

impl LaserType {
    pub fn from_i32(i: i32) -> Option<LaserType> {
        use self::LaserType::*;
        Some(match i {
            LASERTYPE_RIFLE => Rifle,
            LASERTYPE_SHOTGUN => Shotgun,
            LASERTYPE_DOOR => Door,
            LASERTYPE_FREEZE => Freeze,
            LASERTYPE_DRAGGER => Dragger,
            LASERTYPE_GUN => Gun,
            LASERTYPE_PLASMA => Plasma,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::LaserType::*;
        match self {
            Rifle => LASERTYPE_RIFLE,
            Shotgun => LASERTYPE_SHOTGUN,
            Door => LASERTYPE_DOOR,
            Freeze => LASERTYPE_FREEZE,
            Dragger => LASERTYPE_DRAGGER,
            Gun => LASERTYPE_GUN,
            Plasma => LASERTYPE_PLASMA,
        }
    }
}

impl LaserDraggerType {
    pub fn from_i32(i: i32) -> Option<LaserDraggerType> {
        use self::LaserDraggerType::*;
        Some(match i {
            LASER_DRAGGER_TYPE_WEAK => Weak,
            LASER_DRAGGER_TYPE_WEAK_NW => WeakNw,
            LASER_DRAGGER_TYPE_NORMAL => Normal,
            LASERDRAGGERTYPE_NORMAL_NW => NormalNw,
            LASERDRAGGERTYPE_STRONG => Strong,
            LASERDRAGGERTYPE_STRONG_NW => StrongNw,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::LaserDraggerType::*;
        match self {
            Weak => LASER_DRAGGER_TYPE_WEAK,
            WeakNw => LASER_DRAGGER_TYPE_WEAK_NW,
            Normal => LASER_DRAGGER_TYPE_NORMAL,
            NormalNw => LASERDRAGGERTYPE_NORMAL_NW,
            Strong => LASERDRAGGERTYPE_STRONG,
            StrongNw => LASERDRAGGERTYPE_STRONG_NW,
        }
    }
}

impl LaserGunType {
    pub fn from_i32(i: i32) -> Option<LaserGunType> {
        use self::LaserGunType::*;
        Some(match i {
            LASERGUNTYPE_UNFREEZE => Unfreeze,
            LASERGUNTYPE_EXPLOSIVE => Explosive,
            LASERGUNTYPE_FREEZE => Freeze,
            LASERGUNTYPE_EXPFREEZE => Expfreeze,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::LaserGunType::*;
        match self {
            Unfreeze => LASERGUNTYPE_UNFREEZE,
            Explosive => LASERGUNTYPE_EXPLOSIVE,
            Freeze => LASERGUNTYPE_FREEZE,
            Expfreeze => LASERGUNTYPE_EXPFREEZE,
        }
    }
}

impl Weapon {
    pub fn from_i32(i: i32) -> Option<Weapon> {
        use self::Weapon::*;
        use crate::constants::weapon::*;
        Some(match i {
            HAMMER => Hammer,
            PISTOL => Pistol,
            SHOTGUN => Shotgun,
            GRENADE => Grenade,
            RIFLE => Rifle,
            NINJA => Ninja,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::Weapon::*;
        use crate::constants::weapon::*;

        match self {
            Hammer => HAMMER,
            Pistol => PISTOL,
            Shotgun => SHOTGUN,
            Grenade => GRENADE,
            Rifle => RIFLE,
            Ninja => NINJA,
        }
    }
}

impl Team {
    pub fn from_i32(i: i32) -> Option<Team> {
        use self::Team::*;
        Some(match i {
            TEAM_SPECTATORS => Spectators,
            TEAM_RED => Red,
            TEAM_BLUE => Blue,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::Team::*;
        match self {
            Spectators => TEAM_SPECTATORS,
            Red => TEAM_RED,
            Blue => TEAM_BLUE,
        }
    }
}

impl Sound {
    pub fn from_i32(i: i32) -> Option<Sound> {
        use self::Sound::*;
        Some(match i {
            SOUND_GUN_FIRE => GunFire,
            SOUND_SHOTGUN_FIRE => ShotgunFire,
            SOUND_GRENADE_FIRE => GrenadeFire,
            SOUND_HAMMER_FIRE => HammerFire,
            SOUND_HAMMER_HIT => HammerHit,
            SOUND_NINJA_FIRE => NinjaFire,
            SOUND_GRENADE_EXPLODE => GrenadeExplode,
            SOUND_NINJA_HIT => NinjaHit,
            SOUND_RIFLE_FIRE => RifleFire,
            SOUND_RIFLE_BOUNCE => RifleBounce,
            SOUND_WEAPON_SWITCH => WeaponSwitch,
            SOUND_PLAYER_PAIN_SHORT => PlayerPainShort,
            SOUND_PLAYER_PAIN_LONG => PlayerPainLong,
            SOUND_BODY_LAND => BodyLand,
            SOUND_PLAYER_AIRJUMP => PlayerAirjump,
            SOUND_PLAYER_JUMP => PlayerJump,
            SOUND_PLAYER_DIE => PlayerDie,
            SOUND_PLAYER_SPAWN => PlayerSpawn,
            SOUND_PLAYER_SKID => PlayerSkid,
            SOUND_TEE_CRY => TeeCry,
            SOUND_HOOK_LOOP => HookLoop,
            SOUND_HOOK_ATTACH_GROUND => HookAttachGround,
            SOUND_HOOK_ATTACH_PLAYER => HookAttachPlayer,
            SOUND_HOOK_NOATTACH => HookNoattach,
            SOUND_PICKUP_HEALTH => PickupHealth,
            SOUND_PICKUP_ARMOR => PickupArmor,
            SOUND_PICKUP_GRENADE => PickupGrenade,
            SOUND_PICKUP_SHOTGUN => PickupShotgun,
            SOUND_PICKUP_NINJA => PickupNinja,
            SOUND_WEAPON_SPAWN => WeaponSpawn,
            SOUND_WEAPON_NOAMMO => WeaponNoammo,
            SOUND_HIT => Hit,
            SOUND_CHAT_SERVER => ChatServer,
            SOUND_CHAT_CLIENT => ChatClient,
            SOUND_CHAT_HIGHLIGHT => ChatHighlight,
            SOUND_CTF_DROP => CtfDrop,
            SOUND_CTF_RETURN => CtfReturn,
            SOUND_CTF_GRAB_PL => CtfGrabPl,
            SOUND_CTF_GRAB_EN => CtfGrabEn,
            SOUND_CTF_CAPTURE => CtfCapture,
            SOUND_MENU => Menu,
            _ => return None,
        })
    }
    pub fn to_i32(self) -> i32 {
        use self::Sound::*;
        match self {
            GunFire => SOUND_GUN_FIRE,
            ShotgunFire => SOUND_SHOTGUN_FIRE,
            GrenadeFire => SOUND_GRENADE_FIRE,
            HammerFire => SOUND_HAMMER_FIRE,
            HammerHit => SOUND_HAMMER_HIT,
            NinjaFire => SOUND_NINJA_FIRE,
            GrenadeExplode => SOUND_GRENADE_EXPLODE,
            NinjaHit => SOUND_NINJA_HIT,
            RifleFire => SOUND_RIFLE_FIRE,
            RifleBounce => SOUND_RIFLE_BOUNCE,
            WeaponSwitch => SOUND_WEAPON_SWITCH,
            PlayerPainShort => SOUND_PLAYER_PAIN_SHORT,
            PlayerPainLong => SOUND_PLAYER_PAIN_LONG,
            BodyLand => SOUND_BODY_LAND,
            PlayerAirjump => SOUND_PLAYER_AIRJUMP,
            PlayerJump => SOUND_PLAYER_JUMP,
            PlayerDie => SOUND_PLAYER_DIE,
            PlayerSpawn => SOUND_PLAYER_SPAWN,
            PlayerSkid => SOUND_PLAYER_SKID,
            TeeCry => SOUND_TEE_CRY,
            HookLoop => SOUND_HOOK_LOOP,
            HookAttachGround => SOUND_HOOK_ATTACH_GROUND,
            HookAttachPlayer => SOUND_HOOK_ATTACH_PLAYER,
            HookNoattach => SOUND_HOOK_NOATTACH,
            PickupHealth => SOUND_PICKUP_HEALTH,
            PickupArmor => SOUND_PICKUP_ARMOR,
            PickupGrenade => SOUND_PICKUP_GRENADE,
            PickupShotgun => SOUND_PICKUP_SHOTGUN,
            PickupNinja => SOUND_PICKUP_NINJA,
            WeaponSpawn => SOUND_WEAPON_SPAWN,
            WeaponNoammo => SOUND_WEAPON_NOAMMO,
            Hit => SOUND_HIT,
            ChatServer => SOUND_CHAT_SERVER,
            ChatClient => SOUND_CHAT_CLIENT,
            ChatHighlight => SOUND_CHAT_HIGHLIGHT,
            CtfDrop => SOUND_CTF_DROP,
            CtfReturn => SOUND_CTF_RETURN,
            CtfGrabPl => SOUND_CTF_GRAB_PL,
            CtfGrabEn => SOUND_CTF_GRAB_EN,
            CtfCapture => SOUND_CTF_CAPTURE,
            Menu => SOUND_MENU,
        }
    }
}
