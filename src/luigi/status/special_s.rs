use super::*;

unsafe extern "C" fn luigi_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_s_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn luigi_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if LUIGI_CYCLONE_RNG[entry_id] > 1 {
        let rand_num = sv_math::rand(hash40("fighter"), LUIGI_CYCLONE_RNG[entry_id]);
        if rand_num == 1 {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
    }
    else {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
    }
    fun_71000102b0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_s_loop as *const () as _))
}

unsafe extern "C" fn fun_71000102b0(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {      
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
        fighter.set_back_cliff_hangdata(12.0, 12.0);
        fighter.set_front_cliff_hangdata(12.0, 12.0);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_LW);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_FIRST);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE) {
            WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
        }
    }
}

unsafe extern "C" fn luigi_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    let mut bool_check = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        let int_mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_INT_MTRANS);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != int_mtrans {
            if situation_kind == int_mtrans {
                bool_check = true;
            }
        }
    }
    if bool_check {
        fun_71000102b0(fighter);
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn luigi_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC) {
        let float_limit_x_dec = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let param_limit_x_dec = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("limit_x_dec") as u64);
        let mut sum_limit_x_dec = float_limit_x_dec+param_limit_x_dec;
        WorkModule::set_float(fighter.module_accessor, sum_limit_x_dec, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_LIMIT_X_DEC);
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let mut alstack128; //not sure why this is initialized with a value never used
        let kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
        if kinetic_type != *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_LW {
            let air_limit_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("air_limit_x") as u64);
            alstack128 = air_limit_x;
        }
        else {
            let limit_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("limit_x") as u64);
            alstack128 = limit_x;
        }
        if sum_limit_x_dec < alstack128 {
            alstack128 = 0.0;
        }
        else {
            alstack128 -= sum_limit_x_dec;
        }
        smash::app::lua_bind::KineticEnergyNormal::set_limit_speed(control_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: alstack128, y: 0.0});
    }
    let mut float_rise_y: f32;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY) {
        float_rise_y = 0.0;
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                float_rise_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw") as u64, hash40("buoyancy") as u64);
                let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
                if situation_kind == *SITUATION_KIND_AIR {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_AIR_LW);
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                }
            }
            else {
                float_rise_y = 0.0;
            }
        }
        else {
            float_rise_y = 0.0;
        }
    }
    WorkModule::set_float(fighter.module_accessor, float_rise_y, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLOAT_RISE_Y);
    0.into()
}

unsafe extern "C" fn luigi_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
    LUIGI_CYCLONE_RNG[entry_id] = 9;
    0.into()
}

unsafe extern "C" fn luigi_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, luigi_special_s_exit_status)
    .install()
    ;
}