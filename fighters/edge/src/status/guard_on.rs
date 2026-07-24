use super::*;

unsafe extern "C" fn edge_guard_on_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_on_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_guard_on_main_loop as *const () as _))
}

unsafe extern "C" fn edge_guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
        WorkModule::on_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_EFFECT);
    }
    if !fighter.sub_status_guard_on_main_air_common().get_bool()
    && !fighter.sub_guard_cont().get_bool()
    && !fighter.status_guard_main_common().get_bool()  {
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD.into(), false.into());
        }
    }
    if edge_check_valid_wing_enable(boma) {
        fighter.change_status(FIGHTER_EDGE_STATUS_KIND_WING_ACTIVATE.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, edge_guard_on_main_status)
    .install()
    ;
}