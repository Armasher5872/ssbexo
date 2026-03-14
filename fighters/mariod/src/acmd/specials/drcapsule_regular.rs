use super::*;

//Pill Regular ACMD
unsafe extern "C" fn ssbexo_mariod_drcapsule_regular_acmd(agent: &mut L2CAgentBase) {
    let owner_boma = get_owner_boma(agent);
    let pill_id = WorkModule::get_int(owner_boma, *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    if pill_id == 0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 65, 40, 0, 60, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
    if pill_id == 1 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 40, 0, 60, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 30.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 40, 0, 50, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
        }
    }
    if pill_id == 2 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 65, 40, 0, 60, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
    }
    if pill_id == 3 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 40, 20, 0, 100, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 4.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 30.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 40, 20, 0, 100, 1.7, 0.0, 1.7, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.0, 4.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    Agent::new("mariod_drcapsule")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_regular", ssbexo_mariod_drcapsule_regular_acmd, Low)
    .install()
    ;
}