use super::*;

//Arrow Stick ACMD
unsafe extern "C" fn ssbexo_link_bowarrow_stick_acmd(agent: &mut L2CAgentBase) {
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
            ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 75, 50, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        }
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_stick", ssbexo_link_bowarrow_stick_acmd, Low)
    .game_acmd("game_hitstick", ssbexo_link_bowarrow_stick_acmd, Low)
    .install()
    ;
}