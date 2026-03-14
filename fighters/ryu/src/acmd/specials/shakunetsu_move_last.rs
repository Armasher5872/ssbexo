use super::*;

//Shakunetsu Hadoken Move Last ACMD
unsafe extern "C" fn ssbexo_ryu_hadoken_special_move_last_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 55, 60, 0, 58, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    Agent::new("ryu_hadoken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_movespwlast", ssbexo_ryu_hadoken_special_move_last_acmd, Low)
    .game_acmd("game_movespmlast", ssbexo_ryu_hadoken_special_move_last_acmd, Low)
    .game_acmd("game_movespslast", ssbexo_ryu_hadoken_special_move_last_acmd, Low)
    .install()
    ;
}