use super::*;

unsafe extern "C" fn gaogaen_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn gaogaen_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw_start").into(), false.into());
    GroundModule::correct(boma, GroundCorrectKind(if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_AIR} else {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP}));
    fun_7100013e50(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_lw_hit_damage_mul = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("special_lw_hit_damage_mul"));
    let special_lw_hit_stop_mul = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("special_lw_hit_stop_mul"));
    println!("Current X Speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START) {
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_START);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE);
        DamageModule::set_damage_mul_2nd(boma, special_lw_hit_damage_mul);
        HitModule::set_hit_stop_mul(boma, special_lw_hit_stop_mul, HitStopMulTarget{_address: 3}, 0.0);
        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_ALWAYS as u8}, -1.0, -1.0, -1);
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END) {
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE_END);
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_LW_FLAG_STANCE);
        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        DamageModule::set_damage_mul_2nd(boma, 1.0);
        HitModule::set_hit_stop_mul(boma, 1.0, HitStopMulTarget{_address: 3}, 0.0);
    }
    if StatusModule::is_situation_changed(boma) {
        GroundModule::correct(boma, GroundCorrectKind(if situation_kind == *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_AIR}));
        MotionModule::change_motion_inherit_frame(boma, Hash40::new(if situation_kind == *SITUATION_KIND_GROUND {"special_lw_start"} else {"special_air_lw_start"}), -1.0, 1.0, 0.0, false, false);
        fun_7100013e50(fighter, false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(if situation_kind == *SITUATION_KIND_GROUND {FIGHTER_STATUS_KIND_WAIT.into()} else {FIGHTER_STATUS_KIND_FALL.into()}, false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, gaogaen_special_lw_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, gaogaen_special_lw_main_status)
    .install()
    ;
}