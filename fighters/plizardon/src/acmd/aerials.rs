use super::*;

//Fair ACMD
unsafe extern "C" fn ssbexo_plizardon_fair_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    MotionModule::set_rate(boma, 0.666);
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        SET_SPEED_EX(agent, -1.45, 0.4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 80, 0, 20, 8.0, 0.0, 9.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 55, 80, 0, 20, 6.0, 0.0, 8.0, 17.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair Effect
unsafe extern "C" fn ssbexo_plizardon_fair_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.8, 0.6, 0.3);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.2, 0, 0.5);
        FLASH_FRM(agent, 15, 0, 0, 0, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        let vec = Vector3f{x: 0.0, y: 9.0, z: 12.0};
        let f1 = EffectModule::req_follow(boma, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), &vec, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(boma, f1, 1.0, 1.0, 0.333);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        FLASH(agent, 1, 0.7, 0.5, 0.5);
        FLASH_FRM(agent, 10, 0, 0, 0, 0);
    }
}

//Fair Sound
unsafe extern "C" fn ssbexo_plizardon_fair_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_plizardon_attackair_s01"));
    }
}

//Fair Expression
unsafe extern "C" fn ssbexo_plizardon_fair_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("wingl"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("plizardon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackairf", ssbexo_plizardon_fair_acmd, Low)
    .acmd("effect_attackairf", ssbexo_plizardon_fair_effect, Low)
    .acmd("sound_attackairf", ssbexo_plizardon_fair_sound, Low)
    .acmd("expression_attackairf", ssbexo_plizardon_fair_expression, Low)
    .install()
    ;
}