use super::*;

pub unsafe extern "C" fn miiswordsman_special_n2_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n2_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let spd_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_RAPID_SLASH_BRAKE_SPD_X);
    let spd_x_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_RAPID_SLASH_START_SPD_X_MUL);
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed*spd_x_mul));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if situation_kind != *SITUATION_KIND_AIR {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, vec_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, spd_x);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_WORK_INT_SITUATION_PREV);
    }
    else {
        if vec_y < 0.0 {
            vector["y"].assign(&L2CValue::F32(0.0));
        }
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, vec_x, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, spd_x);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, vec_y, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_WORK_INT_SITUATION_PREV);
        sv_kinetic_energy!(unable, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    sv_kinetic_energy!(unable, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n2_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n2"), L2CValue::Hash40s("special_air_n2"), false.into());
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_n2_start_main_loop as *const () as _))
}

unsafe extern "C" fn miiswordsman_special_n2_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST);
        }
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n2"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_LIGHT_SYURIKEN_FLAG_FIRST);
        }
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n2"), -1.0, 1.0, 0.0, false, false);
    }
    if frame >= 10.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_HOLD.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_N2_FIRE.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n2_start_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let spd_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_RAPID_SLASH_BRAKE_SPD_X);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let situation_prev = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_WORK_INT_SITUATION_PREV);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    if situation_prev != situation_kind {
        if situation_kind != *SITUATION_KIND_AIR {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, vec_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, spd_x);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_WORK_INT_SITUATION_PREV);
        }
        else {
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, vec_x, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, spd_x);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, vec_y, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(unable, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MIISWORDSMAN_STATUS_RAPID_SLASH_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_n2_start_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}