use super::*;

//Fair ACMD
unsafe extern "C" fn ssbexo_plizardon_fair_acmd(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 0.666);
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        SET_SPEED_EX(agent, -1.45, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 20, 8.0, 0.0, 9.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 80, 0, 20, 6.0, 0.0, 8.0, 17.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_plizardon_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.8, 0.6, 0.3);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.2, 0, 0.5);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    wait(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        let vec = Vector3f{x: 0.0, y: 9.0, z: 12.0};
        let f1 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), &vec, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(agent.module_accessor, f1, 1.0, 1.0, 0.333);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.7, 0.5, 0.5);
        FLASH_FRM(agent, 10, 0, 0, 0, 0);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_plizardon_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_plizardon_attackair_s01"));
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_plizardon_fair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("wingl"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("plizardon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairf", ssbexo_plizardon_fair_acmd, Low)
    .effect_acmd("effect_attackairf", ssbexo_plizardon_fair_effect, Low)
    .sound_acmd("sound_attackairf", ssbexo_plizardon_fair_sound, Low)
    .expression_acmd("expression_attackairf", ssbexo_plizardon_fair_expression, Low)
    .install()
    ;
}