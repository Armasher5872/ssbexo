use super::*;

unsafe extern "C" fn ssbexo_metaknight_beam_shoot_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let owner_boma = get_owner_boma(agent);
        let damage = WorkModule::get_float(owner_boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.0+damage, 361, 40, 0, 40, 5.0, 0.0, 7.0, 0.9, Some(0.0), Some(1.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0+damage, 361, 40, 0, 40, 5.0, 0.0, -3.7, 2.2, Some(0.0), Some(1.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn ssbexo_metaknight_beam_shoot_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_beam"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
    }
}

unsafe extern "C" fn ssbexo_metaknight_beam_shoot_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_metaknight_attackair_f03"));
        SET_TAKEOUT_SE_STATUS(agent.lua_state_agent);
    }
}

pub fn install() {
    Agent::new("metaknight_beam")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_metaknight_beam_shoot_acmd, Low)
    .effect_acmd("effect_shoot", ssbexo_metaknight_beam_shoot_effect, Low)
    .sound_acmd("sound_shoot", ssbexo_metaknight_beam_shoot_sound, Low)
    .install()
    ;
}