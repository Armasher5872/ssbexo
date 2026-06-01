use super::*;

unsafe extern "C" fn krool_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_start"), L2CValue::Hash40s("special_air_lw_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn krool_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_lw_start"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS) {
        let blunderbuss_boma = get_article_boma(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS);
        LinkModule::set_model_constraint_pos_ort(blunderbuss_boma, *LINK_NO_CONSTRAINT, Hash40::new("top"), Hash40::new("havel"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_rot_offset(blunderbuss_boma, &Vector3f{x: 0.0, y: -90.0*lr, z: 180.0});
    }
    if MotionModule::is_end(boma) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
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
    let boma = fighter.module_accessor;
    if ![*FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(boma, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(boma, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
        WorkModule::set_int(boma, 90, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn krool_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![*FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_LAUNCH].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BLUNDERBUSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::off_flag(boma, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        WorkModule::set_float(boma, 0.0, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_CHARGE);
        WorkModule::set_int(boma, 0, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_ANGLE);
        WorkModule::set_int(boma, 90, *FIGHTER_KROOL_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    0.into()
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, krool_special_lw_exit_status)
    .install()
    ;
}