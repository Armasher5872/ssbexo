use super::*;

//Neutral Special

unsafe extern "C" fn toonlink_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn toonlink_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_START, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    if max_hold_frame < 0 {
        WorkModule::set_int(fighter.module_accessor, -100, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    0.into()
}

unsafe extern "C" fn toonlink_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let toonlink_bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    if fighter_kind == *FIGHTER_KIND_KIRBY {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
            if copy_chara == *FIGHTER_KIND_TOONLINK {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, -1);
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
            }
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
        }
    }
    else {
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let toonlink_bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_FLOAT_CHARGE);
    let bow_charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("bow_charge_max"));
    let max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    let max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
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
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
            }
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
            }
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
            }
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
            }
            if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
            }
        }
    }
    if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
        if charge < bow_charge_max {
            WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_LINK_STATUS_BOW_WORK_FLOAT_CHARGE);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX);
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
        }
    }
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if max_hold_count >= max_hold_frame {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_SHOOT_WIND) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_FIRST), true);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_SHOOT_WIND);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
            if fighter_kind != *FIGHTER_KIND_KIRBY {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x298585bf83));
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ff4ab9a31));
            }
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
            if situation_kind == *SITUATION_KIND_GROUND {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) && max_hold_count < max_hold_frame {
                if situation_kind == *SITUATION_KIND_GROUND {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
                if situation_kind == *SITUATION_KIND_GROUND {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_end"), true, -1.0);
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if toonlink_bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn toonlink_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn toonlink_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let article_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if motion_kind == hash40("special_n_start") {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_link_special_n01"), 0);
    }
    ArticleModule::remove_exist(fighter.module_accessor, article_id, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

unsafe extern "C" fn toonlink_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let article_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    ArticleModule::remove_exist(fighter.module_accessor, article_id, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    0.into()
}

unsafe extern "C" fn toonlink_bowarrow_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    let lr = PostureModule::lr(weapon.module_accessor);
    let power_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("power_min"));
    let power_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("power_max"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_bowarrow"), hash40("speed_max"));
    let charge = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE);
    let power = power_max-power_min;
    let power_charge = power*charge;
    let lerp_speed = weapon.lerp(speed_min.into(), speed_max.into(), charge.into());
    ModelModule::set_scale(weapon.module_accessor, 0.001);
    AttackModule::set_power_mul(weapon.module_accessor, power_min+power_charge);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(17.0*lr), y: owner_pos_y+6.5, z: owner_pos_z});
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, lerp_speed.get_f32()*lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

//Up Special

unsafe extern "C" fn toonlink_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let rslash_landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_landing_frame"));
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_STATUS_RSLASH_WORK_HOLD_FRAME);
    WorkModule::set_float(fighter.module_accessor, rslash_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
    }
    else {
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("special_air_hi"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let get_kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
    let rslash_charge_spd_div = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_charge_spd_div"));
    let rslash_end_air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_accel_x_mul"));
    let rslash_air_max_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let rslash_end_air_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("rslash_end_air_max_x"));
    let mul_x_speed_max = (rslash_end_air_max_x/air_speed_x_stable)/rslash_air_max_x_mul;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    MotionModule::set_rate(fighter.module_accessor, rslash_charge_spd_div);
    if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
        GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X) {
        if get_kinetic_type == *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI {
            sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, rslash_end_air_accel_x_mul);
            sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, mul_x_speed_max);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND) {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END);
                GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LINK_SPECIAL_AIR_HI);
                fighter.set_situation(SITUATION_KIND_AIR.into());
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_GROUND);
            }
            else {
                fighter.change_status(FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
            }
        }
        else {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_HOLD) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD.into(), true.into());
                    return 1.into();
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_TRANSITION_TERM_ID_END) {
                fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
                return 1.into();
            }
        }
    }
    0.into()
}

//Up Special End

unsafe extern "C" fn toonlink_special_hi_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_hi_end_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_hi_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if prev_situation_kind == *SITUATION_KIND_GROUND && situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_HURRICANE_SLASH_MOVE) {
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.763, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.763, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.15378);
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Up Special Glide

unsafe extern "C" fn toonlink_special_hi_glide_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_glide_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    let limit_y_mul = 0.2;
    let combined_y_speed = air_speed_y_stable*limit_y_mul;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*lr, 0.0);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_DEKU_LEAF_ACTIVE);
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_glide_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("special_hi_glide"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_hi_glide_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_hi_glide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    //let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_attack().get_bool() {
        return 1.into();
    }
    /*
    if stick_x < turn_run_stick_x {
        fighter.change_status(FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_TURN.into(), false.into());
    }
    */
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        //fighter.change_status(FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("special_hi_glide"), false, -1.0);
    }
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_glide_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Up Special Glide Turn

unsafe extern "C" fn toonlink_special_hi_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_turn_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    let limit_y_mul = 0.15;
    let combined_y_speed = air_speed_y_stable*limit_y_mul;
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    PostureModule::reverse_lr(fighter.module_accessor);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, combined_y_speed);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*1.5, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, (air_speed_x_stable*lr)*1.5, 0.0);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_DEKU_LEAF_ACTIVE);
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_turn"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("special_hi_glide_turn"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_hi_turn_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_hi_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_attack().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_turn_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Up Special Glide Landing

unsafe extern "C" fn toonlink_special_hi_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_land"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOW, Hash40::new("special_hi_glide_land"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(toonlink_special_hi_land_main_loop as *const () as _))
}

unsafe extern "C" fn toonlink_special_hi_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn toonlink_special_hi_land_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("toonlink")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, toonlink_special_n_exit_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, toonlink_special_hi_main_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, toonlink_special_hi_end_main_status)
    .status(Pre, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE, toonlink_special_hi_glide_pre_status)
    .status(Init, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE, toonlink_special_hi_glide_init_status)
    .status(Main, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE, toonlink_special_hi_glide_main_status)
    .status(End, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_GLIDE, toonlink_special_hi_glide_end_status)
    .status(Pre, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_TURN, toonlink_special_hi_turn_pre_status)
    .status(Init, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_TURN, toonlink_special_hi_turn_init_status)
    .status(Main, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_TURN, toonlink_special_hi_turn_main_status)
    .status(End, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_TURN, toonlink_special_hi_turn_end_status)
    .status(Pre, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_LAND, toonlink_special_hi_land_pre_status)
    .status(Main, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_LAND, toonlink_special_hi_land_main_status)
    .status(End, *FIGHTER_TOONLINK_STATUS_KIND_SPECIAL_HI_LAND, toonlink_special_hi_land_end_status)
    .install()
    ;
    Agent::new("toonlink_bowarrow")
    .status(Init, *WN_LINK_BOWARROW_STATUS_KIND_FLY, toonlink_bowarrow_fly_init_status)
    .install()
    ;
}