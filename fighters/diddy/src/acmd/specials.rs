use super::*;

//Neutral Special Explode ACMD
unsafe extern "C" fn ssbexo_diddy_neutral_special_explode_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 35.0, 361, 92, 0, 30, 8.2, 0.0, 8.5, 7.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Side Special Jump ACMD
unsafe extern "C" fn ssbexo_diddy_special_air_s_jump_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 14.0);
    if is_excute(agent) {
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 2.5, 4.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_GA);
        CATCH(agent, 1, Hash40::new("top"), 4.0, 0.0, 5.5, 4.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_GA);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Down Special Laugh Sound
unsafe extern "C" fn ssbexo_diddy_special_lw_laugh_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(boma, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

pub fn install() {
    Agent::new("diddy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnblow", ssbexo_diddy_neutral_special_explode_acmd, Low)
    .acmd("game_specialairnblow", ssbexo_diddy_neutral_special_explode_acmd, Low)
    .acmd("game_specialairsjump", ssbexo_diddy_special_air_s_jump_acmd, Low)
    .acmd("sound_speciallwlaugh", ssbexo_diddy_special_lw_laugh_sound, Low)
    .acmd("sound_specialairlwlaugh", ssbexo_diddy_special_lw_laugh_sound, Low)
    .install()
    ;
}