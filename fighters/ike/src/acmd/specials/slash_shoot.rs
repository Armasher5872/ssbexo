use super::*;

//Slash ACMD
unsafe extern "C" fn ssbexo_ike_slash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 361, 80, 0, 55, 5.0, 0.0, 7.0, 0.9, Some(0.0), Some(1.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 361, 80, 0, 55, 5.0, 0.0, -3.7, 2.2, Some(0.0), Some(1.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

//Slash Effect
unsafe extern "C" fn ssbexo_ike_slash_effect(agent: &mut L2CAgentBase) {
    let owner_boma = get_owner_boma(agent);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    if owner_situation_kind == *SITUATION_KIND_AIR {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_slash2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("ike_slash"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1, true);
        }
    }
}

//Slash Sound
unsafe extern "C" fn ssbexo_ike_slash_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("se_ike_swing_l"));
        sv_animcmd::SET_TAKEOUT_SE_STATUS(agent.lua_state_agent);
    }
}

pub fn install() {
    Agent::new("ike_slash")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_ike_slash_acmd, Low)
    .effect_acmd("effect_shoot", ssbexo_ike_slash_effect, Low)
    .sound_acmd("sound_shoot", ssbexo_ike_slash_sound, Low)
    .install()
    ;
}