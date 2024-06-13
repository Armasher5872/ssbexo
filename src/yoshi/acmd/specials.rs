use super::*;

//Side Special Start ACMD
unsafe extern "C" fn ssbexo_yoshi_side_special_start_acmd(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 1.333);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_DISP_EGG);
    }
}

//Side Special Loop ACMD
unsafe extern "C" fn ssbexo_yoshi_side_special_loop_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 50, 0, 43, 8.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        JostleModule::set_status(agent.module_accessor, false);
    }
}

pub fn install() {
    Agent::new("yoshi")
    .game_acmd("game_specialairsstart", ssbexo_yoshi_side_special_start_acmd, Priority::Low)
    .game_acmd("game_specialsloop", ssbexo_yoshi_side_special_loop_acmd, Priority::Low)
    .game_acmd("game_specialairsloop", ssbexo_yoshi_side_special_loop_acmd, Priority::Low)
    .install()
    ;
}