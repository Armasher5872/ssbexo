use super::*;

pub unsafe extern "C" fn miiswordsman_special_s3_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s3_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
  	let dash_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common") as u64, hash40("dash_stick_x") as u64);
  	let dash_flick_x = WorkModule::get_param_int(fighter.module_accessor, hash40("common") as u64, hash40("dash_flick_x") as u64);
    let get_param_dash_s4_frame = FighterControlModuleImpl::get_param_dash_s4_frame(fighter.module_accessor);
  	let get_flick_no_reset_x = ControlModule::get_flick_no_reset_x(fighter.module_accessor);
  	let stick_x = fighter.global_table[STICK_X].get_f32();
  	if (dash_stick_x <= stick_x.abs()) && (get_flick_no_reset_x < ((get_param_dash_s4_frame as i32)+dash_flick_x)) {
  		WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FLICK);
    	notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_HAJIKI_NUM);
  	}
  	0.into()
}

pub unsafe extern "C" fn miiswordsman_special_s3_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s3_1") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s3_1") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION_AIR);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_06 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_s3_start_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_s3_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let ground_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION);
    let air_motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION_AIR);
    let chakram_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_ACTIVE_CHAKRAM_OBJECT_ID);
    let s3_chakram_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s") as u64, hash40("s3_chakram_stick_y") as u64);
    let s3_chakram_angle_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s") as u64, hash40("s3_chakram_angle_max") as u64);
    let get_stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let get_stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let s3_chakram_angle_max_abs_rad = s3_chakram_angle_max.abs().to_radians();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if fun_7100022a30(fighter).get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FIRST) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(ground_motion), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_motion), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FIRST);
            }
        }
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            if fun_7100022a30(fighter).get_bool() {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FIRST) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_motion), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_FIRST);
                }
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_motion), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_REQUEST_GENERATE);
        if chakram_object_id != *BATTLE_OBJECT_ID_INVALID {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_IS_GENERATE);
        }
        else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, false, -1);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_IS_GENERATE);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW) {
        let mut angle: f32 = 0.0;
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_FLAG_CHECK_MOTION_HI_LW);
        if s3_chakram_stick_y < 0.0 {
            angle = get_stick_dir;
            if s3_chakram_angle_max_abs_rad >= angle {
                if angle < -s3_chakram_angle_max_abs_rad {
                    angle = -s3_chakram_angle_max_abs_rad;
                }
            }
            else {
                angle = s3_chakram_angle_max_abs_rad;
            }
        }
        if lr < 0.0 {
            if angle >= 0.0 {
                angle = s3_chakram_angle_max_abs_rad;
            }
            else {
                angle = s3_chakram_angle_max_abs_rad*lr;
            }
        }
        if angle < 0.0 {
            angle = 2.0*s3_chakram_angle_max_abs_rad;
        }
        WorkModule::set_float(fighter.module_accessor, angle+0.0, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_FLOAT_SHOOT_ANGLE);
        if get_stick_y >= s3_chakram_stick_y {
            if get_stick_y < -s3_chakram_stick_y {
                WorkModule::set_int64(fighter.module_accessor, hash40("special_s3_1_lw") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION);
                WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s3_1_lw") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION_AIR);
            }
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_s3_1_hi") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s3_1_hi") as i64, *FIGHTER_MIISWORDSMAN_STATUS_CHAKRAM_WORK_INT_MOTION_AIR);
        }
        if situation_kind != *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_motion), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(ground_motion), -1.0, 1.0, 0.0, false, false);
        }
    }
    0.into()
}

unsafe extern "C" fn fun_7100022a30(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if StatusModule::is_changing(fighter.module_accessor) {
        ret = 1.into();
    }
    else {
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                ret = 0.into();
            }
        }
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                ret = 1.into();
            }
        }
        ret = 0.into();
    }
    ret.into()
}

pub unsafe extern "C" fn miiswordsman_special_s3_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_GENERATE_ARTICLE_CHAKRAM, ArticleOperationTarget(0));
    0.into()
}