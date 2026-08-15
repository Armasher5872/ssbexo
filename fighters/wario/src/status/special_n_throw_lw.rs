use super::*;

//Neutral Special Down Throw Pre Status
unsafe extern "C" fn wario_special_n_throw_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Down Throw Init Status
unsafe extern "C" fn wario_special_n_throw_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let x_speed = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_speed);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.01);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    0.into()
}

//Neutral Special Down Throw Main Status
unsafe extern "C" fn wario_special_n_throw_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        grabbed_anim_selector(fighter, "bitten_wario_start", 0.0, 1.0);
    }
    else {
        let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
        if capture_id != 0x50000000 {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN, false);
        }
        grabbed_anim_selector(fighter, "bitten_wario", 0.0, 1.0);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_throw_lw"), L2CValue::Hash40s("special_air_n_throw_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_throw_lw_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_throw_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let capture_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    if CancelModule::is_enable_cancel(boma) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    handle_mash(fighter);
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND.into(), false.into());
            return 1.into();
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW) {
        if capture_id != 0x50000000 {
            AttackModule::hit_absolute_joint(boma, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
        }
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        WorkModule::off_flag(boma, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_THROW);
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Neutral Special Down Throw Exec Status
unsafe extern "C" fn wario_special_n_throw_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Down Throw End Status
unsafe extern "C" fn wario_special_n_throw_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        if ![
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
        ].contains(&status_kind) {
            SoundModule::stop_se(boma, Hash40::new("vc_wario_final01"), 0);
        }
        wario_special_n_end(fighter, true);
    }
    else {
        wario_special_n_end(fighter, true);
    }
    0.into()
}

//Neutral Special Down Throw Exit Status
unsafe extern "C" fn wario_special_n_throw_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        if ![
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F, 
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_FALL, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW_LAND
        ].contains(&status_kind) {
            SoundModule::stop_se(boma, Hash40::new("vc_wario_final01"), 0);
        }
        wario_special_n_exit(fighter, true);
    }
    else {
        wario_special_n_exit(fighter, true);
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW, wario_special_n_throw_lw_exit_status)
    .install()
    ;
}