use super::*;

unsafe extern "C" fn ganon_special_n_volley_fire_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_fire_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_fire_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED) {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_fire_max"), L2CValue::Hash40s("special_air_n_volley_fire_max"), false.into());
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_fire"), L2CValue::Hash40s("special_air_n_volley_fire"), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_n_volley_fire_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_n_volley_fire_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let is_charged = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        if is_charged {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_volley_fire_max"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_volley_fire"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
        }
        if is_charged {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_volley_fire_max"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_volley_fire"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if current_frame == 3.0 && situation_kind == *SITUATION_KIND_AIR && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_fire_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_fire_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ganon_volley"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR);
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_fire_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ganon_volley"), true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE, ganon_special_n_volley_fire_exit_status)
    .install()
    ;
}