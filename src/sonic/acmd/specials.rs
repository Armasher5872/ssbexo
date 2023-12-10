use super::*;

//Homing Attack Search ACMD
unsafe extern "C" fn ssbexo_sonic_homing_attack_search_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("hip"), 45.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
}

//Homing Attack Hit ACMD
unsafe extern "C" fn ssbexo_sonic_homing_attack_hit_acmd(agent: &mut L2CAgentBase) {
    let vector = Vector3f{x: -0.1, y: 0.5, z: 0.0};
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &vector);
    }
}

//Grounded Boost Start ACMD
unsafe extern "C" fn ssbexo_sonic_grounded_boost_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        let rand_num_10 = smash::app::sv_math::rand(hash40("agent"), 10);
        if rand_num_10 <= 2 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_shield"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("waist"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        } 
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("waist"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        } 
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_s_end_start"), 1.0, 1.0, false, 0.0, false, false);
    }
}

//Aerial Boost Start ACMD
unsafe extern "C" fn ssbexo_sonic_aerial_boost_start_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        let rand_num_10 = smash::app::sv_math::rand(hash40("agent"), 10);
        if rand_num_10 <= 2 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sys_shield"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("waist"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
        } 
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::ATTACK(agent, 0, 0, Hash40::new("head"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 1, 0, Hash40::new("bust"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 2, 0, Hash40::new("waist"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            macros::ATTACK(agent, 3, 0, Hash40::new("hip"), 12.0, 30, 100, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        } 
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_s_end_start"), 1.0, 1.0, false, 0.0, false, false);
    }
}

//Grounded Down Special Jump ACMD
unsafe extern "C" fn ssbexo_sonic_grounded_spin_dash_jump_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::UNABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 80, 60, 0, 80, 4.0, 0.0, 1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_captured_same_time_attack(agent.module_accessor, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SONIC_STATUS_SPIN_JUMP_WORK_ID_FLAG_ENABLE_JUMP_AERIAL);
        macros::ENABLE_AREA(agent, *FIGHTER_AREA_KIND_TREAD_JUMP_CHECK);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Aerial Down Special Start ACMD
unsafe extern "C" fn ssbexo_sonic_bounce_bracelet_start_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(agent, 0, 2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        let rand_num_10 = smash::app::sv_math::rand(hash40("agent"), 10);
        if rand_num_10 <= 2 {
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_shield"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.6, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
            macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 1.0);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(agent, 0.0, -20.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::ATTACK(agent, 0, 0, Hash40::new("sphere"), 3.0, 270, 50, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_middle"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            macros::EFFECT_FOLLOW(agent, Hash40::new("sonic_spintrace_homing"), Hash40::new("sphere"), 0, 0, 0, 0, 0, 0, 1, false);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(agent, 0.0, -20.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::ATTACK(agent, 0, 0, Hash40::new("sphere"), 3.0, 270, 50, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

//Aerial Down Special Bounce ACMD
unsafe extern "C" fn ssbexo_sonic_bounce_bracelet_end_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_HI_JUMP);
        macros::SET_SPEED_EX(agent, 0, 5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
    }
}

pub fn install() {
    Agent::new("sonic")
    .game_acmd("game_specialnhomingstart", ssbexo_sonic_homing_attack_search_acmd)
    .game_acmd("game_specialnhit", ssbexo_sonic_homing_attack_hit_acmd)
    .game_acmd("game_specialsstart", ssbexo_sonic_grounded_boost_start_acmd)
    .game_acmd("game_specialairsstart", ssbexo_sonic_aerial_boost_start_acmd)
    .game_acmd("0x195dc47911", ssbexo_sonic_grounded_spin_dash_jump_acmd)
    .game_acmd("game_specialairlwstart", ssbexo_sonic_bounce_bracelet_start_acmd)
    .game_acmd("game_specialairlwend", ssbexo_sonic_bounce_bracelet_end_acmd)
    .install()
    ;
}