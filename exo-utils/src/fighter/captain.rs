use super::*;

pub unsafe extern "C" fn captain_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
    WorkModule::set_int(boma, 0, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_INT_SPECIAL_N_EFFECT_HANDLE);
}

pub unsafe extern "C" fn captain_training_mode_features(boma: *mut BattleObjectModuleAccessor) {
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let status_kind = StatusModule::status_kind(boma);
    if smashball::is_training_mode()
    && status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
        PLAY_SE(agent, Hash40::new("vc_captain_boost"));
        WorkModule::on_flag(boma, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_STORED);
        FighterUtil::flash_eye_info(boma);
    }
}