use super::*;

//Up Smash ACMD
unsafe extern "C" fn ssbexo_pacman_up_smash_acmd(agent: &mut L2CAgentBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(agent.lua_state_agent);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent)  {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent)  {
        SIZE1[get_player_number(module_accessor)] = 6.2;
        SIZE2[get_player_number(module_accessor)] = 4.5;
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent)  {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 3.0, 115, 100, 125, 0, 3.5, 0.0, 8.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            SIZE1[get_player_number(module_accessor)] += 4.0;
            SIZE2[get_player_number(module_accessor)] += 4.0;
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent)  {
        AttackModule::clear_all(module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent)  {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 14.0, 83, 97, 0, 32, SIZE1[get_player_number(module_accessor)], 0.0, 5.8, 0.0, Some(0.0), Some(5.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent)  {
        macros::ATTACK(agent, 0, 0, Hash40::new("ghost1"), 8.0, 83, 92, 0, 32, SIZE2[get_player_number(module_accessor)], 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent)  {
        AttackModule::clear_all(module_accessor);
    }
}

pub fn install() {
    Agent::new("pacman")
    .game_acmd("game_attackhi4", ssbexo_pacman_up_smash_acmd, Priority::Low)
    .install()
    ;
}