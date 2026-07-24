use super::*;

//Retaliation Stance Hit Pre Status
unsafe extern "C" fn edge_special_lw_hit_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Retaliation Stance Hit Init Status
unsafe extern "C" fn edge_special_lw_hit_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH) {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
        }
        else {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    0.into()
}

//Retaliation Stance Hit Main Status
unsafe extern "C" fn edge_special_lw_hit_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_flash").into(), Hash40::new("special_air_lw_flash").into(), false.into());
    }
    else {
        edge_enable_cancel_terms(boma);
        fighter.sub_change_motion_by_situation(Hash40::new("special_lw_parry_hit").into(), Hash40::new("special_air_lw_parry_hit").into(), false.into());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_lw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_lw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let end_frame = MotionModule::end_frame(boma);
    let frame = MotionModule::frame(boma);
    let is_parry_flash = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH);
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        if is_parry_flash {
            fighter.sub_change_motion_by_situation(Hash40::new("special_lw_flash").into(), Hash40::new("special_air_lw_flash").into(), true.into());
        }
        else {
            fighter.sub_change_motion_by_situation(Hash40::new("special_lw_parry_hit").into(), Hash40::new("special_air_lw_parry_hit").into(), true.into());
        }
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            if is_parry_flash {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            }
            else {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }
    if !is_parry_flash {
        if end_frame-frame <= 27.0 {
            edge_try_cancel(fighter);
        }
    }
    else {
        if end_frame-frame == 20.0 {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.04);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
        }
    }
    if MotionModule::is_end(boma) {
        if is_parry_flash {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

//Retaliation Stance Hit Exec Status
unsafe extern "C" fn edge_special_lw_hit_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn edge_special_lw_hit_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let object_id = get_table_value(table, "object_id_").try_integer().unwrap();
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH) {
                WorkModule::set_int(boma, object_id as i32, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
            }
        }
    }
    0.into()
}

//Retaliation Stance Hit End Status
unsafe extern "C" fn edge_special_lw_hit_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    0.into()
}

//Retaliation Stance Hit Exit Status
unsafe extern "C" fn edge_special_lw_hit_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_pre_status)
    .status(Init, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_init_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_main_status)
    .status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_exec_status)
    .status(CheckAttack, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_check_attack_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_end_status)
    .status(Exit, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, edge_special_lw_hit_exit_status)
    .install()
    ;
}