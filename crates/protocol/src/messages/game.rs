use crate::enums::*;
use packer::*;

#[derive(Debug, Clone, Copy)]
pub struct MotdChunk<'a> {
    pub message: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct BroadcastChunk<'a> {
    pub message: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct ChatChunk<'a> {
    pub team: i32,
    pub client_id: i32,
    pub message: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct KillMessageChunk {
    pub killer: i32,
    pub victim: i32,
    pub weapon: i32,
    pub mode_special: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalSoundChunk {
    pub sound: Sound,
}

#[derive(Debug, Clone, Copy)]
pub struct TuneParamsChunk {
    pub ground_control_speed: i32,
    pub ground_control_accel: i32,
    pub ground_friction: i32,
    pub ground_jump_impulse: i32,
    pub air_jump_impulse: i32,
    pub air_control_speed: i32,
    pub air_control_accel: i32,
    pub air_friction: i32,
    pub hook_length: i32,
    pub hook_fire_speed: i32,
    pub hook_drag_accel: i32,
    pub hook_drag_speed: i32,
    pub gravity: i32,
    pub velramp_start: i32,
    pub velramp_range: i32,
    pub velramp_curvature: i32,
    pub gun_curvature: i32,
    pub gun_speed: i32,
    pub gun_lifetime: i32,
    pub shotgun_curvature: i32,
    pub shotgun_speed: i32,
    pub shotgun_speeddiff: i32,
    pub shotgun_lifetime: i32,
    pub grenade_curvature: i32,
    pub grenade_speed: i32,
    pub grenade_lifetime: i32,
    pub laser_reach: i32,
    pub laser_bounce_delay: i32,
    pub laser_bounce_num: i32,
    pub laser_bounce_cost: i32,
    pub laser_damage: i32,
    pub player_collision: i32,
    pub player_hooking: i32,
}

impl Default for TuneParamsChunk {
    fn default() -> Self {
        Self {
            ground_control_speed: 1000,
            ground_control_accel: 200,
            ground_friction: 50,
            ground_jump_impulse: 1320,
            air_jump_impulse: 1200,
            air_control_speed: 500,
            air_control_accel: 150,
            air_friction: 95,
            hook_length: 38000,
            hook_fire_speed: 8000,
            hook_drag_accel: 300,
            hook_drag_speed: 1500,
            gravity: 50,
            velramp_start: 55000,
            velramp_range: 200000,
            velramp_curvature: 140,
            gun_curvature: 125,
            gun_speed: 220000,
            gun_lifetime: 200,
            shotgun_curvature: 125,
            shotgun_speed: 275000,
            shotgun_speeddiff: 80,
            shotgun_lifetime: 20,
            grenade_curvature: 700,
            grenade_speed: 100000,
            grenade_lifetime: 200,
            laser_reach: 80000,
            laser_bounce_delay: 15000,
            laser_bounce_num: 100,
            laser_bounce_cost: 0,
            laser_damage: 500,
            player_collision: 100,
            player_hooking: 100,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DDNetTuneParamsChunk {
    pub ground_control_speed: i32,
    pub ground_control_accel: i32,
    pub ground_friction: i32,
    pub ground_jump_impulse: i32,
    pub air_jump_impulse: i32,
    pub air_control_speed: i32,
    pub air_control_accel: i32,
    pub air_friction: i32,
    pub hook_length: i32,
    pub hook_fire_speed: i32,
    pub hook_drag_accel: i32,
    pub hook_drag_speed: i32,
    pub gravity: i32,
    pub velramp_start: i32,
    pub velramp_range: i32,
    pub velramp_curvature: i32,
    pub gun_curvature: i32,
    pub gun_speed: i32,
    pub gun_lifetime: i32,
    pub shotgun_curvature: i32,
    pub shotgun_speed: i32,
    pub shotgun_speeddiff: i32,
    pub shotgun_lifetime: i32,
    pub grenade_curvature: i32,
    pub grenade_speed: i32,
    pub grenade_lifetime: i32,
    pub laser_reach: i32,
    pub laser_bounce_delay: i32,
    pub laser_bounce_num: i32,
    pub laser_bounce_cost: i32,
    pub laser_damage: i32,
    pub player_collision: i32,
    pub player_hooking: i32,
    pub jetpack_strength: i32,
    pub shotgun_strength: i32,
    pub explosion_strength: i32,
    pub hammer_strength: i32,
    pub hook_duration: i32,
    pub hammer_fire_delay: i32,
    pub gun_fire_delay: i32,
    pub shotgun_fire_delay: i32,
    pub grenade_fire_delay: i32,
    pub laser_fire_delay: i32,
    pub ninja_fire_delay: i32,
    pub hammer_hit_fire_delay: i32,
    pub ground_elasticity_x: i32,
    pub ground_elasticity_y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponPickupChunk {
    pub weapon: Weapon,
}

#[derive(Debug, Clone, Copy)]
pub struct EmoticonChunk {
    pub client_id: i32,
    pub emoticon: Emoticon,
}

#[derive(Debug, Clone, Copy)]
pub struct AddVoteOptionListChunk<'a> {
    pub num_options: i32,
    pub description: [&'a str; 15],
}

#[derive(Debug, Clone, Copy)]
pub struct AddVoteOptionChunk<'a> {
    pub description: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct SvVoteOptionRemove<'a> {
    pub description: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct SvVoteSet<'a> {
    pub timeout: i32,
    pub description: &'a str,
    pub reason: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct VoteStatusServerChunk {
    pub yes: i32,
    pub no: i32,
    pub pass: i32,
    pub total: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SayClientChunk<'a> {
    pub team: i32,
    pub message: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct SetTeamClientChunk {
    pub team: Team,
}

#[derive(Debug, Clone, Copy)]
pub struct SetSpectatorModeClientChunk {
    pub spectator_id: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct StartInfoClientChunk<'a> {
    pub name: &'a str,
    pub clan: &'a str,
    pub country: i32,
    pub skin: &'a str,
    pub use_custom_color: i32,
    pub color_body: i32,
    pub color_feet: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ChangeInfoChunk<'a> {
    pub name: &'a str,
    pub clan: &'a str,
    pub country: i32,
    pub skin: &'a str,
    pub use_custom_color: i32,
    pub color_body: i32,
    pub color_feet: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct EmoticonClientChunk {
    pub emoticon: Emoticon,
}

#[derive(Debug, Clone, Copy)]
pub struct VoteClientChunk {
    pub vote: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct CallVoteClientChunk<'a> {
    pub kind: &'a str,
    pub value: &'a str,
    pub reason: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub struct IsDDNetLegacyChunk {
    pub ddnet_version: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct DDRaceTimeLegacyChunk {
    pub time: i32,
    pub check: i32,
    pub finish: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SvRecordLegacy {
    pub server_time_best: i32,
    pub player_time_best: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Unused2;

#[derive(Debug, Clone, Copy)]
pub struct SvTeamsStateLegacy {
    pub teams: [i32; 64],
}

#[derive(Debug, Clone, Copy)]
pub struct ClShowOthersLegacy {
    pub show: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SvMyOwnMessage {
    pub test: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ShowDistanceChunk {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct ShowOthersChunk {
    pub show: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SvTeamsState {
    pub teams: [i32; 64],
}

#[derive(Debug, Clone, Copy)]
pub struct SvDdraceTime {
    pub time: i32,
    pub check: i32,
    pub finish: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct RecordChunk {
    pub server_time_best: i32,
    pub player_time_best: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct KillMessageTeamChunk {
    pub team: i32,
    pub first: i32,
}

impl<'a> MotdChunk<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<MotdChunk<'a>, DecoderError> {
        let result = Ok(MotdChunk {
            message: encoder.read_string()?,
        });

        result
    }
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.message)?;
        Ok(())
    }
}
impl<'a> BroadcastChunk<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<BroadcastChunk<'a>, DecoderError> {
        let result = Ok(BroadcastChunk {
            message: encoder.read_string()?,
        });

        result
    }
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.message)?;
        Ok(())
    }
}

impl<'a> ChatChunk<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<ChatChunk<'a>, DecoderError> {
        let result = Ok(ChatChunk {
            team: encoder.read_int()?,
            client_id: encoder.read_int()?,
            message: encoder.read_string()?,
        });

        result
    }
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.team)?;
        encoder.write_int(self.client_id)?;
        encoder.write_string(self.message)?;
        Ok(())
    }
}

impl KillMessageChunk {
    pub fn decode(encoder: &mut Decoder) -> Result<KillMessageChunk, DecoderError> {
        Ok(KillMessageChunk {
            killer: encoder.read_int()?,
            victim: encoder.read_int()?,
            weapon: encoder.read_int()?,
            mode_special: encoder.read_int()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.killer)?;
        encoder.write_int(self.victim)?;
        encoder.write_int(self.weapon)?;
        encoder.write_int(self.mode_special)?;

        Ok(())
    }
}

impl GlobalSoundChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        Ok(GlobalSoundChunk {
            sound: Sound::from_i32(decoder.read_int()?).unwrap(),
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.sound.to_i32())?;

        Ok(())
    }
}

impl DDNetTuneParamsChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        Ok(Self {
            ground_control_speed: decoder.read_int()?,
            ground_control_accel: decoder.read_int()?,
            ground_friction: decoder.read_int()?,
            ground_jump_impulse: decoder.read_int()?,
            air_jump_impulse: decoder.read_int()?,
            air_control_speed: decoder.read_int()?,
            air_control_accel: decoder.read_int()?,
            air_friction: decoder.read_int()?,
            hook_length: decoder.read_int()?,
            hook_fire_speed: decoder.read_int()?,
            hook_drag_accel: decoder.read_int()?,
            hook_drag_speed: decoder.read_int()?,
            gravity: decoder.read_int()?,
            velramp_start: decoder.read_int()?,
            velramp_range: decoder.read_int()?,
            velramp_curvature: decoder.read_int()?,
            gun_curvature: decoder.read_int()?,
            gun_speed: decoder.read_int()?,
            gun_lifetime: decoder.read_int()?,
            shotgun_curvature: decoder.read_int()?,
            shotgun_speed: decoder.read_int()?,
            shotgun_speeddiff: decoder.read_int()?,
            shotgun_lifetime: decoder.read_int()?,
            grenade_curvature: decoder.read_int()?,
            grenade_speed: decoder.read_int()?,
            grenade_lifetime: decoder.read_int()?,
            laser_reach: decoder.read_int()?,
            laser_bounce_delay: decoder.read_int()?,
            laser_bounce_num: decoder.read_int()?,
            laser_bounce_cost: decoder.read_int()?,
            laser_damage: decoder.read_int()?,
            player_collision: decoder.read_int()?,
            player_hooking: decoder.read_int()?,
            jetpack_strength: decoder.read_int()?,
            shotgun_strength: decoder.read_int()?,
            explosion_strength: decoder.read_int()?,
            hammer_strength: decoder.read_int()?,
            hook_duration: decoder.read_int()?,
            hammer_fire_delay: decoder.read_int()?,
            gun_fire_delay: decoder.read_int()?,
            shotgun_fire_delay: decoder.read_int()?,
            grenade_fire_delay: decoder.read_int()?,
            laser_fire_delay: decoder.read_int()?,
            ninja_fire_delay: decoder.read_int()?,
            hammer_hit_fire_delay: decoder.read_int()?,
            ground_elasticity_x: decoder.read_int()?,
            ground_elasticity_y: decoder.read_int()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.ground_control_speed)?;
        encoder.write_int(self.ground_control_accel)?;
        encoder.write_int(self.ground_friction)?;
        encoder.write_int(self.ground_jump_impulse)?;
        encoder.write_int(self.air_jump_impulse)?;
        encoder.write_int(self.air_control_speed)?;
        encoder.write_int(self.air_control_accel)?;
        encoder.write_int(self.air_friction)?;
        encoder.write_int(self.hook_length)?;
        encoder.write_int(self.hook_fire_speed)?;
        encoder.write_int(self.hook_drag_accel)?;
        encoder.write_int(self.hook_drag_speed)?;
        encoder.write_int(self.gravity)?;
        encoder.write_int(self.velramp_start)?;
        encoder.write_int(self.velramp_range)?;
        encoder.write_int(self.velramp_curvature)?;
        encoder.write_int(self.gun_curvature)?;
        encoder.write_int(self.gun_speed)?;
        encoder.write_int(self.gun_lifetime)?;
        encoder.write_int(self.shotgun_curvature)?;
        encoder.write_int(self.shotgun_speed)?;
        encoder.write_int(self.shotgun_speeddiff)?;
        encoder.write_int(self.shotgun_lifetime)?;
        encoder.write_int(self.grenade_curvature)?;
        encoder.write_int(self.grenade_speed)?;
        encoder.write_int(self.grenade_lifetime)?;
        encoder.write_int(self.laser_reach)?;
        encoder.write_int(self.laser_bounce_delay)?;
        encoder.write_int(self.laser_bounce_num)?;
        encoder.write_int(self.laser_bounce_cost)?;
        encoder.write_int(self.laser_damage)?;
        encoder.write_int(self.player_collision)?;
        encoder.write_int(self.player_hooking)?;
        encoder.write_int(self.jetpack_strength)?;
        encoder.write_int(self.shotgun_strength)?;
        encoder.write_int(self.explosion_strength)?;
        encoder.write_int(self.hammer_strength)?;
        encoder.write_int(self.hook_duration)?;
        encoder.write_int(self.hammer_fire_delay)?;
        encoder.write_int(self.gun_fire_delay)?;
        encoder.write_int(self.shotgun_fire_delay)?;
        encoder.write_int(self.grenade_fire_delay)?;
        encoder.write_int(self.laser_fire_delay)?;
        encoder.write_int(self.ninja_fire_delay)?;
        encoder.write_int(self.hammer_hit_fire_delay)?;
        encoder.write_int(self.ground_elasticity_x)?;
        encoder.write_int(self.ground_elasticity_y)?;

        Ok(())
    }
}

impl TuneParamsChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        Ok(Self {
            ground_control_speed: decoder.read_int()?,
            ground_control_accel: decoder.read_int()?,
            ground_friction: decoder.read_int()?,
            ground_jump_impulse: decoder.read_int()?,
            air_jump_impulse: decoder.read_int()?,
            air_control_speed: decoder.read_int()?,
            air_control_accel: decoder.read_int()?,
            air_friction: decoder.read_int()?,
            hook_length: decoder.read_int()?,
            hook_fire_speed: decoder.read_int()?,
            hook_drag_accel: decoder.read_int()?,
            hook_drag_speed: decoder.read_int()?,
            gravity: decoder.read_int()?,
            velramp_start: decoder.read_int()?,
            velramp_range: decoder.read_int()?,
            velramp_curvature: decoder.read_int()?,
            gun_curvature: decoder.read_int()?,
            gun_speed: decoder.read_int()?,
            gun_lifetime: decoder.read_int()?,
            shotgun_curvature: decoder.read_int()?,
            shotgun_speed: decoder.read_int()?,
            shotgun_speeddiff: decoder.read_int()?,
            shotgun_lifetime: decoder.read_int()?,
            grenade_curvature: decoder.read_int()?,
            grenade_speed: decoder.read_int()?,
            grenade_lifetime: decoder.read_int()?,
            laser_reach: decoder.read_int()?,
            laser_bounce_delay: decoder.read_int()?,
            laser_bounce_num: decoder.read_int()?,
            laser_bounce_cost: decoder.read_int()?,
            laser_damage: decoder.read_int()?,
            player_collision: decoder.read_int()?,
            player_hooking: decoder.read_int()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.ground_control_speed)?;
        encoder.write_int(self.ground_control_accel)?;
        encoder.write_int(self.ground_friction)?;
        encoder.write_int(self.ground_jump_impulse)?;
        encoder.write_int(self.air_jump_impulse)?;
        encoder.write_int(self.air_control_speed)?;
        encoder.write_int(self.air_control_accel)?;
        encoder.write_int(self.air_friction)?;
        encoder.write_int(self.hook_length)?;
        encoder.write_int(self.hook_fire_speed)?;
        encoder.write_int(self.hook_drag_accel)?;
        encoder.write_int(self.hook_drag_speed)?;
        encoder.write_int(self.gravity)?;
        encoder.write_int(self.velramp_start)?;
        encoder.write_int(self.velramp_range)?;
        encoder.write_int(self.velramp_curvature)?;
        encoder.write_int(self.gun_curvature)?;
        encoder.write_int(self.gun_speed)?;
        encoder.write_int(self.gun_lifetime)?;
        encoder.write_int(self.shotgun_curvature)?;
        encoder.write_int(self.shotgun_speed)?;
        encoder.write_int(self.shotgun_speeddiff)?;
        encoder.write_int(self.shotgun_lifetime)?;
        encoder.write_int(self.grenade_curvature)?;
        encoder.write_int(self.grenade_speed)?;
        encoder.write_int(self.grenade_lifetime)?;
        encoder.write_int(self.laser_reach)?;
        encoder.write_int(self.laser_bounce_delay)?;
        encoder.write_int(self.laser_bounce_num)?;
        encoder.write_int(self.laser_bounce_cost)?;
        encoder.write_int(self.laser_damage)?;
        encoder.write_int(self.player_collision)?;
        encoder.write_int(self.player_hooking)?;
        Ok(())
    }
}

impl WeaponPickupChunk {
    pub fn decode(encoder: &mut Decoder) -> Result<WeaponPickupChunk, DecoderError> {
        Ok(WeaponPickupChunk {
            weapon: Weapon::from_i32(encoder.read_int()?).ok_or(DecoderError::UnknownId)?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.weapon.to_i32())?;

        Ok(())
    }
}

impl EmoticonChunk {
    pub fn decode(encoder: &mut Decoder) -> Result<EmoticonChunk, DecoderError> {
        Ok(EmoticonChunk {
            client_id: encoder.read_int()?,
            emoticon: Emoticon::from_i32(encoder.read_int()?).unwrap(),
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.client_id)?;
        encoder.write_int(self.emoticon.to_i32())?;

        Ok(())
    }
}

impl<'a> AddVoteOptionListChunk<'a> {
    pub fn decode(decoder: &mut Decoder<'a>) -> Result<Self, DecoderError> {
        Ok(Self {
            num_options: decoder.read_int()?,
            description: [
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
                decoder.read_string()?,
            ],
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.num_options)?;

        for &e in &self.description {
            encoder.write_string(e)?;
        }

        Ok(())
    }
}

impl<'a> AddVoteOptionChunk<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<AddVoteOptionChunk<'a>, DecoderError> {
        Ok(AddVoteOptionChunk {
            description: encoder.read_string()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.description)?;
        Ok(())
    }
}

impl<'a> SvVoteOptionRemove<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<SvVoteOptionRemove<'a>, DecoderError> {
        Ok(SvVoteOptionRemove {
            description: encoder.read_string()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.description)?;
        Ok(())
    }
}

impl<'a> SayClientChunk<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<SayClientChunk<'a>, DecoderError> {
        Ok(SayClientChunk {
            team: encoder.read_int()?,
            message: encoder.read_string()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.team as i32)?;
        encoder.write_string(self.message)?;
        Ok(())
    }
}

impl SetTeamClientChunk {
    pub fn decode(encoder: &mut Decoder) -> Result<SetTeamClientChunk, DecoderError> {
        Ok(SetTeamClientChunk {
            team: Team::from_i32(encoder.read_int()?).ok_or(DecoderError::UnknownId)?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int(self.team.to_i32())?;

        Ok(())
    }
}

impl<'a> StartInfoClientChunk<'a> {
    pub fn decode(decoder: &mut Decoder<'a>) -> Result<Self, DecoderError> {
        Ok(Self {
            name: decoder.read_string()?,
            clan: decoder.read_string()?,
            country: decoder.read_int()?,
            skin: decoder.read_string()?,
            use_custom_color: decoder.read_int()?,
            color_body: decoder.read_int()?,
            color_feet: decoder.read_int()?,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.name)?;
        encoder.write_string(self.clan)?;
        encoder.write_int(self.country)?;
        encoder.write_string(self.skin)?;
        encoder.write_int(self.use_custom_color as i32)?;
        encoder.write_int(self.color_body)?;
        encoder.write_int(self.color_feet)?;

        Ok(())
    }
}
