use super::*;

unsafe extern "C" fn demon_attack_squat_4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !fun_710002aed0(fighter).get_bool() {
        demon_attack_squat_4_change_motion(fighter, 0.into());
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05);
        MotionModule::set_trans_move_speed_no_scale(boma, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_squat_4_main_loop as *const () as _))
    }
    else {
        return 0.into();
    }
}

unsafe extern "C" fn demon_attack_squat_4_change_motion(fighter: &mut L2CFighterCommon, combo_index: L2CValue) {
    let boma = fighter.module_accessor;
    let motion_kind;
    WorkModule::set_int(boma, combo_index.get_i32(), *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    if combo_index.get_i32() != 0 {
        motion_kind = Hash40::new("attack_hi32");
        fighter.clear_lua_stack();
        sv_kinetic_energy::set_motion_energy_update_flag(fighter.lua_state_agent);
    }
    else {
        motion_kind = Hash40::new("attack_squat_4");
    }
    MotionModule::change_motion(boma, motion_kind, 0.0, 1.0, false, 0.0, false, false);
}

unsafe extern "C" fn demon_attack_squat_4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let combo_count = WorkModule::get_int(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_HI_3_FLAG_INC_STEP);
            if combo_count == 0 {
                demon_attack_squat_4_change_motion(fighter, 1.into());
            }
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_4, demon_attack_squat_4_main_status)
    .install()
    ;
}