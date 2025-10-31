use super::*;

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_miiswordsman_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 60, 0, 87, 4.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(18.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.6);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 361, 60, 0, 87, 4.0, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(17.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 1.6);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    FT_MOTION_RATE(agent, 0.75);
}

pub fn install() {
    Agent::new("miiswordsman")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackdash", ssbexo_miiswordsman_dash_attack_acmd, Low)
    .install()
    ;
}