use super::*;

//Up Special Start ACMD
unsafe extern "C" fn ssbexo_link_special_hi_start_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Start Effect
unsafe extern "C" fn ssbexo_link_special_hi_start_effect(_agent: &mut L2CAgentBase) {}

//Up Special Start Sound
unsafe extern "C" fn ssbexo_link_special_hi_start_sound(_agent: &mut L2CAgentBase) {}

//Up Special Start Expression
unsafe extern "C" fn ssbexo_link_special_hi_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhistart", ssbexo_link_special_hi_start_acmd, Low)
    .game_acmd("game_specialairhistart", ssbexo_link_special_hi_start_acmd, Low)
    .effect_acmd("effect_specialhistart", ssbexo_link_special_hi_start_effect, Low)
    .effect_acmd("effect_specialairhistart", ssbexo_link_special_hi_start_effect, Low)
    .sound_acmd("sound_specialhistart", ssbexo_link_special_hi_start_sound, Low)
    .sound_acmd("sound_specialairhistart", ssbexo_link_special_hi_start_sound, Low)
    .expression_acmd("expression_specialhistart", ssbexo_link_special_hi_start_expression, Low)
    .expression_acmd("expression_specialairhistart", ssbexo_link_special_hi_start_expression, Low)
    .install()
    ;
}