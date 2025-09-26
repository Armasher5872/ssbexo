use super::*;

//Cloud Guard On Pre Status
unsafe extern "C" fn cloud_guard_on_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GUARD_ON, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32, 0, 0);
    0.into()
}

//Cloud Guard On Init Status
unsafe extern "C" fn cloud_guard_on_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let guard_off_disable_shield_recovery = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("guard_off_disable_shield_recovery"));
    WorkModule::set_int(fighter.module_accessor, guard_off_disable_shield_recovery, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_SHIELD_RECOVERY_FRAME);
    0.into()
}

//Cloud Guard On Main Status
unsafe extern "C" fn cloud_guard_on_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_guard_on"), 0.0, 1.0, false, 0.0, false, false);
    cloud_sub_guard_cont_pre(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        cloud_sub_guard_on_uniq(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(cloud_sub_guard_on_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_guard_on_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS
    ];
    let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON {
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_RUN {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
        }
    }
    for term in transition_terms.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *term);
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
}

unsafe extern "C" fn cloud_sub_guard_on_uniq(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let catch_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
    if bool_check.get_bool() {
        if 0 <= catch_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME);
            if catch_frame < 0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn cloud_guard_on_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_status_guard_on_main_air_common().get_bool()
    && !fighter.sub_guard_cont().get_bool()
    && !fighter.status_guard_main_common().get_bool()  {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD.into(), false.into());
        }
    }
    0.into()
}

//Guard On Exec Status
unsafe extern "C" fn cloud_guard_on_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Guard On End Status
unsafe extern "C" fn cloud_guard_on_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Guard On Exit Status
unsafe extern "C" fn cloud_guard_on_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_GUARD_ON, cloud_guard_on_exit_status)
    .install()
    ;
}