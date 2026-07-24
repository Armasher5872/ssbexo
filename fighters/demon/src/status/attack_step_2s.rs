use super::*;

//Spinning Demon to Left Hook Pre Status
unsafe extern "C" fn demon_attack_step_2s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Spinning Demon to Left Hook Main Status
unsafe extern "C" fn demon_attack_step_2s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("attack_step_2s"), 0.0, 1.0, false, 0.0, false, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_14);
    ControlModule::reset_special_command(boma, true);
    MotionModule::set_trans_move_speed_no_scale(boma, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_2s_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_step_2s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
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
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_RAGE_DRIVE) {
        if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 {
            if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), false.into());
                return 1.into();
            }
            else {
                if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
                    WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), false.into());
                    return 1.into();
                }
            }
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    0.into()
}

//Spinning Demon to Left Hook End Status
unsafe extern "C" fn demon_attack_step_2s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_RAGE_DRIVE);
    0.into()
}

//Spinning Demon to Left Hook Exit Status
unsafe extern "C" fn demon_attack_step_2s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_RAGE_DRIVE);
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_pre_status)
    .status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_main_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_exit_status)
    .install()
    ;
}