use super::*;

/*   CATCH STATUS SCRIPTS   */

unsafe extern "C" fn luigi_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Catch()
}

unsafe extern "C" fn luigi_catch_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDash()
}

unsafe extern "C" fn luigi_catch_turn_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchTurn()
}

unsafe extern "C" fn luigi_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | 0) as u32, 0, 0);
        0.into()
    }
    else {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | 0) as u32, 0, 0);
        0.into()
    }
}

unsafe extern "C" fn luigi_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 0.0, false, 0.0, false, false);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, *WEAPON_LUIGI_PLUNGER_STATUS_KIND_PULL, ArticleOperationTarget(0));
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_catchcommon"), -1);
        fighter.status_CatchPull_common(hash40("catch_wait").into());
        fighter.sub_shift_status_main(L2CValue::Ptr(luigi_catch_pull_loop as *const () as _))
    }
    else {
        fighter.status_CatchPull()
    }
}

unsafe extern "C" fn luigi_catch_pull_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        let throw_f_status_kind = fighter.global_table[THROW_F_STATUS_KIND].get_i32();
        fighter.change_status(throw_f_status_kind.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    let mut condition: bool = true;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let catch_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    */
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP {
        /*
        fighter.status_end_CatchPull();
        if status_kind != *FIGHTER_STATUS_KIND_THROW {
            if ![*FIGHTER_STATUS_KIND_CATCH_WAIT, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
                if condition {
                    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(0));
                }
                ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(0));
            }
            else {
                condition = false;
            }
        }
        else {
            if ![hash40("throw_f"), hash40("throw_b"), hash40("throw_hi"), hash40("throw_lw"), hash40("catch_attack")].contains(&catch_motion_kind) {
                return 0.into();
            }
            condition = false;
        }
        */
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        0.into()
    }
    else {
        fighter.status_end_CatchPull()
    }
    //fighter.status_end_CatchPull()
}

unsafe extern "C" fn luigi_catch_dash_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchDashPull()
}

unsafe extern "C" fn luigi_catch_dash_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDashPull()
}

unsafe extern "C" fn luigi_catch_wait_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchWait()
}

unsafe extern "C" fn luigi_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchWait()
}

unsafe extern "C" fn luigi_catch_cut_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchCut()
}

unsafe extern "C" fn luigi_catch_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchAttack()
}

/*   THROW STATUS SCRIPTS   */

unsafe extern "C" fn luigi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, (*FIGHTER_LOG_MASK_FLAG_THROW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64, *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32, *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 0);
        0.into()
    }
    else {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, (*FIGHTER_LOG_MASK_FLAG_THROW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64, *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32, *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 0);
        0.into()
    }
}

unsafe extern "C" fn luigi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw"), L2CValue::Hash40s("special_air_lw_throw"), false.into());
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_f"), false, -1.0);
        fighter.sub_shift_status_main(L2CValue::Ptr(luigi_throw_loop as *const () as _))
    }
    else {
        fighter.status_Throw()
    }
}

unsafe extern "C" fn luigi_throw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    fighter.status_end_Throw();
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    delete_plunger(global_fighter, false);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    if LUIGI_CYCLONE_RNG[entry_id] > 1 {
        LUIGI_CYCLONE_RNG[entry_id] -= 2;
    }
    0.into()
}

/*   SIDE B (Now Luigi Cyclone) STATUS SCRIPTS   */

unsafe extern "C" fn luigi_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_s_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if LUIGI_CYCLONE_RNG[entry_id] > 1 {
        let rand_num = sv_math::rand(hash40("fighter"), LUIGI_CYCLONE_RNG[entry_id]);
        if rand_num == 1 {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
    }
    fun_71000102b0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_s_loop as *const () as _))
}

unsafe extern "C" fn fun_71000102b0(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {      
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
        fighter.set_back_cliff_hangdata(12.0, 12.0);
        fighter.set_front_cliff_hangdata(12.0, 12.0);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
    }
}

