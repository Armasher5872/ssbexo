use super::*;

unsafe extern "C" fn ridley_attack_lw4_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_NO_REACTION));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_4 as u32, 0);
    0.into()
}

unsafe extern "C" fn ridley_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.attack_lw4_mtrans();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.status_ThrowKirby_Uniq(L2CValue::Bool(false));
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_status_ThrowKirby_Uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_attack_lw4_main_loop as *const () as _))
}

unsafe extern "C" fn ridley_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND)
    && !MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_status_uniq_process_ThrowKirby_execFixPos();
        return 0.into()
    }
    fighter.status_AttackLw4_Main_param(FIGHTER_STATUS_KIND_WAIT.into());
    0.into()
}

unsafe extern "C" fn ridley_attack_lw4_map_correction_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let frame = MotionModule::frame(fighter.module_accessor);
    let prev_frame = MotionModule::prev_frame(fighter.module_accessor);
    let start_air_frame = 8.0;
    let fall_start_frame = 21.0;
    let fall_stop_frame = 22.0;
    let landing_frame = 23.0;
    if frame <= fall_start_frame {
        return 0.into()
    }
    if prev_frame < start_air_frame 
    && frame >= start_air_frame {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if prev_frame < fall_stop_frame && frame >= fall_stop_frame {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
            sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
            MotionModule::set_frame(fighter.module_accessor, fall_stop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor)
            && GroundModule::is_passable_ground(fighter.module_accessor) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_THROW_KIRBY_GROUND);
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn ridley_up_special_wall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] += 1;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall_b"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall"), 0.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_wall"))*-1.0;
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    let mut speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_y_on_stop_wall"));
    if speed_y <  0.0{
        speed_y = 0.0;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_up_special_wall_main_loop as *const () as _))
}

unsafe extern "C" fn ridley_up_special_wall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    false.into()
}

pub fn install() {
    Agent::new("ridley")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4, ridley_attack_lw4_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, ridley_attack_lw4_main_status)
    .status(MapCorrection, *FIGHTER_STATUS_KIND_ATTACK_LW4, ridley_attack_lw4_map_correction_status)
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, ridley_up_special_wall_main_status)
    .install()
    ;
}