use super::*;

unsafe extern "C" fn link_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_START);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CONTINUE_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_START, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_SHOOT_NUM);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_DOUBLE_COUNT);
    if max_hold_frame < 0 {
        WorkModule::set_int(fighter.module_accessor, -100, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    }
    if fighter_kind == *FIGHTER_KIND_KIRBY {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_LINK {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
            }
        }
    }
    else {
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_LINKARROW {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, false, -1);
            }
        }
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
            let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
                WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_NORMAL_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
            }
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let slow_rate = SlowModule::rate(fighter.module_accessor);
    let bow_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
    let max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
    let charge = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_FLOAT_CHARGE);
    let special_n_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    let bow_double_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_DOUBLE_COUNT);
    let bow_charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("bow_charge_max"));
    let max_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_hold_frame"));
    let second_bowarrow_interval_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("second_bowarrow_interval_frame"));
    let max_degree = 80.0;
    let change_degree_per_frame = 2.5*slow_rate;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
            }
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_end"), true, -1.0);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("air_n_start"), true, -1.0);
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("air_n"), true, -1.0);
            }
            if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("air_n_end"), true, -1.0);
            }
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) {
        if [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) && bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                let bow_arrow_agent = get_weapon_common_from_accessor(&mut *bow_arrow_boma);
                let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                        if arrow_type != *WN_LINK_BOWARROW_ICE_ARROW {
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_ice_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                            LAST_EFFECT_SET_COLOR(bow_arrow_agent, 0.0, 0.0, 1.0);
                            WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_ICE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                        }
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                        if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_light_arrow_charge"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                            WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_LIGHT_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                            if situation_kind != *SITUATION_KIND_GROUND {
                                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 0.19, false, 0.0, false, false);
                                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
                                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                            }
                            else {
                                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 0.19, false, 0.0, false, false);
                                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
                                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                            }
                        }
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                        if arrow_type != *WN_LINK_BOWARROW_FIRE_ARROW {
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_fire_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.6, true);
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                            LAST_EFFECT_SET_COLOR(bow_arrow_agent, 1.0, 0.0, 0.0);
                            WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_FIRE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                        }
                    }
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                        if arrow_type != *WN_LINK_BOWARROW_SHOCK_ARROW {
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                            EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                            LAST_EFFECT_SET_COLOR(bow_arrow_agent, 1.0, 0.0, 1.0);
                            WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_SHOCK_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                        }
                    }
                }
            }
        }
    }
    else {
        if [hash40("special_n_start"), hash40("special_air_n_start"), hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
            if stick_y > 0.6 {
                if special_n_degree < max_degree {
                    WorkModule::set_float(fighter.module_accessor, special_n_degree+change_degree_per_frame, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
                }
            }
            if stick_y < -0.6 {
                if special_n_degree > -max_degree {
                    WorkModule::set_float(fighter.module_accessor, special_n_degree-change_degree_per_frame, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
                }
            }
            change_angle(fighter.module_accessor, special_n_degree, max_degree, "special_n_hi", "special_n_lw");
        }
    }
    if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
        if charge < bow_charge_max {
            WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_LINK_STATUS_BOW_WORK_FLOAT_CHARGE);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE_MAX);
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_MAX_HOLD_COUNT);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
            }
        }
    }
    if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
        change_angle(fighter.module_accessor, special_n_degree, max_degree, "special_n_end_hi", "special_n_end_lw");
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
        let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
        let hold_frame = if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {300} else {max_hold_frame};
        if max_hold_count >= hold_frame {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
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
            link_shoot_arrow(fighter);
        }
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_CHARGE) && arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
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
            link_shoot_arrow(fighter);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE) {
        if bow_double_count == second_bowarrow_interval_frame {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), true);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_FLAG_DOUBLE);
            ItemModule::remove_item(fighter.module_accessor, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_START {
            if fighter_kind != *FIGHTER_KIND_KIRBY {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x298585bf83));
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ff4ab9a31));
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_STEP_END, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
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
                    change_angle(fighter.module_accessor, special_n_degree, max_degree, "special_n_end_hi", "special_n_end_lw");
                    link_shoot_arrow(fighter);
                }
                else {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LINK_STATUS_BOW_WORK_INT_STEP);
                    if situation_kind == *SITUATION_KIND_GROUND {
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n"), true, -1.0);
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else {
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air"), true, -1.0);
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    change_angle(fighter.module_accessor, special_n_degree, max_degree, "special_n_hi", "special_n_lw");
                }
            }
        }
        if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_HOLD {
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
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
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
                change_angle(fighter.module_accessor, special_n_degree, max_degree, "special_n_end_hi", "special_n_end_lw");
                link_shoot_arrow(fighter);
            }
        }
        if bow_step == *FIGHTER_LINK_STATUS_BOW_STEP_END {
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn link_shoot_arrow(fighter: &mut L2CFighterCommon) {
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_FIRST), true);
}

unsafe extern "C" fn link_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let bow_article_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    if fighter_kind != *FIGHTER_KIND_LINK {
        ArticleModule::remove_exist(fighter.module_accessor, bow_article_id, ArticleOperationTarget(0));
    }
    else {
        ArticleModule::change_status_exist(fighter.module_accessor, bow_article_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    }
    if motion_kind == hash40("special_n_start") {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_link_special_n01"), 0);
    }
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    0.into()
}

unsafe extern "C" fn link_special_n_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let bow_article_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOW_ARTICLE_ID);
    if fighter_kind != *FIGHTER_KIND_LINK {
        ArticleModule::remove_exist(fighter.module_accessor, bow_article_id, ArticleOperationTarget(0));
    }
    else {
        ArticleModule::change_status_exist(fighter.module_accessor, bow_article_id, *WN_LINK_BOW_STATUS_KIND_BACK);
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        if ArticleModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
            let item_id = ArticleModule::get_int(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
            LinkModule::remove_model_constraint(item_boma, true);
            if LinkModule::is_link(item_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink_all(item_boma);
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE);
        }
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(0));
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, link_special_n_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, link_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, link_special_n_exit_status)
    .install()
    ;
}