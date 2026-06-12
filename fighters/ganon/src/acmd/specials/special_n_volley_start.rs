use super::*;

//Neutral Special Volley Start ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_start_acmd(_agent: &mut L2CAgentBase) {}

//Grounded Neutral Special Volley Start Effect
unsafe extern "C" fn ssbexo_ganon_grounded_neutral_special_volley_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let effect = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new("haver"), &Vector3f{x: 4.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 3.8, true, 0, 0, 0, 0, 0, true, true);
        WorkModule::set_int(boma, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
}

//Aerial Neutral Special Volley Start Effect
unsafe extern "C" fn ssbexo_ganon_aerial_neutral_special_volley_start_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footr"), 0, 0, 0, -0.286, -45, 25, 1, true);
        EFFECT_FOLLOW(agent, Hash40::new("ganon_engokua_flash"), Hash40::new("footl"), 0, 0, 0, -0.286, -45, 25, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        let effect = EffectModule::req_follow(boma, Hash40::new("ganon_volley"), Hash40::new("haver"), &Vector3f{x: 4.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 3.8, true, 0, 0, 0, 0, 0, true, true);
        WorkModule::set_int(boma, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
}

//Neutral Special Volley Start Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_start_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_n03"));
    }
}

//Neutral Special Volley Start Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_volley_start_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialnvolleystart", ssbexo_ganon_neutral_special_volley_start_acmd, Low)
    .acmd("effect_specialnvolleystart", ssbexo_ganon_grounded_neutral_special_volley_start_effect, Low)
    .acmd("sound_specialnvolleystart", ssbexo_ganon_neutral_special_volley_start_sound, Low)
    .acmd("expression_specialnvolleystart", ssbexo_ganon_neutral_special_volley_start_expression, Low)
    .acmd("game_specialairnvolleystart", ssbexo_ganon_neutral_special_volley_start_acmd, Low)
    .acmd("effect_specialairnvolleystart", ssbexo_ganon_aerial_neutral_special_volley_start_effect, Low)
    .acmd("sound_specialairnvolleystart", ssbexo_ganon_neutral_special_volley_start_sound, Low)
    .acmd("expression_specialairnvolleystart", ssbexo_ganon_neutral_special_volley_start_expression, Low)
    .install()
    ;
}