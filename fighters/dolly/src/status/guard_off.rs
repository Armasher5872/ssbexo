use super::*;

unsafe extern "C" fn dolly_guard_off_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let rate = fighter.status_GuardOff_Common().get_f32();
    let shield_radius = WorkModule::get_param_float(boma, hash40("shield_radius"), 0);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) {
        MotionModule::change_motion(boma, Hash40::new("guard_damage"), 2.0, 0.0, false, 0.0, false, false);
        EffectModule::req_follow(boma, Hash40::new("sys_genesis_end"), Hash40::new("throw"), &Vector3f::zero(), &Vector3f::zero(), shield_radius*0.06, true, *EFFECT_SUB_ATTRIBUTE_NONE as u32, 0, 0, *EFFECT_FLIP_NONE, 0, false, false);
        EffectModule::set_rate_last(boma, 1.2);
        EffectModule::req_common(boma, Hash40::new("just_shield"), 0.0);
        let just_shield_se = FighterUtil::get_just_shield_se(fighter.global_table[FIGHTER_KIND].get_i32());
        let se_handle = SoundModule::play_se(boma, just_shield_se, true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(boma, se_handle as i32, 0.9, 0);
        SoundModule::stop_se(boma, Hash40::new("se_common_guardon"), 0);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("guard_off"), 0.0, rate, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_guard_off_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_guard_off_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff_Main()
}

pub fn install() {
    Agent::new("dolly")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_OFF, dolly_guard_off_main_status)
    .install()
    ;
}