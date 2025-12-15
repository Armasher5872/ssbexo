use super::*;

//Neutral Special Down Throw Land ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_down_throw_land_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let multiplier = WorkModule::get_float(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_PILEDRIVER_MULTIPLIER);
        let total_damage = 14.6*multiplier;
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, total_damage, 63, 95, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 1.0, 63, 91, 0, 28, 5.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        CHECK_FINISH_CAMERA(agent, 0, 4);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Neutral Special Down Throw Land Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_down_throw_land_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.2);
        EFFECT(agent, Hash40::new("wario_shock"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Neutral Special Down Throw Land Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_down_throw_land_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_special_l04"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_wario_win02"));
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_wario_step_right_s"));
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_wario_swing_l"));
    }
}

//Neutral Special Down Throw Land Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_down_throw_land_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        QUAKE(agent, *CAMERA_QUAKE_KIND_L);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnthrowlwland", ssbexo_wario_neutral_special_down_throw_land_acmd, Low)
    .effect_acmd("effect_specialnthrowlwland", ssbexo_wario_neutral_special_down_throw_land_effect, Low)
    .sound_acmd("sound_specialnthrowlwland", ssbexo_wario_neutral_special_down_throw_land_sound, Low)
    .expression_acmd("expression_specialnthrowlwland", ssbexo_wario_neutral_special_down_throw_land_expression, Low)
    .install()
    ;
}