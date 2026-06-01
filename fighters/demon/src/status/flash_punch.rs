use super::*;

unsafe extern "C" fn demon_flash_punch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH)
    && !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER)
    && !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH)
    && !WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK) {
        MotionModule::change_motion(boma, Hash40::new_raw(0xbdeb83518), 0.0, 1.0, false, 0.0, false, false); //Standard Flash Punch Anim
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH) {
            MotionModule::change_motion(boma, Hash40::new("attack_13"), 0.0, 1.0, false, 0.0, false, false); //One Two Punch
        }
        else {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER) {
                MotionModule::change_motion(boma, Hash40::new_raw(0xbdeb83518), 0.0, 1.0, false, 0.0, false, false); //Demon Slayer
            }
            else {
                if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH) {
                    MotionModule::change_motion(boma, Hash40::new("attack_stand_4"), 0.0, 1.0, false, 0.0, false, false); //Twin Fang Stature Smash
                }
                else {
                    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK) {
                        MotionModule::change_motion(boma, Hash40::new("twin_fang_double_kick"), 0.0, 1.0, false, 0.0, false, false); //Twin Fang Double Kick
                    }
                }
            }
        }
    }
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_flash_punch_main_loop as *const () as _))
}

unsafe extern "C" fn demon_flash_punch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(boma) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }
        if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH) {
                if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH) {
                    WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH.into(), false.into());
                }
                if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH) {
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH.into(), false.into());
                }
                if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK) {
                    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH.into(), false.into());
                }
                return 0.into();
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_FLASH_PUNCH) {
                if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH) {
                    WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH.into(), false.into());
                }
                return 0.into();
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH) {
            if fighter.global_table[CURRENT_FRAME].get_f32() >= 15.0 {
                CancelModule::enable_cancel(boma);
            }
        }
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn demon_flash_punch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
    }
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK) {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH, demon_flash_punch_main_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH, demon_flash_punch_end_status)
    .install()
    ;
}