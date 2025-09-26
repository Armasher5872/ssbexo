use super::*;

unsafe extern "C" fn ridley_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_lw4"), smash_restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(ridley_attack_lw4_main_loop as *const () as _))   
}

unsafe extern "C" fn ridley_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let start_air_frame = 5.0;
    let fall_loop_frame = 21.0;
    let landing_frame = 22.0;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if current_frame == start_air_frame {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if current_frame >= fall_loop_frame {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
            MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if current_frame >= fall_loop_frame && current_frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
                SA_SET(fighter, *SITUATION_KIND_AIR);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
                sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
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
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, ridley_attack_lw4_main_status)
    .status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, ridley_up_special_wall_main_status)
    .install()
    ;
}