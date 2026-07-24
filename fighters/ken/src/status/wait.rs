use super::*;

unsafe extern "C" fn ken_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_wait_main_loop as *const () as _))
}

unsafe extern "C" fn ken_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let opponent_lr_1on1 = WorkModule::get_float(boma, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if opponent_lr_1on1 != 0.0 {
        let lr = PostureModule::lr(boma);
        if opponent_lr_1on1 == lr {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK) {
                let global_stick_x = fighter.global_table[STICK_X].get_f32();
                let stick_x = -(global_stick_x*lr);
                let walk_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("walk_stick_x"));
                if walk_stick_x <= stick_x {
                    let stick_y = fighter.global_table[STICK_Y].get_f32();
                    let squat_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y"));
                    if squat_stick_y < stick_y {
                        fighter.change_status(FIGHTER_RYU_STATUS_KIND_WALK_BACK.into(), true.into());
                        return 0.into();
                    }
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ken")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_WAIT, ken_wait_main_status)
    .install()
    ;
}