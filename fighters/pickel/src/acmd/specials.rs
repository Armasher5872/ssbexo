use super::*;

unsafe extern "C" fn ssbexo_pickel_trolley_special_s_drive_partial_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.5, 43, 74, 0, 60, 4.0, 0.0, 3.0, 3.0, Some(0.0), Some(3.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        WorkModule::off_flag(agent.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

unsafe extern "C" fn ssbexo_pickel_trolley_special_s_drive_empty_partial_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 4.0, 0.0, 3.0, 3.0, Some(0.0), Some(3.0), Some(-3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

pub fn install() {
    Agent::new("pickel_trolley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsdrivepartial", ssbexo_pickel_trolley_special_s_drive_partial_acmd, Low)
    .game_acmd("game_specialsdriveemptypartial", ssbexo_pickel_trolley_special_s_drive_empty_partial_acmd, Low)
    .install()
    ;
}