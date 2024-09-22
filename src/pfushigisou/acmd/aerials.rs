use super::*;

//Uair ACMD
unsafe extern "C" fn ssbexo_pfushigisou_uair_acmd(agent: &mut L2CAgentBase) {
    let get_sum_speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);

    }
    frame(agent.lua_state_agent, 12.0);
    if get_sum_speed_y > 0.0 {
        if macros::is_excute(agent) {
            macros::SET_SPEED_EX(agent, 0, -2.208, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("flower"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 90, 72, 0, 64, 7.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 90, 72, 0, 64, 13.0, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(agent.module_accessor, 0, 30, 5, 0.5, false);
        AttackModule::set_poison_param(agent.module_accessor, 1, 30, 5, 0.5, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_pfushigisou_dair_acmd(agent: &mut L2CAgentBase) {
    let get_sum_speed_y = KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    frame(agent.lua_state_agent, 3.0);
    MotionModule::set_rate(agent.module_accessor, 0.8);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    MotionModule::set_rate(agent.module_accessor, 1.0);
    if get_sum_speed_y < 0.0 {
        if macros::is_excute(agent) {
            macros::SET_SPEED_EX(agent, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    if macros::is_excute(agent) {
        KineticModule::suspend_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 27, 4.0, 0.0, -13.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 270, 62, 0, 17, 13.0, 0.0, -14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 270, 98, 0, 9, 4.0, 0.0, -13.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 10.0, 270, 62, 0, 8, 13.0, 0.0, -14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(agent.module_accessor, 0, 30, 5, 0.5, false);
        AttackModule::set_poison_param(agent.module_accessor, 1, 30, 5, 0.5, false);
        AttackModule::set_poison_param(agent.module_accessor, 2, 30, 5, 0.5, false);
        AttackModule::set_poison_param(agent.module_accessor, 3, 30, 5, 0.5, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        KineticModule::resume_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .game_acmd("game_attackairhi", ssbexo_pfushigisou_uair_acmd, Priority::Low)
    .game_acmd("game_attackairlw", ssbexo_pfushigisou_dair_acmd, Priority::Low)
    .install()
    ;
}