use super::*;

//Volley ACMD
unsafe extern "C" fn ssbexo_ganon_volley_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let scale = WorkModule::get_float(agent.module_accessor, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_DAMAGE_SCALE);
        ATTACK(agent, 0, 0, Hash40::new("rot"), 3.0+(scale*3.0), 80, 40, 0, 20, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

//Volley Effect
unsafe extern "C" fn ssbexo_ganon_volley_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let scale = WorkModule::get_float(agent.module_accessor, *WEAPON_GANON_VOLLEY_INSTANCE_WORK_ID_FLOAT_SCALE);
        if scale > 1.0 {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_volley"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 3000.0+(scale*1000.0), true);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("ganon_volley"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 3000.0, true);
        }
    }
}

//Volley Sound
unsafe extern "C" fn ssbexo_ganon_volley_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let glow = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_special_n05"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, glow as i32, 2.0, 0);
        SoundModule::set_remain_se(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("ganon_volley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_fly", ssbexo_ganon_volley_acmd, Low)
    .effect_acmd("effect_fly", ssbexo_ganon_volley_effect, Low)
    .sound_acmd("sound_fly", ssbexo_ganon_volley_sound, Low)
    .install()
    ;
}