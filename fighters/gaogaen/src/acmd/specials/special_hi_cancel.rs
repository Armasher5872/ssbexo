use super::*;

//Up Special Cancel ACMD
unsafe extern "C" fn ssbexo_gaogaen_up_special_cancel_acmd(_agent: &mut L2CAgentBase) {}

//Up Special Cancel Effect
unsafe extern "C" fn ssbexo_gaogaen_up_special_cancel_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 8, 5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

//Up Special Cancel Sound
unsafe extern "C" fn ssbexo_gaogaen_up_special_cancel_sound(_agent: &mut L2CAgentBase) {}

//Up Special Cancel Expression
unsafe extern "C" fn ssbexo_gaogaen_up_special_cancel_expression(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialairhicancel", ssbexo_gaogaen_up_special_cancel_acmd, Low)
    .acmd("effect_specialairhicancel", ssbexo_gaogaen_up_special_cancel_effect, Low)
    .acmd("sound_specialairhicancel", ssbexo_gaogaen_up_special_cancel_sound, Low)
    .acmd("expression_specialairhicancel", ssbexo_gaogaen_up_special_cancel_expression, Low)
    .install()
    ;
}