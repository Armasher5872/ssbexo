use super::*;

//Neutral Special

unsafe extern "C" fn chrom_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let kinetic_energy_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let kinetic_energy_normal_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergyNormal;
    let gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let special_n_start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_spd_x_mul"));
    let special_n_brake_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_brake_spd_x"));
    let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    vector["x"].assign(&L2CValue::F32(vec_x*special_n_start_spd_x_mul));
    if copy_chara == kind {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO);
    }
    if situation_kind != *SITUATION_KIND_AIR {
        smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f::zero(), &Vector3f::zero(), module_accessor);
        smash::app::lua_bind::KineticEnergyNormal::set_accel(kinetic_energy_normal_stop_energy, &Vector2f::zero());
        smash::app::lua_bind::KineticEnergyNormal::set_brake(kinetic_energy_normal_stop_energy, &Vector2f{x: special_n_brake_spd_x, y: 0.0});
        smash::app::lua_bind::KineticEnergyNormal::set_stable_speed(kinetic_energy_normal_stop_energy, &Vector2f::zero());
        smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_stop_energy);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    }
    else {
        if vec_y < 0.0 {
            vector["y"].assign(&L2CValue::F32(0.0));
        }
        smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f::zero(), &Vector3f::zero(), module_accessor);
        smash::app::lua_bind::KineticEnergyNormal::set_accel(kinetic_energy_normal_stop_energy, &Vector2f::zero());
        smash::app::lua_bind::KineticEnergyNormal::set_brake(kinetic_energy_normal_stop_energy, &Vector2f{x: special_n_brake_spd_x, y: 0.0});
        smash::app::lua_bind::KineticEnergyNormal::set_stable_speed(kinetic_energy_normal_stop_energy, &Vector2f::zero());
        smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_stop_energy);
        smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: vec_y}, &Vector3f::zero(), module_accessor);
        smash::app::lua_bind::KineticEnergy::enable(gravity_energy);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    0.into()
}

unsafe extern "C" fn chrom_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let kinetic_energy_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let kinetic_energy_normal_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergyNormal;
    let special_n_start_spd_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_start_spd_x_mul"));
    let special_n_brake_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("special_n_brake_spd_x"));
    let copy_chara = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let prev_situation_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(get_sum_speed));
    vector["y"].assign(&L2CValue::F32(get_sum_speed));
    vector["x"].assign(&L2CValue::F32(vec_x*special_n_start_spd_x_mul));
    if copy_chara == kind {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO);
    }
    if situation_kind != prev_situation_kind {
        if situation_kind != *SITUATION_KIND_AIR {
            smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f::zero(), &Vector3f::zero(), module_accessor);
            smash::app::lua_bind::KineticEnergyNormal::set_accel(kinetic_energy_normal_stop_energy, &Vector2f::zero());
            smash::app::lua_bind::KineticEnergyNormal::set_brake(kinetic_energy_normal_stop_energy, &Vector2f{x: special_n_brake_spd_x, y: 0.0});
            smash::app::lua_bind::KineticEnergyNormal::set_stable_speed(kinetic_energy_normal_stop_energy, &Vector2f::zero());
            smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_stop_energy);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
        }
        else {
            if vec_y < 0.0 {
                vector["y"].assign(&L2CValue::F32(0.0));
            }
            smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f::zero(), &Vector3f::zero(), module_accessor);
            smash::app::lua_bind::KineticEnergyNormal::set_accel(kinetic_energy_normal_stop_energy, &Vector2f::zero());
            smash::app::lua_bind::KineticEnergyNormal::set_brake(kinetic_energy_normal_stop_energy, &Vector2f{x: special_n_brake_spd_x, y: 0.0});
            smash::app::lua_bind::KineticEnergyNormal::set_stable_speed(kinetic_energy_normal_stop_energy, &Vector2f::zero());
            smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_stop_energy);
            smash::app::lua_bind::KineticEnergy::reset_energy(gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: vec_y}, &Vector3f::zero(), module_accessor);
            smash::app::lua_bind::KineticEnergy::enable(gravity_energy);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_INT_SITUATION_PREV);
        }
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_loop"), L2CValue::Hash40s("special_air_n_loop"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_n_loop_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_n_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_loop"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_loop"), -1.0, 1.0, 0.0, false, false);
    }
    WorkModule::add_float(fighter.module_accessor, 1.0, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX.into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
        fighter.change_status(FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_loop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_count = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_WORK_FLOAT_CHARGE_COUNT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX) {
        AttackModule::set_power_up(fighter.module_accessor, 1.0+charge_count/60.0);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_end_max"), L2CValue::Hash40s("special_air_n_end_max"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_n_end_max_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_n_end_max_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if special_zoom_gfx > 0 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx >= 4 {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        macros::CAM_ZOOM_OUT(fighter);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end_max"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end_max"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn chrom_special_n_end_max_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
            let frame = fighter.global_table[CURRENT_FRAME].get_f32();
            if frame > 18.0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_N_FLAG_CHARGE_MAX) {
                WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                if special_zoom_gfx < 2 {
                    SlowModule::set_whole(fighter.module_accessor, 8, 80);
                    macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                    macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    AttackModule::set_power_up(fighter.module_accessor, 1.0);
    0.into()
}

unsafe extern "C" fn chrom_special_n_end_max_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    AttackModule::set_power_up(fighter.module_accessor, 1.0);
    0.into()
}

//Side Special

unsafe extern "C" fn chrom_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        macros::SET_SPEED_EX(fighter, 2.0, sum_speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn chrom_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
        fighter.change_status(FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
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

unsafe extern "C" fn chrom_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::detach_all(fighter.module_accessor, 5);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("chrom_final_speedline"), false, true);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
    0.into()
}

unsafe extern "C" fn chrom_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::detach_all(fighter.module_accessor, 5);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("chrom_final_speedline"), false, true);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
    0.into()
}

