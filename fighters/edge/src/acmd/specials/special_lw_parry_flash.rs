use super::*;

//Retaliation Stance Parry Scintilla ACMD
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_flash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW_FLASH);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_SHIELD, 0, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW_FLASH);
    }
}

//Retaliation Stance Parry Scintilla Effect
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_flash_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_muzzleflash"), Hash40::new("waist"), 0, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        FLASH(agent, 1, 1, 1, 0.75);
    }
    wait(lua_state, 1.0);
    for _ in 0..4 {
        if is_excute(agent) {
            FLASH(agent, 0.7, 0.7, 0.7, 0.5);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            FLASH(agent, 0.67, 0, 0.78, 0.31);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            COL_NORMAL(agent);
        }
        wait(lua_state, 2.0);
    }
}

//Retaliation Stance Parry Scintilla Sound
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_flash_sound(agent: &mut L2CAgentBase) {
    let rand = sv_math::randf(hash40("fighter"), 1.0);
    let boma = agent.module_accessor;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_winged_jump01"));
        PLAY_SE(agent, Hash40::new("se_edge_special_l01_01"));
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if rand > 0.66 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_l01"));
            }
        }
        else if rand > 0.33 {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("vc_edge_special_s02"));
            }
        }
    }
}

//Retaliation Stance Parry Scintilla Expression
unsafe extern "C" fn ssbexo_edge_retaliation_stance_parry_flash_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_speciallwparryflash", ssbexo_edge_retaliation_stance_parry_flash_acmd, Low)
    .acmd("game_specialairlwparryflash", ssbexo_edge_retaliation_stance_parry_flash_acmd, Low)
    .acmd("effect_speciallwparryflash", ssbexo_edge_retaliation_stance_parry_flash_effect, Low)
    .acmd("effect_specialairlwparryflash", ssbexo_edge_retaliation_stance_parry_flash_effect, Low)
    .acmd("sound_speciallwparryflash", ssbexo_edge_retaliation_stance_parry_flash_sound, Low)
    .acmd("sound_specialairlwparryflash", ssbexo_edge_retaliation_stance_parry_flash_sound, Low)
    .acmd("expression_speciallwparryflash", ssbexo_edge_retaliation_stance_parry_flash_expression, Low)
    .acmd("expression_specialairlwparryflash", ssbexo_edge_retaliation_stance_parry_flash_expression, Low)
    .install()
    ;
}