use super::*;

//Up Tilt ACMD
unsafe extern "C" fn ssbexo_mario_up_tilt_acmd(agent: &mut L2CAgentBase) {
    let rng = sv_math::rand(hash40("agent"), 100);
    let attribute = if rng <= 11 {if rng == 1 {Hash40::new("collision_attr_mario_local_coin")} else {Hash40::new("collision_attr_coin")}} else {Hash40::new("collision_attr_normal")};
    let sound = if rng <= 11 {if rng == 1 {*COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN} else {*COLLISION_SOUND_ATTR_COIN}} else {*COLLISION_SOUND_ATTR_PUNCH};
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 5.5, 96, 130, 0, 28, 3.5, -0.5, -0.8, 0.2, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attribute, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 5.5, 96, 130, 0, 28, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attribute, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("arml"), 5.5, 96, 130, 0, 28, 5.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, attribute, *ATTACK_SOUND_LEVEL_M, sound, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("mario")
    .game_acmd("game_attackhi3", ssbexo_mario_up_tilt_acmd, Priority::Low)
    .install()
    ;
}