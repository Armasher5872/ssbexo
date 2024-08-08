use super::*;

unsafe extern "C" fn reflet_resurrection_book_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn reflet_resurrection_book_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn reflet_resurrection_thundersword_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn reflet_resurrection_thundersword_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn reflet_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_AIR);
    }
    0.into()
}

unsafe extern "C" fn reflet_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let special_hi_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
    let special_hi_current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_float(fighter.module_accessor, special_hi_landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    if special_hi_current_point <= 0 {
        FighterSpecializer_Reflet::set_flag_to_table(module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    reflet_special_hi_check_jump(fighter);
    reflet_special_hi_try_2nd(fighter);
    0.into()
}

unsafe extern "C" fn reflet_special_hi_check_jump(fighter: &mut L2CFighterCommon) {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let get_special_hi_jump_speed = FighterSpecializer_Reflet::get_special_hi_jump_speed(module_accessor);
    let grav_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let control_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x2c13759450);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("air_speed_x_limit"));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP) {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_add(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_add*control_mul);
        smash::app::lua_bind::FighterKineticEnergyController::set_accel_x_mul(control_energy as *mut smash::app::FighterKineticEnergyController, air_accel_x_mul*control_mul);
        smash::app::lua_bind::KineticEnergy::reset_energy(control_energy as *mut smash::app::KineticEnergy, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, &Vector2f{x: 0.0, y: 0.0}, &NONE_VECTOR, fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::reset_energy(grav_energy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: get_special_hi_jump_speed.y}, &NONE_VECTOR, fighter.module_accessor);
        smash::app::lua_bind::KineticEnergy::unable(stop_energy);
        smash::app::lua_bind::KineticEnergyNormal::set_limit_speed(control_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: air_speed_x_limit*control_mul, y: 0.0});
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, get_special_hi_jump_speed.x, 0.0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
}

unsafe extern "C" fn reflet_special_hi_try_2nd(fighter: &mut L2CFighterCommon) {
    let special_hi_current_point = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if special_hi_current_point > 0 {
            fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2.into(), false.into())
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
}

pub fn install() {
    Agent::new("reflet")
    .status(Pre, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_BOOK, reflet_resurrection_book_pre_status)
    .status(Init, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_BOOK, reflet_resurrection_book_init_status)
    .status(Pre, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_THUNDERSWORD, reflet_resurrection_thundersword_pre_status)
    .status(Init, *FIGHTER_REFLET_STATUS_KIND_RESURRECTION_THUNDERSWORD, reflet_resurrection_thundersword_init_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, reflet_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, reflet_special_hi_main_status)
    .install()
    ;
}