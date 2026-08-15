use super::*;

//Punisher Jump Squat Expression
unsafe extern "C" fn ssbexo_cloud_punisher_jumpsquat_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_punishjumpsquat", ssbexo_cloud_punisher_jumpsquat_expression, Low)
    .install()
    ;
}