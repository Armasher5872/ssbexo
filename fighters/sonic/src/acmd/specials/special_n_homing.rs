use super::*;

//Neutral Special Homing ACMD
unsafe extern "C" fn ssbexo_sonic_special_n_homing_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        MotionModule::set_rate(agent.module_accessor, 80.0);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_N_HOMING_FLAG_IS_KIRBY) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("hip"), 7.0, 77, 40, 0, 60, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
            AttackModule::set_captured_same_time_attack(agent.module_accessor, 0, true);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("rot"), 7.0, 77, 40, 0, 60, 5.0, 0.5, 1.5, 0.0, Some(0.5), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.5);
            AttackModule::set_captured_same_time_attack(agent.module_accessor, 0, true);
        }
    }
    if is_excute(agent) {
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnhoming", ssbexo_sonic_special_n_homing_acmd, Low)
    .install()
    ;
}