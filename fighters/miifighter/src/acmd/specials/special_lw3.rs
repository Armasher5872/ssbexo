use super::*;

//Counter Throw ACMD
unsafe extern "C" fn ssbexo_miifighter_counter_throw_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
        SEARCH(agent, 1, 0, Hash40::new("top"), 8.0, 0.0, 7.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ITEM, *COLLISION_PART_MASK_ALL, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MIIFIGHTER_STATUS_COUNTER_THROW_FLAG_SHIELD);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

pub fn install() {
    Agent::new("miifighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw3", ssbexo_miifighter_counter_throw_acmd, Low)
    .game_acmd("game_specialairlw3", ssbexo_miifighter_counter_throw_acmd, Low)
    .install()
    ;
}