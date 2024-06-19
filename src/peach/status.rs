use super::*;

/*   THROW STATUS SCRIPTS   */

unsafe extern "C" fn peach_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_THROW_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, (*FIGHTER_LOG_MASK_FLAG_THROW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_THROW as u32, 0);
        0.into()
    }
    else {
        fighter.status_pre_Throw()
    }
}

unsafe extern "C" fn peach_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_throw"), 0.0, 1.0, false, 0.0, false, false);
        peach_throw_sub_status(fighter);
        fighter.sub_shift_status_main(L2CValue::Ptr(peach_throw_main_loop as *const () as _))
    }
    else {
        fighter.status_Throw()
    }
}

unsafe extern "C" fn peach_throw_sub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.ThrowUniq();
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_ThrowUniq as *const () as _));
    0.into()
}

unsafe extern "C" fn peach_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn peach_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    JostleModule::set_status(fighter.module_accessor, true);
    fighter.status_end_Throw()
}

unsafe extern "C" fn peach_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    JostleModule::set_status(fighter.module_accessor, true);
    0.into()
}

/*   DOWN SPECIAL STATUS SCRIPTS   */

unsafe extern "C" fn peach_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn peach_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let special_lw_uniq_item_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_uniq_item_rate"));
    let special_lw_uniq_item_starring_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_starring_rate")); //6
    let special_lw_uniq_item_powblock_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_powblock_rate")); //16
    let special_lw_uniq_item_soccerball_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_soccerball_rate")); //26
    let special_lw_uniq_item_greenshell_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_greenshell_rate")); //38
    let special_lw_uniq_item_firebar_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_firebar_rate")); //51
    let special_lw_uniq_item_beamsword_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_uniq_item_beamsword_rate")); //65
    let special_lw_uniq_item_bombhei_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_uniq_item_bombhei_rate")); //82
    let special_lw_uniq_item_boomerang_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_boomerang_rate")); //101
    let special_lw_uniq_item_banana_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_banana_rate")); //122
    let special_lw_uniq_item_doseisan_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_uniq_item_doseisan_rate")); //147
    let special_lw_uniq_item_freezer_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_freezer_rate")); //173
    let special_lw_uniq_item_fireflower_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw_utility"), hash40("special_lw_uniq_item_fireflower_rate")); //201
    let rng = sv_math::rand(hash40("fighter"), special_lw_uniq_item_rate);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        FighterSpecializer_Peach::special_lw_check_num_of_item(module_accessor);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_LW_FLAG_FIX_RAND) {
            WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_NONE as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND);
        }
        else {
            match rng {
                _ if (0..=special_lw_uniq_item_starring_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_STARRING as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (7..=special_lw_uniq_item_powblock_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_POWBLOCK as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (17..=special_lw_uniq_item_soccerball_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_SOCCERBALL as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (27..=special_lw_uniq_item_greenshell_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_GREENSHELL as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (39..=special_lw_uniq_item_firebar_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_FIREBAR as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (52..=special_lw_uniq_item_beamsword_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_BEAMSWORD as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (66..=special_lw_uniq_item_bombhei_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_BOMBHEI as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (83..=special_lw_uniq_item_boomerang_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_BOOMERANG as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (102..=special_lw_uniq_item_banana_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_BANANA as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (123..=special_lw_uniq_item_doseisan_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_DOSEISAN as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (148..=special_lw_uniq_item_freezer_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_FREEZER as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ if (174..=special_lw_uniq_item_fireflower_rate).contains(&rng) => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_FIREFLOWER as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND),
                _ => WorkModule::set_int64(fighter.module_accessor, *ITEM_KIND_NONE as i64, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND)
            }
        }
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    0.into()
}

unsafe extern "C" fn peach_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_ITEM_NO_COUNT);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_04 + -1);
    fighter.pop_lua_stack(1);
    fighter.sub_shift_status_main(L2CValue::Ptr(peach_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn peach_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("peach")
    .status(Pre, *FIGHTER_STATUS_KIND_THROW, peach_throw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, peach_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, peach_throw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_THROW, peach_throw_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, peach_special_lw_main_status)
    .install()
    ;
}