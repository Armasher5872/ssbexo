use super::*;

unsafe extern "C" fn ganon_special_hi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(fighter.module_accessor);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32e468d950), Hash40::new("throw"), Hash40::new("invalid"));
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    VisibilityModule::set_whole(fighter.module_accessor, true);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    GroundModule::set_ignore_boss(fighter.module_accessor, false);
    JostleModule::set_status(fighter.module_accessor, true);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_throw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_hi_throw_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_hi_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_end"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_throw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ganon_special_hi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, ganon_special_hi_throw_exit_status)
    .install()
    ;
}