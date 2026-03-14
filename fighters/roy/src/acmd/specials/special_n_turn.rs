use super::*;

//Neutral Special Turn Effect
unsafe extern "C" fn ssbexo_roy_neutral_special_turn_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_roy_sword3"), Hash40::new("tex_roy_sword4"), 15, Hash40::new("sword1"), 0.0, 0.0, 0.0, Hash40::new("sword1"), -0.0, -0.0, 14.0, true, Hash40::new("roy_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("roy_fire"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("roy_attack_fire"), Hash40::new("sword1"), 0.0, 0.0, 7.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_RND(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }  
}

//Neutral Special Turn Sound
unsafe extern "C" fn ssbexo_roy_neutral_special_turn_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SoundModule::play_status_se(agent.module_accessor, Hash40::new("se_roy_special_n01"), false, false, false);
    }  
}

pub fn install() {
    Agent::new("roy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .effect_acmd("effect_specialnturn", ssbexo_roy_neutral_special_turn_effect, Low)
    .effect_acmd("effect_specialairnturn", ssbexo_roy_neutral_special_turn_effect, Low)
    .sound_acmd("sound_specialnturn", ssbexo_roy_neutral_special_turn_sound, Low)
    .sound_acmd("sound_specialairnturn", ssbexo_roy_neutral_special_turn_sound, Low)
    .install()
    ;
}