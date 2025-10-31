use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_lucas_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 17.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 75, 76, 0, 60, 2.5, 0.0, 5.5, 1.2, Some(0.0), Some(5.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(agent, 1, 0, Hash40::new("top"), 13.0, 46, 81, 0, 60, 5.8, 0.0, 5.5, 12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("lucas")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_lucas_dash_attack_acmd, Low)
    .install()
    ;
}