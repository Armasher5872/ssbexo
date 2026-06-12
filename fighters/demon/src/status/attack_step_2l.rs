use super::*;

//Dragon Uppercut Main Status
unsafe extern "C" fn demon_attack_step_2l_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT) {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_step_2l_elec"), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_step_2l"), -1.0, 1.0, 0.0, false, false);
    }
    ItemModule::set_have_item_visibility(boma, false, 0);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_14);
    ControlModule::reset_special_command(boma, true);
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_2l_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_step_2l_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    0.into()
}

//Dragon Uppercut End Status
unsafe extern "C" fn demon_attack_step_2l_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
    0.into()
}

//Dragon Uppercut Exit Status
unsafe extern "C" fn demon_attack_step_2l_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, demon_attack_step_2l_main_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, demon_attack_step_2l_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, demon_attack_step_2l_exit_status)
    .install()
    ;
}