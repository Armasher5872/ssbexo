use super::*;

//Boomerang Turn ACMD
unsafe extern "C" fn ssbexo_link_boomerang_turn_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 70, 40, 0, 50, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH, 0, 0, *MA_MSC_CMD_SEARCH_SEARCH, 10.0 /*radius*/, 0.0, 0.0 /*y*/, 0.0 /*x*/, *COLLISION_KIND_MASK_ATTACK, *COLLISION_CATEGORY_MASK_ITEM | *COLLISION_CATEGORY_MASK_FIGHTER | *COLLISION_CATEGORY_MASK_ITEM_E, *COLLISION_SITUATION_MASK_GA, 0, *COLLISION_PART_MASK_ALL, *HIT_STATUS_MASK_ALL, 1, false, 1);
        search!(agent, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
}

pub fn install() {
    Agent::new("link_boomerang")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_turn", ssbexo_link_boomerang_turn_acmd, Low)
    .install()
    ;
}