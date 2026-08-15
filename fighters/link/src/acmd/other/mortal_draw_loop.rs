use super::*;

//Mortal Draw Loop ACMD
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_acmd(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Effect
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_effect(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Sound
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_sound(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Expression
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_mortaldrawloop", ssbexo_link_mortal_draw_loop_acmd, Low)
    .acmd("effect_mortaldrawloop", ssbexo_link_mortal_draw_loop_effect, Low)
    .acmd("sound_mortaldrawloop", ssbexo_link_mortal_draw_loop_sound, Low)
    .acmd("expression_mortaldrawloop", ssbexo_link_mortal_draw_loop_expression, Low)
    .install()
    ;
}