use super::*;

unsafe extern "C" fn luigi_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_CatchPull()
}

unsafe extern "C" fn luigi_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_catch_pull"), 0.0, 0.0, false, 0.0, false, false);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, *WEAPON_LUIGI_PLUNGER_STATUS_KIND_PULL, ArticleOperationTarget(0));
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_catchcommon"), -1);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchPull_Main as *const () as _))
    }
    else {
        fighter.status_CatchPull_common(hash40("catch_wait").into());
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchPull_Main as *const () as _))
    }
}

unsafe extern "C" fn luigi_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.status_end_CatchPull();
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        let mut condition: bool = true;
        let catch_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
        if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
            if status_kind == *FIGHTER_STATUS_KIND_THROW {
                if ![hash40("throw_f"), hash40("throw_b"), hash40("throw_hi"), hash40("throw_lw"), hash40("catch_attack")].contains(&catch_motion_kind) {
                    return 0.into();
                }
            }
            condition = false;
            return 0.into()
        }
        else {
            if condition {
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(0));
            }
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(0));
            return 0.into()
        }
    }
    else {
        fighter.status_end_CatchPull()
    }
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end_status)
    .install()
    ;
}