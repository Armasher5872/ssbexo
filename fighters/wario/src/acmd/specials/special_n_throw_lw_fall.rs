use super::*;

//Neutral Special Down Throw Fall ACMD
unsafe extern "C" fn ssbexo_wario_neutral_special_throw_lw_fall_acmd(_agent: &mut L2CAgentBase) {}

//Neutral Special Down Throw Fall Effect
unsafe extern "C" fn ssbexo_wario_neutral_special_throw_lw_fall_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("rot"), 0, 20, 0, 90, 0, 0, 1.2, true);
    }
}

//Neutral Special Down Throw Fall Sound
unsafe extern "C" fn ssbexo_wario_neutral_special_throw_lw_fall_sound(_agent: &mut L2CAgentBase) {}

//Neutral Special Down Throw Fall Expression
unsafe extern "C" fn ssbexo_wario_neutral_special_throw_lw_fall_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairnthrowlwfall", ssbexo_wario_neutral_special_throw_lw_fall_acmd, Low)
    .effect_acmd("effect_specialairnthrowlwfall", ssbexo_wario_neutral_special_throw_lw_fall_effect, Low)
    .sound_acmd("sound_specialairnthrowlwfall", ssbexo_wario_neutral_special_throw_lw_fall_sound, Low)
    .expression_acmd("expression_specialairnthrowlwfall", ssbexo_wario_neutral_special_throw_lw_fall_expression, Low)
    .install()
    ;
}