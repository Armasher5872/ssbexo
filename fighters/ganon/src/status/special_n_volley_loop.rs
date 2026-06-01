use super::*;

unsafe extern "C" fn ganon_special_n_volley_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.02);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
        }
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PLAY_SE(fighter, Hash40::new("se_ganon_special_n04"));
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_loop"), L2CValue::Hash40s("special_air_n_volley_loop"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_n_volley_loop_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_n_volley_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_n_loop_count = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_N_LOOP_COUNT);
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            if special_n_loop_count > 4 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_volley_max"), -1.0, 1.0, 0.0, false, false);
            }
            else if special_n_loop_count == 4 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_volley_max_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_volley_loop"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
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
            if special_n_loop_count > 4 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_volley_max"), -1.0, 1.0, 0.0, false, false);
            }
            else if special_n_loop_count == 4 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_volley_max_start"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_volley_loop"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if current_frame == 3.0 && situation_kind == *SITUATION_KIND_AIR && !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.06);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.5/*Maximum Horizontal Air Speed*/, 0.0);
    }
    if current_frame == 31.0 && situation_kind == *SITUATION_KIND_AIR {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.03833);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.65);
    }
    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE.into(), false.into());
    }
    if special_n_loop_count > 9 {
        WorkModule::on_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        if special_n_loop_count > 4 {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_max"), L2CValue::Hash40s("special_air_volley_n_max"), false.into());
        }
        else if special_n_loop_count == 4 {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_max_start"), L2CValue::Hash40s("special_air_n_volley_max_start"), false.into());
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_volley_loop"), L2CValue::Hash40s("special_air_n_volley_loop"), false.into());
        }
        WorkModule::inc_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_N_LOOP_COUNT);
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let boma = fighter.module_accessor;
    let effect = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    if (160.0..=200.0).contains(&current_frame) {
        WorkModule::add_float(boma, 0.1, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
        EffectModule::set_scale(boma, effect as u32, &Vector3f{x: 3.8+(0.1*(current_frame-159.0)), y: 3.8+(0.1*(current_frame-159.0)), z: 3.8+(0.1*(current_frame-159.0))});
        EffectModule::set_pos(boma, effect as u32, &Vector3f{x: 4.0+(0.1*(current_frame-159.0)), y: 0.0, z: 0.0});
    }
    WorkModule::add_float(boma, 0.01667, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    STOP_SE(fighter, Hash40::new("se_ganon_special_n04"));
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE {
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
        WorkModule::set_float(boma, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
        WorkModule::set_float(boma, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
        EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_N_LOOP_COUNT);
    0.into()
}

unsafe extern "C" fn ganon_special_n_volley_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    STOP_SE(fighter, Hash40::new("se_ganon_special_n04"));
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_FIRE {
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_N_CHARGED);
        WorkModule::set_float(boma, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_DAMAGE_CHARGE);
        WorkModule::set_float(boma, 1.0, *FIGHTER_GANON_INSTANCE_WORK_ID_FLOAT_VOLLEY_SCALE_CHARGE);
        EffectModule::kill_kind(boma, Hash40::new("ganon_volley"), true, true);
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_N_LOOP_COUNT);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_VOLLEY_LOOP, ganon_special_n_volley_loop_exit_status)
    .install()
    ;
}