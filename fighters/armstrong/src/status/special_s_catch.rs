use super::*;

unsafe extern "C" fn armstrong_special_s_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_catch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
    sv_module_access::_catch(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, get_sum_speed_x*0.2, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, get_sum_speed_x*0.2, 0.0, 0.0, 0.0, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    WorkModule::set_float(fighter.module_accessor, get_sum_speed_y, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CATCH_INIT_Y_VEL);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31697c2b98), hash40("catched_ganon"), hash40("catched_air_ganon"));
    0.into()
}

unsafe extern "C" fn armstrong_special_s_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s_catch"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_s_catch_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_s_catch_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let init_y_vel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CATCH_INIT_Y_VEL);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            if current_frame > 28.0 {
                fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
                return 1.into();
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::clear_speed_all(fighter.module_accessor);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 6.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.7);
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.7);
            sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
            sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_catch"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if init_y_vel > 0.25 {
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.0);  
            } 
        }
        else {
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.0);   
        }
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 6.0);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.7);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.7);
        sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_s_catch_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_s_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CATCH_INIT_Y_VEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_catch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CATCH_INIT_Y_VEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_SPECIAL_S_JUMP);
    0.into()
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, armstrong_special_s_catch_exit_status)
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_s_catch_exit_status)
    .install()
    ;
}