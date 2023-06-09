use super::*;

//Up Tilt
#[acmd_script( agent = "pacman", script = "game_attackhi3", category = ACMD_GAME)]
unsafe fn ssbuexo_pacman_up_tilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0, 96, 100, 0, 70, 6.5, 10.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Up Tilt Effect
#[acmd_script( agent = "pacman", script = "effect_attackhi3", category = ACMD_EFFECT)]
unsafe fn ssbuexo_pacman_up_tilt_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new_raw(0x0e27bc68a2u64), Hash40::new("top"), -2.0, 8.0, 4.5, 0, -95, -82, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_pacman_up_tilt,
        ssbuexo_pacman_up_tilt_effect
    );
}