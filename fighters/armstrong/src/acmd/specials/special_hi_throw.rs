use super::*;

//Up Special Throw ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 361, 108, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 2.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

//Up Special Throw Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("armstrong_flame_grab"), false, true);
        EFFECT(agent, Hash40::new("armstrong_flame_grab_bomb"), Hash40::new("top"), 0, 12, -4, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Special Throw Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_ganon_special_h02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_h04"));
    }
}

//Up Special Throw Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_specialhithrow", ssbexo_armstrong_up_special_throw_acmd, Low)
    .effect_acmd("effect_specialhithrow", ssbexo_armstrong_up_special_throw_effect, Low)
    .sound_acmd("sound_specialhithrow", ssbexo_armstrong_up_special_throw_sound, Low)
    .expression_acmd("expression_specialhithrow", ssbexo_armstrong_up_special_throw_expression, Low)
    .install()
    ;
}