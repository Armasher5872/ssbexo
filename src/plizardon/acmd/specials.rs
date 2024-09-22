use super::*;

//Flamethrower Move ACMD
unsafe extern "C" fn ssbexo_plizardon_flamethrower_move_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 45, 50, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LIZARDON_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 58, 30, 0, 15, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LIZARDON_FIRE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 58, 30, 0, 15, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.7, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_LIZARDON_FIRE, *ATTACK_REGION_NONE);
    }
}

//Explosion Start ACMD
unsafe extern "C" fn ssbexo_plizardon_explosion_start_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 77, 0, 70, 11.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 361, 77, 0, 70, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-10.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
    }
}

pub fn install() {
    Agent::new("plizardon_breath")
    .game_acmd("game_move", ssbexo_plizardon_flamethrower_move_acmd, Priority::Low)
    .install()
    ;
    Agent::new("plizardon_explosion")
    .game_acmd("game_start", ssbexo_plizardon_explosion_start_acmd, Priority::Low)
    .install()
    ;
}