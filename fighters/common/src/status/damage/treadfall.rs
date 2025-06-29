use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_TreadFall)]
unsafe extern "C" fn status_treadfall(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_check = false;
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let motion_hash = Hash40::new_raw(0xd88a289d5);
    if motion_kind == motion_hash.hash as u64 {
        if !MotionModule::is_end(fighter.module_accessor) {
            bool_check = true;
        }
    }
    if !bool_check {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x9b5c6425d), 0.0, 1.0, false, 0.0, false, false);
    }
    if prev_status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_tread_fall_uniq_check();
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_tread_fall_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TreadFall_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_treadfall
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}