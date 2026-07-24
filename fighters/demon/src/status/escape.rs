use super::*;

unsafe extern "C" fn demon_escape_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if prev_status_kind == *FIGHTER_STATUS_KIND_DASH {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_MIST_STEP_ACTIVE);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY4, *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A);
    }
    fighter.status_Escape_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_escape_main_loop as *const () as _))
}

unsafe extern "C" fn demon_escape_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let cmd_cat4 = fighter.global_table[CMD_CAT4].get_i32();
    let boma = fighter.module_accessor;
    let rage_system = WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM);
    let is_attack = cmd_cat1 & (*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 |*FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0;
    if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_MIST_STEP_ACTIVE) {
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND /*623B*/ != 0 {
            if is_attack {
                if rage_system {
                    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
                    fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
                }
            }
            else {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2H.into(), true.into());
            }
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG != 0 {
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY != 0 && rage_system {
                WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE.into(), true.into());
            }
            else {
                WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L.into(), true.into());
            }
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F.into(), true.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2.into(), true.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP.into(), false.into());
            return 1.into();
        }
        if cmd_cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB != 0 {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1.into(), true.into());
            return 1.into();
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ESCAPE_ATTACK.into(), false.into());
        return 1.into();
    }
    fighter.status_Escape_Main()
}

unsafe extern "C" fn demon_escape_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let frame = MotionModule::frame(boma);
    let max_time = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_n_hit_normal_frame"))-WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_n_hit_xlu_frame"));
    let xlu_frame = if max_time-frame <= 0.0 {0.0} else {max_time-frame};
    if [
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2H, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1
    ].contains(&status_kind) {
        HitModule::set_xlu_frame_global(boma, xlu_frame as i32, 0);
    }
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ESCAPE, demon_escape_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ESCAPE, demon_escape_end_status)
    .install()
    ;
}