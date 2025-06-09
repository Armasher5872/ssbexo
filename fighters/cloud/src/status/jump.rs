use super::*;

unsafe extern "C" fn cloud_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_jump_item_rocketbelt();
    cloud_status_jump_sub(fighter, hash40("invalid").into(), 0.0f32.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Jump_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_jump_sub(fighter: &mut L2CFighterCommon, motion_kind: L2CValue, value: L2CValue) -> L2CValue {
    let global_stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let jump_neutral_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_neutral_x"));
    let mini_jump = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let stick_x = global_stick_x*lr*-1.0;
    let mot;
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&0xFE.into());
    ControlModule::reset_trigger(fighter.module_accessor);
    fighter.sub_air_check_fall_common_pre();
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_POWBLOCK_QUAKE_JUMP) {
        if stick_x <= jump_neutral_x {
            if !mini_jump {
                if is_punisher {
                    mot = hash40("punish_jump_f");
                }
                else {
                    mot = hash40("jump_f");
                }
            }
            else {
                if is_punisher {
                    mot = hash40("punish_jump_f_mini");
                }
                else {
                    mot = hash40("jump_f_mini");
                }
            }
        }
        else {
            if !mini_jump {
                if is_punisher {
                    mot = hash40("punish_jump_b");
                }
                else {
                    mot = hash40("jump_b");   
                }
            }
            else {
                if is_punisher {
                    mot = hash40("punish_jump_b_mini");
                }
                else {
                    mot = hash40("jump_b_mini");   
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_item_usagihat_jump_01"), true, false, false, false, enSEType(0));
        }
    }
    else {
        if is_punisher {
            mot = hash40("punish_jump_f_mini");
        }
        else {
            mot = hash40("jump_f_mini");
        }
    }
    let ret = if motion_kind.get_u64() != hash40("invalid") {motion_kind.get_u64()} else {mot};
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ret), 0.0, 1.0, false, value.get_f32(), false, false);
    if fighter.global_table[FALL_BRAKE_UNIQ].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[FALL_BRAKE_UNIQ].get_ptr());
        callable(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_fall_common_uniq as *const () as _));
    ret.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_JUMP, cloud_jump_main_status)
    .install()
    ;
}