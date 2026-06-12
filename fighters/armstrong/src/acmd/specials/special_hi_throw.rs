use super::*;

//Up Special Throw ACMD
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 361, 108, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 8.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        armstrong_clear_charge(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL);
    }
}

//Up Special Throw Effect
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("armstrong_flame_grab"), false, true);
    }
}

//Up Special Throw Sound
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

//Up Special Throw Expression
unsafe extern "C" fn ssbexo_armstrong_up_special_throw_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .acmd("game_specialhithrow", ssbexo_armstrong_up_special_throw_acmd, Low)
    .acmd("effect_specialhithrow", ssbexo_armstrong_up_special_throw_effect, Low)
    .acmd("sound_specialhithrow", ssbexo_armstrong_up_special_throw_sound, Low)
    .acmd("expression_specialhithrow", ssbexo_armstrong_up_special_throw_expression, Low)
    .install()
    ;
}