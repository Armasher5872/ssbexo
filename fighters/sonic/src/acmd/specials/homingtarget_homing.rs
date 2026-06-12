use super::*;

//Homing Effect
unsafe extern "C" fn ssbexo_sonic_homingtarget_homing_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sonic_homing_target"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 0.9, false, 0, 0, 0, 0, 0, false, false);
        WorkModule::set_int(boma, handle as i32, *WEAPON_SONIC_HOMINGTARGET_INSTANCE_WORK_ID_INT_TARGET_EFFECT_HANDLE);
        EffectModule::set_rgb(boma, handle as u32, 1.0, 0.0, 0.0);
        EffectModule::set_alpha(boma, handle as u32, 0.5);
    }
    frame(lua_state, 120.0);
    if is_excute(agent) {
        EffectModule::set_rgb(boma, WorkModule::get_int(boma, *WEAPON_SONIC_HOMINGTARGET_INSTANCE_WORK_ID_INT_TARGET_EFFECT_HANDLE) as u32, 0.0, 1.0, 0.0);
    }
}

pub fn install() {
    Agent::new("sonic_homingtarget")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("effect_homing", ssbexo_sonic_homingtarget_homing_effect, Low)
    .install()
    ;
}