use super::*;

pub unsafe extern "C" fn littlemac_handle_star_strength(fighter: &mut L2CFighterCommon) {
    let ko_gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let star_punch_strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    let special_held_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && star_punch_strength < 3 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    if special_held_timer == 3 && star_punch_strength != 1 && ko_gauge >= 34.0 {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
    if special_held_timer == 5 && star_punch_strength != 2 && ko_gauge >= 68.0 {
        WorkModule::set_int(fighter.module_accessor, 2, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
    if special_held_timer == 7 && star_punch_strength != 3 && ko_gauge == 100.0 {
        WorkModule::set_int(fighter.module_accessor, 3, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
}

pub unsafe extern "C" fn littlemac_can_cancel_into_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let ko_gauge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let ret;
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        ret = ko_gauge == 100.0 && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    }
    else {
        ret = ko_gauge > 0.0 && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    }
    ret.into()
}

pub unsafe extern "C" fn littlemac_training_mode_features(boma: *mut BattleObjectModuleAccessor) {
    let status_kind = StatusModule::status_kind(boma);
    if smashball::is_training_mode()
    && status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        EffectModule::req_on_joint(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(6.0, 15.0, 0.0), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    }
}