use super::*;

unsafe extern "C" fn krool_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_lw4"), smash_restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_attack_lw4_main_loop as *const () as _))   
}

unsafe extern "C" fn krool_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let start_air_frame = 2.0;
    let fall_loop_frame = 20.0;
    let landing_frame = 21.0;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if current_frame == start_air_frame {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if current_frame >= fall_loop_frame {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -11.0);
            MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if current_frame >= fall_loop_frame && current_frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
                SA_SET(fighter, *SITUATION_KIND_AIR);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -11.0);
                sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn krool_special_hi_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_start"), L2CValue::Hash40s("special_air_lw_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn krool_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
        let blunderbuss_boma = get_article_boma(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS);
        LinkModule::set_model_constraint_pos_ort(blunderbuss_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        if lr == 1.0 {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 180.0});
        }
        else {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 90.0});
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
        }
        return 1.into();
    }
    0.into()   
}

unsafe extern "C" fn krool_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn krool_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH].contains(&status_kind) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_charge_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_lw_charge_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.01);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.01);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_charge_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_charge"), L2CValue::Hash40s("special_air_lw_charge"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_lw_charge_main_loop as *const () as _))
}

unsafe extern "C" fn krool_special_lw_charge_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_charge"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_charge"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
        let blunderbuss_boma = get_article_boma(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS);
        LinkModule::set_model_constraint_pos_ort(blunderbuss_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        if lr == 1.0 {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 180.0});
        }
        else {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 90.0});
        }
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH.into(), false.into());
        return 1.into();
    }
    0.into()   
}

unsafe extern "C" fn krool_special_lw_charge_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::add_float(fighter.module_accessor, 1.0/60.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    0.into()
}

unsafe extern "C" fn krool_special_lw_charge_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_charge_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_launch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_lw_launch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_launch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_launch"), L2CValue::Hash40s("special_air_lw_launch"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_lw_launch_main_loop as *const () as _))
}

unsafe extern "C" fn krool_special_lw_launch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_launch"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
        let blunderbuss_boma = get_article_boma(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS);
        LinkModule::set_model_constraint_pos_ort(blunderbuss_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        if lr == 1.0 {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 180.0});
        }
        else {
            LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0, z: 90.0});
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let lr = PostureModule::lr(fighter.module_accessor);
        let special_lw_charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        let speed = 2.5+special_lw_charge;
        let stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
        let vec_stick_x = stick["x"].get_f32();
        let vec_stick_y = stick["y"].get_f32();
        let stick_angle = vec_stick_y.atan2(vec_stick_x);
        let stick_degrees = stick_angle.to_degrees();
        let degrees = if stick_degrees > 65.0 {65.0} else if stick_degrees < 15.0 {15.0} else {stick_degrees};
        let speed_x = ((degrees+90.0).to_radians().sin()*speed)*lr;
        let speed_y = (degrees-90.0).to_radians().cos()*speed;
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x*0.015);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
        WorkModule::set_int(fighter.module_accessor, degrees as i32, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_launch"), -1.0, 1.0, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
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

unsafe extern "C" fn krool_special_lw_launch_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn krool_special_lw_launch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    0.into()
}

unsafe extern "C" fn krool_special_lw_launch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
    0.into()
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, krool_attack_lw4_main_status)
    .status(Pre, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, krool_special_hi_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_exit_status)
    .status(Pre, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_pre_status)
    .status(Init, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_init_status)
    .status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_main_status)
    .status(Exec, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_exec_status)
    .status(End, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_end_status)
    .status(Exit, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_exit_status)
    .status(Pre, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_pre_status)
    .status(Init, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_init_status)
    .status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_main_status)
    .status(Exec, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_exec_status)
    .status(End, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_end_status)
    .status(Exit, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH, krool_special_lw_launch_exit_status)
    .install()
    ;
}