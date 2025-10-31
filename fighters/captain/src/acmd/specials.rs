use super::*;

unsafe extern "C" fn ssbexo_captain_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.867);
    frame(agent.lua_state_agent, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KICK_WALL_CHECK);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 15.0, 88, 35, 0, 80, 5.76, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 12.0, 88, 20, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 9.0, 88, 20, 0, 80, 5.0, 4.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairlw", ssbexo_captain_aerial_down_special_acmd, Low)
    .install()
    ;
}