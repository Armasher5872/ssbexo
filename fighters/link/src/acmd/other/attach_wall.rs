use super::*;

//Wall Cling Sound
unsafe extern "C" fn ssbexo_link_wall_cling_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        STOP_SE(agent, Hash40::new("se_link_step_right_s_ft"));
    }
}

//Wall Cling Expression
unsafe extern "C" fn ssbexo_link_wall_cling_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_attachwall", ssbexo_link_wall_cling_sound, Priority::Low)
    .acmd("expression_attachwall", ssbexo_link_wall_cling_expression, Priority::Low)
    .install()
    ;
}