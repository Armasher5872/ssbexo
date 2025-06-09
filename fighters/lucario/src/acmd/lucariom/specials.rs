use super::*;

//Mega Lucario Mega Evolve ACMD
unsafe extern "C" fn ssbexo_lucariom_mega_evolve_acmd(_agent: &mut L2CAgentBase) {}

//Mega Lucario Mega Evolve Effect
unsafe extern "C" fn ssbexo_lucariom_mega_evolve_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_lucario_final"), false, false, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("lucario_final_hadou_l"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, false);
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("lucario_final_hadou_l"), Hash40::new("haver"), 1, 0, 0, 0, 0, 0, 1, false);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("lucario_final_hadou_l"), Hash40::new("havel"), 0, 0, 1, 180, 0, 180, 1, false);
            EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("lucario_final_hadou_l"), Hash40::new("haver"), 1, 0, 0, 180, 0, 180, 1, false);
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("lucario_final_megasymbol"), Hash40::new("top"), 6, 23, -13, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//Mega Lucario Mega Evolve Sound
unsafe extern "C" fn ssbexo_lucariom_mega_evolve_sound(_agent: &mut L2CAgentBase) {}

//Mega Lucario Mega Evolve Expression
unsafe extern "C" fn ssbexo_lucariom_mega_evolve_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("lucario_lucariom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_megaevolve", ssbexo_lucariom_mega_evolve_acmd, Low)
    .game_acmd("game_megaevolveair", ssbexo_lucariom_mega_evolve_acmd, Low)
    .effect_acmd("effect_megaevolve", ssbexo_lucariom_mega_evolve_effect, Low)
    .effect_acmd("effect_megaevolveair", ssbexo_lucariom_mega_evolve_effect, Low)
    .sound_acmd("sound_megaevolve", ssbexo_lucariom_mega_evolve_sound, Low)
    .sound_acmd("sound_megaevolveair", ssbexo_lucariom_mega_evolve_sound, Low)
    .expression_acmd("expression_megaevolve", ssbexo_lucariom_mega_evolve_expression, Low)
    .expression_acmd("expression_megaevolveair", ssbexo_lucariom_mega_evolve_expression, Low)
    .install()
    ;
}