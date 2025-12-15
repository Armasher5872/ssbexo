use super::*;

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
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
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
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_charge"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
        let blunderbuss_boma = get_article_boma(fighter.module_accessor, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS);
        LinkModule::set_model_constraint_pos_ort(blunderbuss_boma, *LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0*lr, z: 180.0});
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

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_pre_status)
    .status(Init, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_init_status)
    .status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_main_status)
    .status(Exec, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_exec_status)
    .status(End, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_end_status)
    .status(Exit, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, krool_special_lw_charge_exit_status)
    .install()
    ;
}