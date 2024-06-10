use super::*;

unsafe extern "C" fn marth_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn marth_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let get_energy_motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let get_energy_gravity = KineticModule::get_energy(fighter.module_accessor, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY) as *mut smash::app::KineticEnergy;
    let air_hi_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_hi_start_x_mul"));
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if situation_kind != *SITUATION_KIND_AIR {
        KineticEnergy::reset_energy(get_energy_motion, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS_IGNORE_NORMAL, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticEnergy::enable(get_energy_motion);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    else {
        vector["x"].assign(&L2CValue::F32(vec_x*air_hi_start_x_mul));
        vector["y"].assign(&L2CValue::F32(0.0));
        KineticEnergy::reset_energy(get_energy_stop, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: vec_x, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticEnergy::reset_energy(get_energy_gravity, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: vec_y}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticEnergy::reset_energy(get_energy_motion, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS_ANGLE, &Vector2f{x: 0.0, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
        KineticEnergy::enable(get_energy_stop);
        KineticEnergy::enable(get_energy_gravity);
        KineticEnergy::enable(get_energy_motion);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn marth_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if stick_x > 0.7 {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_F.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_HI.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn marth_special_hi_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn marth_special_hi_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_hi"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_special_hi_hi_main_loop as *const () as _))
}

unsafe extern "C" fn marth_special_hi_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_hi_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    ret
}

unsafe extern "C" fn marth_special_hi_hi_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let fall_x_mul_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul_value"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    if fighter.global_table[STATUS_KIND] == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        WorkModule::set_float(fighter.module_accessor, fall_x_mul_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    */
    0.into()
}

unsafe extern "C" fn marth_special_hi_f_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn marth_special_hi_f_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_f"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_special_hi_f_main_loop as *const () as _))
}

unsafe extern "C" fn marth_special_hi_f_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn marth_special_hi_f_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
    ret
}

unsafe extern "C" fn marth_special_hi_f_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let fall_x_mul_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul_value"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    if fighter.global_table[STATUS_KIND] == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        WorkModule::set_float(fighter.module_accessor, fall_x_mul_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    0.into()
}

pub fn install() {
    Agent::new("marth")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, marth_special_hi_end_status)
    .status(Pre, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_HI, marth_special_hi_hi_pre_status)
    .status(Main, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_HI, marth_special_hi_hi_main_status)
    .status(Exec, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_HI, marth_special_hi_hi_exec_status)
    .status(End, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_HI, marth_special_hi_hi_end_status)
    .status(Pre, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_F, marth_special_hi_f_pre_status)
    .status(Main, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_F, marth_special_hi_f_main_status)
    .status(Exec, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_F, marth_special_hi_f_exec_status)
    .status(End, FIGHTER_MARTH_STATUS_KIND_SPECIAL_HI_F, marth_special_hi_f_end_status)
    .install()
    ;
}