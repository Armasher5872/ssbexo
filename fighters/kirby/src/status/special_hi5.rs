use super::*;

unsafe extern "C" fn kirby_special_hi_5_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_KIRBY_FINALCUTTER_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_71001a6740(fighter);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_FINALCUTTER_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi5") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi5") as i64, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_FINALCUTTER_AIR_MOT);
    fun_710022ad50(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kirby_special_hi_5_main_loop as *const () as _))
}

unsafe extern "C" fn kirby_special_hi_5_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_land_weak_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_land_weak_frame"));
    WorkModule::set_float(fighter.module_accessor, special_hi_land_weak_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(0));
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 5);
    sv_module_access::effect(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    0.into()
}

unsafe extern "C" fn kirby_special_hi_5_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("kirby")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_pre_status)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_init_status)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_main_status)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_exec_status)
    .status(End, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_end_status)
    .status(Exit, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI5, kirby_special_hi_5_exit_status)
    .install()
    ;
}