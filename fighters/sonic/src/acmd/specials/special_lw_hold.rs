use super::*;

//Down Special Hold ACMD
unsafe extern "C" fn ssbexo_sonic_down_special_hold_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT as u64, 0, Hash40::new("hip"), 1.0, 90, 0, 0, 58, 3.8, 0.0, 1.5, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(agent.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_ATTACK_ID_DEFAULT, true);
        AttackModule::set_attack_keep_rumble(agent.module_accessor, 0, true);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwhold", ssbexo_sonic_down_special_hold_acmd, Low)
    .game_acmd("game_specialairlwhold", ssbexo_sonic_down_special_hold_acmd, Low)
    .game_acmd("game_speciallwholdchargelw", ssbexo_sonic_down_special_hold_acmd, Low)
    .game_acmd("game_speciallwholdchargemiddle", ssbexo_sonic_down_special_hold_acmd, Low)
    .game_acmd("game_speciallwholdchargehi", ssbexo_sonic_down_special_hold_acmd, Low)
    .install()
    ;
}