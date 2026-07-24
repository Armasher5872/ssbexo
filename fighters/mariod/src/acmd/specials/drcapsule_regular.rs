use super::*;

//Pill Regular ACMD
unsafe extern "C" fn ssbexo_mariod_drcapsule_regular_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 65, 40, 0, 60, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(boma);
    }
}

pub fn install() {
    Agent::new("mariod_drcapsule")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_regular", ssbexo_mariod_drcapsule_regular_acmd, Low)
    .install()
    ;
}