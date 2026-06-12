use super::*;

unsafe extern "C" fn demon_flash_punch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    demon_flash_punch_change_motion(fighter, 0.into());
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_flash_punch_main_loop as *const () as _))
}

unsafe extern "C" fn demon_flash_punch_change_motion(fighter: &mut L2CFighterCommon, combo_index: L2CValue) {
    let boma = fighter.module_accessor;
    let motion_kind;
    WorkModule::set_int(boma, combo_index.get_i32(), *FIGHTER_DEMON_STATUS_ATTACK_COMBO_WORK_INT_COMBO);
    if combo_index.get_i32() != 0 {
        if combo_index.get_i32() != 1 {
            if combo_index.get_i32() != 2 {
                motion_kind = Hash40::new("twin_fang_double_kick");
            }
            else {
                motion_kind = Hash40::new("twin_fang_stature_smash");
            }
        }
        else {
            motion_kind = Hash40::new("demon_slayer");
        }
    }
    else {
        motion_kind = Hash40::new("one_two_punch");
    }
    MotionModule::change_motion(boma, motion_kind, 0.0, 1.0, false, 0.0, false, false);
}

unsafe extern "C" fn demon_flash_punch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
            if combo_count == 0 {
                demon_flash_punch_change_motion(fighter, 2.into());
            }
            if combo_count == 2 {
                demon_flash_punch_change_motion(fighter, 3.into());
            }
        }
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH);
            if combo_count == 0 {
                demon_flash_punch_change_motion(fighter, 1.into());
            }
            if combo_count == 2 {
                demon_flash_punch_change_motion(fighter, 3.into());
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
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH, demon_flash_punch_main_status)
    .install()
    ;
}