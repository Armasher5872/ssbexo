use super::*;

unsafe extern "C" fn gaogaen_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let last_strans = WorkModule::get_int(boma, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
    let is_revenge = WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
    let mut catch_motion_kind: u64 = if is_revenge {hash40("throw_f_revenge")} else {hash40("throw_f")};
    if fighter.sub_is_throw_status_kind(status_kind.into()).get_bool() {
        if last_strans == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F {
            if is_revenge {
                catch_motion_kind = hash40("throw_f_revenge");
            }
            else {
                catch_motion_kind = hash40("throw_f");
            }
        }
        if last_strans == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B {
            if is_revenge {
                catch_motion_kind = hash40("throw_b_revenge");
            }
            else {
                catch_motion_kind = hash40("throw_b");
            }
        }
        if last_strans == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI {
            if is_revenge {
                catch_motion_kind = hash40("throw_hi_revenge");
            }
            else {
                catch_motion_kind = hash40("throw_hi");
            }
        }
        if last_strans == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW {
            if is_revenge {
                catch_motion_kind = hash40("throw_lw_revenge");
            }
            else {
                catch_motion_kind = hash40("throw_lw");
            }
        }
        WorkModule::set_int64(boma, catch_motion_kind as i64, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
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