use super::*;

//Special S Main, new up-special script in side-special status
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn armstrong_special_s_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ARMSTRONG_IS_SPECIAL_HI[entry_id] {
        KineticModule::clear_speed_all(fighter.module_accessor);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_hi_loop as *const () as _))
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            fun_7100010bc0(fighter);
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::set_attach_ground(fighter.module_accessor, true);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_s_loop as *const () as _))
    }
}

pub unsafe fn fun_7100010bc0(fighter: &mut L2CFighterCommon) {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
}

pub unsafe fn armstrong_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), true.into());
            USE_DROPKICK[entry_id] = false;
        }
        return 1.into()
    }
    0.into()
}

//Side Special Exec, disables side-special kinetic stuff if you did Up Special
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn armstrong_special_s_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ARMSTRONG_IS_SPECIAL_HI[entry_id] {
        0.into()
    }
    else {
        fighter.sub_transition_group_check_air_cliff();
        original!(fighter)
    }
}

//Special S End, resets flags when status ends
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn armstrong_special_s_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ARMSTRONG_IS_SPECIAL_HI[entry_id] {
        ARMSTRONG_IS_SPECIAL_HI[entry_id] = false;
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            USE_DROPKICK[entry_id] = false;
        }
    }
    0.into()
}

//Special Hi Pre, switch status before anything happens in LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn armstrong_special_hi_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ARMSTRONG_IS_SPECIAL_HI[entry_id] = true;
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
    0.into()
}

pub unsafe fn armstrong_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI);
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if frame > 14.0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), true.into());
            }
        }
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
        }
        ARMSTRONG_IS_SPECIAL_HI[entry_id] = false;
        return 1.into()
    }
    0.into()
}

//Special Air S Catch, technically remapped to Up Special
#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn armstrong_special_air_s_catch_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_air_s_catch_loop as *const () as _))
}

//Catch Loop, checks for Stick X in order to add horizontal speed
pub unsafe fn armstrong_special_air_s_catch_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[CURRENT_FRAME].get_f32() == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 0.0, 1.0, false, 0.0, false, false);
        fun_7100010bc0(fighter);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
        let influence = if stick_x.abs() < 0.2 {0.0} else {3.3*stick_x.signum()};
        let explosion_air_speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        let explosion_air_speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
        WorkModule::set_float(fighter.module_accessor, influence, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
        WorkModule::set_float(fighter.module_accessor, 0.665, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, explosion_air_speed_y, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_speed, fighter, explosion_air_speed_x, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() >= 18.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
                return 0.into();
            }
        }
        else {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

//Special Air S Fall, technically remapped to Up Special
#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn armstrong_special_air_s_fall_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_s_fall_check_dead_offset_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_fall_check_dead_offset_y"));
    WorkModule::set_float(fighter.module_accessor, special_s_fall_check_dead_offset_y+0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CHECK_DEAD_OFFSET_Y);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fall"), 0.0, 1.0, false, 0.0, false, false);
    fun_7100010bc0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_air_s_fall_loop as *const () as _))
}

//Fall Loop, checks for Stick X in order to add horizontal speed
pub unsafe fn armstrong_special_air_s_fall_loop(fighter: &mut L2CFighterCommon) -> bool {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor);
    let influence = if stick_x.abs() < 0.2 {0.0} else {2.5*stick_x.signum()};
    macros::SET_SPEED_EX(fighter, influence, -5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
    }
    false.into()
}

pub fn install() {
    install_status_scripts!(
        armstrong_special_s_status_main,
        armstrong_special_s_status_exec,
        armstrong_special_s_status_end,
        armstrong_special_hi_status_pre,
        armstrong_special_air_s_catch_status_main,
        armstrong_special_air_s_fall_status_main
    );
}