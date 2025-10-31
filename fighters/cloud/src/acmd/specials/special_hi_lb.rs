use super::*;

//Limit Break Climhazzard ACMD
unsafe extern "C" fn ssbexo_cloud_limit_break_climhazzard_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        display_final_window(true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 40, 100, 80, 0, 2.0, 0.0, 7.0, 3.0, Some(0.0), Some(7.0), Some(3.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 78, 100, 80, 0, 5.0, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 6.0, 102, 100, 80, 0, 3.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        SA_SET(agent, *SITUATION_KIND_AIR);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_LB_SCENE);
        ATTACK(agent, 0, 1, Hash40::new("haver"), 7.0, 88, 107, 0, 50, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 7.0, 88, 107, 0, 50, 5.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 1, Hash40::new("top"), 7.0, 88, 107, 0, 50, 3.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(6.5), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        ATTACK(agent, 0, 1, Hash40::new("haver"), 7.0, 88, 110, 0, 50, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 1, Hash40::new("haver"), 7.0, 88, 110, 0, 50, 5.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 1, Hash40::new("haver"), 7.0, 90, 40, 0, 98, 3.5, 0.0, 14.5, -10.0, Some(0.0), Some(6.0), Some(-10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT_RESERVE);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_IS_ENABLE_CONTROL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_FALL);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        display_final_window(false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x25813802b6));
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Limit Break Climhazzard Effect
unsafe extern "C" fn ssbexo_cloud_limit_break_climhazzard_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("cloud_speedline"), Hash40::new("top"), 0, 7, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 1, 0.965, 0.376);
        sv_animcmd::EFFECT_COLOR(agent.lua_state_agent);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_CLIMHAZZARD_SWORD, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.6);
        EFFECT(agent, Hash40::new("cloud_climhazzard_sting"), Hash40::new("haver"), 0, 3, 0, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_red"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.1);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("cloud_climhazzard_slash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_OFF_KIND(agent, Hash40::new("cloud_climhazzard_sting"), false, true);
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_CLIMHAZZARD_SWORD, false, true);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi_lb", ssbexo_cloud_limit_break_climhazzard_acmd, Low)
    .game_acmd("game_specialairhi_lb", ssbexo_cloud_limit_break_climhazzard_acmd, Low)
    .effect_acmd("effect_specialhi_lb", ssbexo_cloud_limit_break_climhazzard_effect, Low)
    .effect_acmd("effect_specialairhi_lb", ssbexo_cloud_limit_break_climhazzard_effect, Low)
    .install()
    ;
}