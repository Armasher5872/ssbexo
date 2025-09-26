use super::*;

unsafe extern "C" fn littlemac_special_s_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let pos = *PostureModule::pos(fighter.module_accessor);
    let special_s_jump_x_speed_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_x_speed_"));
    let special_s_jump_y_speed_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_y_speed_"));
    let special_s_jump_x_speed_air_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_x_speed_air_"));
    let special_s_jump_y_speed_air_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_y_speed_air_"));
    let special_s_jump_brake_x_ = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_brake_x_"));
    let special_s_end_landing_frame_ = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_end_landing_frame_"));
    let special_s_jump_landing_disable_frame_ = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_landing_disable_frame_"));
    let air_accel_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_AIR_ACCEL_Y);
    let x_speed;
    let y_speed;
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;
    let mut vector = fighter.Vector3__create(x.into(), y.into(), z.into());
    let vec_y = vector["y"].get_f32();
    littlemac_mtrans_smpl_off_flag(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT);
    if situation_kind != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_jump"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        x_speed = special_s_jump_x_speed_air_*lr;
        y_speed = special_s_jump_y_speed_air_;
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_jump"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        x_speed = special_s_jump_x_speed_*lr;
        y_speed = special_s_jump_y_speed_;
    }
    fighter.set_situation(SITUATION_KIND_AIR.into());
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_speed, y_speed);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_s_jump_brake_x_, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    WorkModule::set_float(fighter.module_accessor, special_s_end_landing_frame_ as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::set_int(fighter.module_accessor, special_s_jump_landing_disable_frame_, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        littlemac_special_s_jump_sub_status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(littlemac_special_s_jump_sub_status as *const () as _));
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    vector["x"].assign(&L2CValue::F32(pos.x));
    vector["y"].assign(&L2CValue::F32(pos.y));
    vector["z"].assign(&L2CValue::F32(pos.z));
    WorkModule::set_float(fighter.module_accessor, vec_y, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
    fighter.sub_shift_status_main(L2CValue::Ptr(littlemac_special_s_jump_main_loop as *const () as _))
}

unsafe extern "C" fn littlemac_special_s_jump_sub_status(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if bool_check.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    }
    0.into()
}

unsafe extern "C" fn littlemac_special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let star_punch_strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    let disable_landing_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_LANDING_FRAME);
    let special_s_jump_attack_shift_frame_min = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_jump_attack_shift_frame_min"));
    let frame = MotionModule::frame(fighter.module_accessor);
    littlemac_handle_star_strength(fighter);
    if situation_kind == *SITUATION_KIND_GROUND {
        if disable_landing_frame < 0 {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    if frame > special_s_jump_attack_shift_frame_min as f32 && (ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT);
    }
    if littlemac_can_cancel_into_dash(fighter).get_bool() {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return 1.into();
    }
    if frame == 9.0 && star_punch_strength > 0 {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x+0.5, get_sum_speed_y+0.3);
    }
    if MotionModule::is_end(fighter.module_accessor) || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_S_FLAG_IS_BLOW_SHIFT) {
        fighter.change_status(FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW.into(), false.into());
        return 1.into();
    }
    littlemac_special_s_ray_check(fighter);
    0.into()
}

unsafe extern "C" fn littlemac_special_s_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let star_punch_strength = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_STATUS_SPECIAL_N_FLAG_RESET_KO_GAUGE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
    }
    if status_kind != *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW {
        match star_punch_strength {
            1 => WorkModule::sub_float(fighter.module_accessor, 1.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE),
            2 => WorkModule::sub_float(fighter.module_accessor, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE),
            3 => WorkModule::sub_float(fighter.module_accessor, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE),
            _ => WorkModule::sub_float(fighter.module_accessor, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE)
        }
        if star_punch_strength > 0 {
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_REQUEST_KO_GAUGE_MAX_EFFECT);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_KO_GAGE_MAX_KEEP_FRAME);
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
    0.into()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, littlemac_special_s_jump_main_status)
    .status(End, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP, littlemac_special_s_jump_end_status)
    .install()
    ;
}