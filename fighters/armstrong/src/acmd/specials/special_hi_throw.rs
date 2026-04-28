use super::*;

//Up Special Throw ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 361, 108, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        FT_MOTION_RATE(agent, 30.0/24.0);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
        armstrong_clear_charge(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

//Up Special Throw Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS_NO_STOP(agent, Hash40::new("armstrong_flame_grab"), Hash40::new("havel"), 3, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("armstrong_flame_grab"), false, true);
    }
}

//Up Special Throw Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h02"));
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

//Up Special Throw Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .game_acmd("game_specialhithrow", ssbexo_armstrong_up_special_throw_acmd, Low)
    .effect_acmd("effect_specialhithrow", ssbexo_armstrong_up_special_throw_effect, Low)
    .sound_acmd("sound_specialhithrow", ssbexo_armstrong_up_special_throw_sound, Low)
    .expression_acmd("expression_specialhithrow", ssbexo_armstrong_up_special_throw_expression, Low)
    .install()
    ;
}