unsafe extern "C" fn luigi_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let mut bool_check = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        let int_mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != int_mtrans {
            if situation_kind == int_mtrans {
                bool_check = true;
            }
        }
    }
    if bool_check {
        fun_71000102b0(fighter);
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC) {
        let float_limit_x_dec = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let param_limit_x_dec = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("limit_x_dec") as u64);
        let mut sum_limit_x_dec = float_limit_x_dec+param_limit_x_dec;
        WorkModule::set_float(fighter.module_accessor, sum_limit_x_dec, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let mut alstack128; //not sure why this is initialized with a value never used
        let kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
        if kinetic_type != *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_LW {
            let air_limit_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("air_limit_x") as u64);
            alstack128 = air_limit_x;
        }
        else {
            let limit_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("limit_x") as u64);
            alstack128 = limit_x;
        }
        if sum_limit_x_dec < alstack128 {
            alstack128 = 0.0;
        }
        else {
            alstack128 -= sum_limit_x_dec;
        }
        smash::app::lua_bind::KineticEnergyNormal::set_limit_speed(control_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: alstack128, y: 0.0});
    }
    let mut float_rise_y: f32;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY) {
        float_rise_y = 0.0;
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                float_rise_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("buoyancy") as u64);
                let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
                if situation_kind == *SITUATION_KIND_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_LW);
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                }
            }
            else {
                float_rise_y = 0.0;
            }
        }
        else {
            float_rise_y = 0.0;
        }
    }
    WorkModule::set_float(fighter.module_accessor, float_rise_y, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_RISE_Y);
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
    LUIGI_CYCLONE_RNG[entry_id] = 9;
    0.into()
}

unsafe extern "C" fn luigi_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SPECIAL S CHARGE STATUS SCRIPTS   */
//0'd out to prevent any Green Missile logic from leaking into Luigi Cyclone logic

unsafe extern "C" fn luigi_special_s_charge_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_charge_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_charge_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_charge_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_charge_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_charge_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SPECIAL S BREATH STATUS SCRIPTS   */
//0'd out to prevent any Green Missile logic from leaking into Luigi Cyclone logic

unsafe extern "C" fn luigi_special_s_breath_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_breath_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_breath_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_breath_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_breath_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_breath_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SPECIAL S RAM STATUS SCRIPTS   */
//0'd out to prevent any Green Missile logic from leaking into Luigi Cyclone logic

unsafe extern "C" fn luigi_special_s_ram_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SPECIAL S END STATUS SCRIPTS   */
//0'd out to prevent any Green Missile logic from leaking into Luigi Cyclone logic

unsafe extern "C" fn luigi_special_s_end_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   SPECIAL S WALL STATUS SCRIPTS   */
//0'd out to prevent any Green Missile logic from leaking into Luigi Cyclone logic

unsafe extern "C" fn luigi_special_s_wall_pre_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_main_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_wall_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   DOWN B STATUS SCRIPTS   */

unsafe extern "C" fn luigi_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//New Down B Status Kind, deals with the looping portion

unsafe extern "C" fn luigi_special_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch"), false, -1.0);
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 12.0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 12.0);
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_loop"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_loop"), -1.0, 1.0, 0.0, false, false);
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    let mut deleter_status_check: bool = true;
    let mut remover_status_check: bool = true;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    fighter.status_end_Catch();
    if status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL {
        remover_status_check = false;
        deleter_status_check = false;
    }
    if remover_status_check {
        let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
        ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    }
    if deleter_status_check {
        let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
        delete_plunger(global_fighter, false);
    }
    */
    0.into()
}

//New Down B Status Kind, deals with the projectile

unsafe extern "C" fn luigi_special_lw_shoot_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_shoot_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 14.0);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_end"), L2CValue::Hash40s("special_air_lw_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_shoot_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_shoot_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_shoot_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    delete_plunger(global_fighter, false);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    0.into()
}

//New Down B Status Kind, deals with the end of the move

unsafe extern "C" fn luigi_special_lw_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, 14.0);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_end"), L2CValue::Hash40s("special_air_lw_end"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_lw_end_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_lw_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_lw_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_OBAKYUMU_OBJECT_ID);
    ArticleModule::remove_exist_object_id(fighter.module_accessor, object_id as u32);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    delete_plunger(global_fighter, false);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(End, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH, luigi_catch_dash_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_TURN, luigi_catch_turn_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_main_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_CUT, luigi_catch_cut_end_status)
    .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, luigi_catch_attack_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, luigi_throw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, luigi_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, luigi_throw_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_exit_status)
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE, luigi_special_s_charge_exit_status)
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_BREATH, luigi_special_s_breath_exit_status)
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_exit_status)
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, luigi_special_s_end_exit_status)
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_pre_status)
    .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_init_status)
    .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_main_status)
    .status(Exec, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_exec_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_end_status)
    .status(Exit, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, luigi_special_s_wall_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, luigi_special_lw_exit_status)
    .status(Pre, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_pre_status)
    .status(Main, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_main_status)
    .status(End, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, luigi_special_lw_loop_end_status)
    .status(Pre, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_SHOOT, luigi_special_lw_shoot_pre_status)
    .status(Main, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_SHOOT, luigi_special_lw_shoot_main_status)
    .status(End, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_SHOOT, luigi_special_lw_shoot_end_status)
    .status(Pre, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_pre_status)
    .status(Main, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_main_status)
    .status(End, FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_END, luigi_special_lw_end_end_status)
    .install()
    ;
}