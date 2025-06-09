use super::*;

//Operator Down Special ACMD
unsafe extern "C" fn ssbexo_cloud_operator_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ShieldModule::set_status(agent.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        ShieldModule::set_status(agent.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
}

//Grounded Operator Down Special Effect
unsafe extern "C" fn ssbexo_cloud_grounded_operator_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_muzzleflash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 70.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Aerial Operator Down Special Effect
unsafe extern "C" fn ssbexo_cloud_aerial_operator_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_muzzleflash"), Hash40::new("waist"), 0, 0, 0, 0, 0, 90, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Grounded Operator Down Special Sound
unsafe extern "C" fn ssbexo_cloud_grounded_operator_down_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let swap = SoundModule::play_se(agent.module_accessor, Hash40::new("se_cloud_special_l01"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swap as i32, 4.0, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_step_left_m"));
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_step_right_m"));
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_landing03"));
    }
}

//Aerial Operator Down Special Sound
unsafe extern "C" fn ssbexo_cloud_aerial_operator_down_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let swap = SoundModule::play_se(agent.module_accessor, Hash40::new("se_cloud_special_l01"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swap as i32, 4.0, 0);
    }
}

//Grounded Operator Down Special Expression
unsafe extern "C" fn ssbexo_cloud_grounded_operator_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}

//Aerial Operator Down Special Expression
unsafe extern "C" fn ssbexo_cloud_aerial_operator_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwstart", ssbexo_cloud_operator_down_special_acmd, Low)
    .game_acmd("game_specialairlwstart", ssbexo_cloud_operator_down_special_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_cloud_grounded_operator_down_special_effect, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_cloud_aerial_operator_down_special_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_cloud_grounded_operator_down_special_sound, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_cloud_aerial_operator_down_special_sound, Low)
    .expression_acmd("expression_speciallwstart", ssbexo_cloud_grounded_operator_down_special_expression, Low)
    .expression_acmd("expression_specialairlwstart", ssbexo_cloud_aerial_operator_down_special_expression, Low)
    .install()
    ;
}