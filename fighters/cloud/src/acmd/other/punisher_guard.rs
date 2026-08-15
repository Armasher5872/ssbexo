use super::*;

//Punisher Guard Expression
unsafe extern "C" fn ssbexo_cloud_punisher_guard_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("sword") as i64, hash40("sword_r") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("expression_punishguard", ssbexo_cloud_punisher_guard_expression, Low)
    .install()
    ;
}