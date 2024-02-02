use super::*;

pub unsafe extern "C" fn miiswordsman_special_lw2_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw2_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("special_lw2") {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HOP);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HOP);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        let lw2_start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw2_start_mul_spd_x") as u64);
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR,  get_sum_speed_x*lw2_start_mul_spd_x, 0.0, 0.0, 0.0, 0.0);
        let lw2_start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw2_start_air_acl_x"));
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, lw2_start_air_acl_x);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, get_sum_speed_y, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw2_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_11 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_11 - 1);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw2"), L2CValue::Hash40s("special_air_lw2"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_lw2_start_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_lw2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (!fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_CONTINUE) {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw2"), -1.0, 1.0, 0.0);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_CONTINUE);
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_CONTINUE) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw2"), -1.0, 1.0, 0.0);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_CONTINUE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw2_start_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable") as u64, 0);
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y") as u64, 0);
    let lw2_attack_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw2_attack_spd_y") as u64);
    let lw2_hit_xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw2_hit_xlu_frame") as u64);
    let lw2_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw2_attack_acl_y") as u64);
    let lw2_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("lw2_attack_max_y") as u64);
    let after_xlu_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_INT_HIT_AFTER_XLU_FRAME);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_SPECIAL_FALL) {
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable);
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HOP) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HOP);
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_REVERSE_SLASH_HOP) {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, lw2_attack_spd_y);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_REVERSE_SLASH_HOP);
                }
            }
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -lw2_attack_acl_y);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, lw2_attack_max_y);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HIT) {
        WorkModule::set_int(fighter.module_accessor, lw2_hit_xlu_frame, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_INT_HIT_AFTER_XLU_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_FLAG_HIT);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        if after_xlu_frame > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_REVERSE_SLASH_INT_HIT_AFTER_XLU_FRAME);
        }
        if after_xlu_frame <= 0 {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_lw2_start_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}