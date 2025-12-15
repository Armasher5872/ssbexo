use super::*;

unsafe extern "C" fn gaogaen_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        let last_strans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
                if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
                    if last_strans != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                    else {
                        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                        else {
                            WorkModule::set_int64(fighter.module_accessor, hash40("throw_lw") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                        }
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                    }
                    else {
                        WorkModule::set_int64(fighter.module_accessor, hash40("throw_hi") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                    }
                }
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
                }
                else {
                    WorkModule::set_int64(fighter.module_accessor, hash40("throw_b") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
                }
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE) {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f_revenge") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, hash40("throw_f") as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);   
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, gaogaen_catch_wait_end_status)
    .install()
    ;
}