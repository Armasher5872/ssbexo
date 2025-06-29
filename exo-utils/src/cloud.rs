use super::*;

pub unsafe extern "C" fn cloud_can_limit_break(fighter: &mut L2CFighterCommon, required_limit: i32) -> L2CValue {
    let limit_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    let special_wait_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && special_wait_timer <= 0 {
        WorkModule::set_int(fighter.module_accessor, 15, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    }
    if special_wait_timer > 0 {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    }
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && special_wait_timer > 0 
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) 
    && limit_level >= required_limit {
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn cloud_training_mode_features(boma: *mut BattleObjectModuleAccessor) {
    let status_kind = StatusModule::status_kind(boma);
    if smashball::is_training_mode()
    && status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_int(boma, 4, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        WorkModule::set_float(boma, 100.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
    }
}