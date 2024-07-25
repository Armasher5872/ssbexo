use super::*;

unsafe extern "C" fn luigi_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | 0) as u32, 0, 0);
        0.into()
    }
    else {
        fighter.status_pre_CatchPull()
    }
}

unsafe extern "C" fn luigi_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_catch"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch_pull"), false, -1.0);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_catchcommon"), -1);
        fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_catch_pull_main_loop as *const () as _))
    }
    else {
        fighter.status_CatchPull_common(hash40("catch_wait").into());
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_CatchPull_Main as *const () as _))
    }
}

unsafe extern "C" fn luigi_special_lw_catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if stick_y > 0.6 {
        WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
    else if stick_x < -0.6 {
        WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchPull()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end_status)
    .install()
    ;
}