use super::*;

//Hadoken ACMD
unsafe extern "C" fn ssbexo_ryu_hadoken_acmd(agent: &mut L2CAgentBase) {
    wait(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.916);
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 80, 60, 80, 0, 8.0, 0.0, 7.0, 5.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 80, 60, 40, 0, 8.0, 0.0, 7.0, 5.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_RYU_GENERATE_ARTICLE_HADOKEN, false, -1);
            MotionModule::set_rate(agent.module_accessor, 2.15);
        }
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 57.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
    }
}

pub fn install() {
    Agent::new("ryu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialn", ssbexo_ryu_hadoken_acmd, Low)
    .game_acmd("game_specialairn", ssbexo_ryu_hadoken_acmd, Low)
    .install()
    ;
}