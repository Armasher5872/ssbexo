use super::*;

//Up Special Glide Start ACMD
unsafe extern "C" fn ssbexo_link_special_hi_glide_start_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Glide Start Effect
unsafe extern "C" fn ssbexo_link_special_hi_glide_start_effect(_agent: &mut L2CAgentBase) {}

//Up Special Glide Start Sound
unsafe extern "C" fn ssbexo_link_special_hi_glide_start_sound(_agent: &mut L2CAgentBase) {}

//Up Special Glide Start Expression
unsafe extern "C" fn ssbexo_link_special_hi_glide_start_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiglidestart", ssbexo_link_special_hi_glide_start_acmd, Low)
    .effect_acmd("effect_specialhiglidestart", ssbexo_link_special_hi_glide_start_effect, Low)
    .sound_acmd("sound_specialhiglidestart", ssbexo_link_special_hi_glide_start_sound, Low)
    .expression_acmd("expression_specialhiglidestart", ssbexo_link_special_hi_glide_start_expression, Low)
    .install()
    ;
}