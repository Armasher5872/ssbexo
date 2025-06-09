use super::*;

//Forward Smash ACMD
unsafe extern "C" fn ssbexo_ness_forward_smash_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("bat") as i64, hash40("bat_visible") as i64);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 361, 67, 0, 70, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 20.0, 361, 67, 0, 70, 2.6, 0.0, 3.85, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 22.0, 361, 67, 0, 70, 3.0, 0.0, 7.7, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NESS_BAT, *ATTACK_REGION_BAT);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_NESS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("bat") as i64, hash40("bat_invisible") as i64);
    }
}

//Yoyo Smash Charge ACMD
unsafe extern "C" fn ssbexo_ness_yoyo_smash_charge_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ness")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attacks4", ssbexo_ness_forward_smash_acmd, Low)
    .install()
    ;
    Agent::new("ness_yoyohead")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackhi4charge", ssbexo_ness_yoyo_smash_charge_acmd, Low)
    .game_acmd("game_attacklw4charge", ssbexo_ness_yoyo_smash_charge_acmd, Low)
    .install()
    ;
}