use super::*;

//Grounded Side Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_catch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    }
}

//Grounded Side Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Grounded Side Special Catch Sound
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_catch_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_jump"));
        PLAY_SE(agent, Hash40::new("se_ganon_jump01"));
    }
}

//Grounded Side Special Catch Expression
unsafe extern "C" fn ssbexo_armstrong_grounded_side_special_catch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Aerial Side Special Catch ACMD
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    }
}

//Aerial Side Special Catch Effect
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("sys_catch"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Aerial Side Special Catch Sound
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_ganon_special_s02"));
        PLAY_SE(agent, Hash40::new("se_ganon_special_s03"));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_jump"));
        PLAY_SE(agent, Hash40::new("se_ganon_jump01"));
    }
}

//Aerial Side Special Catch Expression
unsafe extern "C" fn ssbexo_armstrong_aerial_side_special_catch_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .game_acmd("game_specials", ssbexo_armstrong_grounded_side_special_catch_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_armstrong_grounded_side_special_catch_effect, Low)
    .sound_acmd("sound_specials", ssbexo_armstrong_grounded_side_special_catch_sound, Low)
    .expression_acmd("expression_specials", ssbexo_armstrong_grounded_side_special_catch_expression, Low)
    .game_acmd("game_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_acmd, Low)
    .effect_acmd("effect_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_effect, Low)
    .sound_acmd("sound_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_sound, Low)
    .expression_acmd("expression_specialairscatch", ssbexo_armstrong_aerial_side_special_catch_expression, Low)
    .install()
    ;
}