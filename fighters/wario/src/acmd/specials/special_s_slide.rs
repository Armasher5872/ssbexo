use super::*;

//Side Special Slide ACMD
unsafe extern "C" fn ssbexo_wario_side_special_slide_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Side Special Slide Effect
unsafe extern "C" fn ssbexo_wario_side_special_slide_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

//Side Special Slide Sound
unsafe extern "C" fn ssbexo_wario_side_special_slide_sound(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        SoundModule::play_se(boma, Hash40::new("se_wario_special_s05"), true, false, false, false, smash::app::enSEType(0));
    }
}

//Side Special Slide Expression
unsafe extern "C" fn ssbexo_wario_side_special_slide_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 9);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialsslide", ssbexo_wario_side_special_slide_acmd, Low)
    .acmd("effect_specialsslide", ssbexo_wario_side_special_slide_effect, Low)
    .acmd("sound_specialsslide", ssbexo_wario_side_special_slide_sound, Low)
    .acmd("expression_specialsslide", ssbexo_wario_side_special_slide_expression, Low)
    .install()
    ;
}