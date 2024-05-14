use super::*;

//Down Smash ACMD
unsafe extern "C" fn ssbexo_koopajr_down_smash_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 0.7);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        ModelModule::joint_global_position(agent.module_accessor, Hash40::new("top"), &mut Vector3f{x: 0.0, y: 0.0, z: 0.0}, false);
        ModelModule::joint_global_position(agent.module_accessor, Hash40::new("hip"), &mut Vector3f{x: 0.0, y: 0.0, z: 0.0}, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 53, 91, 0, 41, 5.2, 0.0, 3.5, 12.5, Some(0.0), Some(6.5), Some(12.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 53, 91, 0, 41, 5.2, 0.0, 3.5, -12.0, Some(0.0), Some(6.5), Some(-12.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
    frame(agent.lua_state_agent, 65.0);
    macros::FT_MOTION_RATE(agent, 0.85);
}

pub fn install() {
    Agent::new("koopajr")
    .game_acmd("game_attacklw4", ssbexo_koopajr_down_smash_acmd)
    .install()
    ;
}