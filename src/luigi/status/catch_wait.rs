use super::*;

unsafe extern "C" fn luigi_catch_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.status_CatchWait_common(hash40("special_lw_catch_pull").into())
    }
    else {
        fighter.status_CatchWait_common(hash40("catch_pull").into())
    }
}

unsafe extern "C" fn luigi_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.status_end_CatchWait();
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        let mut condition: bool = true;
        let catch_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
            if status_kind == *FIGHTER_STATUS_KIND_THROW {
                if ![hash40("throw_f"), hash40("throw_b"), hash40("throw_hi"), hash40("throw_lw"), hash40("catch_attack")].contains(&catch_motion_kind) {
                    return 0.into();
                }
            }
            condition = false;
        }
        if condition {
            let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
            ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
        }
        0.into()
    }
    else {
        fighter.status_end_CatchWait()
    }
}

pub fn install() {
    Agent::new("luigi")
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_end_status)
    .install()
    ;
}