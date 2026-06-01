use super::*;

unsafe extern "C" fn krool_special_hi_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn krool_special_hi_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    ArticleModule::generate_article(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, false, -1);
    ArticleModule::change_status_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_START);
    fun_710001dba0(fighter, Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fun_710001ea30(fighter);
    if !StopModule::is_stop(boma) {
        fun_7100023a20(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100023a20 as *const () as _));
    GroundModule::select_cliff_hangdata(boma, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_hi_start_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100023a20(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if bool_check.get_bool() {
        let stop_speed_x = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP); sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)};
        let gravity_speed_y = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)};
        let pyth = ((stop_speed_x*stop_speed_x)+(gravity_speed_y*gravity_speed_y)).sqrt();
        let movement_y = WorkModule::get_float(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        if gravity_speed_y >= 0.0 {
            WorkModule::set_float(boma, pyth+movement_y, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
        else {
            WorkModule::set_float(boma, (-pyth)+movement_y, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
    }
    0.into()
}

unsafe extern "C" fn krool_special_hi_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if prev_situation_kind == *SITUATION_KIND_GROUND {
        if situation_kind == *SITUATION_KIND_AIR {
            fun_710001dba0(fighter, Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), true.into());
        }
    }
    else {
        if situation_kind == *SITUATION_KIND_GROUND {
            fun_710001dba0(fighter, Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), true.into());
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, krool_special_hi_start_pre_status)
    .status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_START, krool_special_hi_start_main_status)
    .install()
    ;
}