unsafe extern "C" fn chrom_special_s_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_s_attack_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn chrom_special_s_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_attack"), L2CValue::Hash40s("special_air_s_attack"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_s_attack_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_attack"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_attack"), -1.0, 1.0, 0.0, false, false);
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

unsafe extern "C" fn chrom_special_s_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_s_attack_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_s_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Up Special

unsafe extern "C" fn chrom_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP) {
        if situation_kind == *SITUATION_KIND_GROUND {
            macros::ADD_SPEED_NO_LIMIT(fighter, 0, 3.3);
        }
        else {
            macros::ADD_SPEED_NO_LIMIT(fighter, 0, 4.0);
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn chrom_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    macros::ADD_SPEED_NO_LIMIT(fighter, 0, 1.5);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.12);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*lr, 0.0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_hold"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_hi_hold_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_hi_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_hold_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let lr = PostureModule::lr(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable*lr, 0.0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_drop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_hi_drop_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_hi_drop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_drop_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_land"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(chrom_special_hi_land_main_loop as *const () as _))
}

unsafe extern "C" fn chrom_special_hi_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn chrom_special_hi_land_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Down Special

unsafe extern "C" fn chrom_special_lw_hit_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let kinetic_energy_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let kinetic_energy_normal_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergyNormal;
    let kinetic_energy_gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let fighter_kinetic_energy_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::FighterKineticEnergyGravity;
    let hit_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    let start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_mul_spd_x"));
    let start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_air_acl_x"));
    let attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_acl_y"));
    let attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_max_y"));
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(sum_speed));
    vector["y"].assign(&L2CValue::F32(sum_speed));
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if status_kind != *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT {
            return 0.into();
        }
        WorkModule::set_float(fighter.module_accessor, 10.0+(2.0*hit_count as f32), *FIGHTER_ROY_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    }
    else {
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_ROY_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        }
        else {
            smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_stop_energy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: start_mul_spd_x*vec_x, y: 0.0}, &Vector3f::zero(), module_accessor);
            smash::app::lua_bind::KineticEnergyNormal::set_brake(kinetic_energy_normal_stop_energy, &Vector2f{x: start_air_acl_x, y: 0.0});
            smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_stop_energy);
            smash::app::lua_bind::KineticEnergy::reset_energy(kinetic_energy_gravity_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: vec_y}, &Vector3f::zero(), module_accessor);
            smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_kinetic_energy_gravity, -attack_acl_y);
            smash::app::lua_bind::FighterKineticEnergyGravity::set_stable_speed(fighter_kinetic_energy_gravity, attack_max_y);
            smash::app::lua_bind::KineticEnergy::enable(kinetic_energy_gravity_energy);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_ROY_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    0.into()
}

unsafe extern "C" fn chrom_special_lw_hit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_lw_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD_CHK) {
        ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_ROY_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    if special_lw_count < 3 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
    }
    0.into()
}

pub fn install() {
    Agent::new("chrom")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, chrom_special_n_exit_status)
    .status(Pre, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_pre_status)
    .status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_init_status)
    .status(Main, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_main_status)
    .status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_exec_status)
    .status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_end_status)
    .status(Exit, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, chrom_special_n_loop_exit_status)
    .status(Pre, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_pre_status)
    .status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_init_status)
    .status(Main, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_main_status)
    .status(Exec, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_exec_status)
    .status(CheckAttack, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_check_attack_status)
    .status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_end_status)
    .status(Exit, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END_MAX, chrom_special_n_end_max_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, chrom_special_s_exit_status)
    .status(Pre, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_pre_status)
    .status(Init, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_init_status)
    .status(Main, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_main_status)
    .status(Exec, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_exec_status)
    .status(End, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_end_status)
    .status(Exit, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_S_ATTACK, chrom_special_s_attack_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, chrom_special_hi_exit_status)
    .status(Pre, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_pre_status)
    .status(Init, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_init_status)
    .status(Main, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_main_status)
    .status(Exec, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_exec_status)
    .status(End, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_end_status)
    .status(Exit, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_HOLD, chrom_special_hi_hold_exit_status)
    .status(Pre, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_pre_status)
    .status(Init, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_init_status)
    .status(Main, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_main_status)
    .status(Exec, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_exec_status)
    .status(End, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_end_status)
    .status(Exit, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_DROP, chrom_special_hi_drop_exit_status)
    .status(Pre, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_pre_status)
    .status(Init, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_init_status)
    .status(Main, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_main_status)
    .status(Exec, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_exec_status)
    .status(End, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_end_status)
    .status(Exit, *FIGHTER_CHROM_STATUS_KIND_SPECIAL_HI_LAND, chrom_special_hi_land_exit_status)
    .status(Init, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_special_lw_hit_init_status)
    .status(End, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, chrom_special_lw_hit_end_status)
    .install()
    ;
}