use super::*;

//Side Special End ACMD
unsafe extern "C" fn ssbexo_wario_side_special_end_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Side Special End Effect
unsafe extern "C" fn ssbexo_wario_side_special_end_effect(_agent: &mut L2CAgentBase) {}

//Side Special End Sound
unsafe extern "C" fn ssbexo_wario_side_special_end_sound(_agent: &mut L2CAgentBase) {}

//Side Special End Expression
unsafe extern "C" fn ssbexo_wario_side_special_end_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialsend", ssbexo_wario_side_special_end_acmd, Low)
    .acmd("effect_specialsend", ssbexo_wario_side_special_end_effect, Low)
    .acmd("sound_specialsend", ssbexo_wario_side_special_end_sound, Low)
    .acmd("expression_specialsend", ssbexo_wario_side_special_end_expression, Low)
    .acmd("game_specialsairend", ssbexo_wario_side_special_end_acmd, Low)
    .acmd("effect_specialsairend", ssbexo_wario_side_special_end_effect, Low)
    .acmd("sound_specialsairend", ssbexo_wario_side_special_end_sound, Low)
    .acmd("expression_specialsairend", ssbexo_wario_side_special_end_expression, Low)
    .install()
    ;
}