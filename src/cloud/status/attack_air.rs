use super::*;

unsafe extern "C" fn cloud_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_sub_attack_air_common(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackAir_Main as *const () as _))
}

unsafe extern "C" fn cloud_sub_attack_air_common(fighter: &mut L2CFighterCommon, value: L2CValue) {
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::reset_flick_y(fighter.module_accessor);
    ControlModule::reset_flick_sub_y(fighter.module_accessor);
    fighter.global_table[FLICK_Y].assign(&L2CValue::I32(0xFE));
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.attack_air_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_attack_air_uniq as *const () as _));
    if value.get_bool() {
        cloud_sub_attack_air_kind(fighter);
    }
    ControlModule::reset_attack_air_kind(fighter.module_accessor);
}

unsafe extern "C" fn cloud_sub_attack_air_kind(fighter: &mut L2CFighterCommon) {
    let motion_kind = cloud_sub_attack_air_kind_set_log_info(fighter).get_u64();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
}

unsafe extern "C" fn cloud_sub_attack_air_kind_set_log_info(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_air_kind;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    match get_attack_air_kind {
        _ if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F => {
            if is_punisher {
                attack_air_kind = hash40("punish_attack_air_f");
            }
            else {
                attack_air_kind = hash40("attack_air_f");
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F);
        },
        _ if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B => {
            if is_punisher {
                attack_air_kind = hash40("punish_attack_air_b");
            }
            else {
                attack_air_kind = hash40("attack_air_b");
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B);
        },
        _ if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI => {
            if is_punisher {
                attack_air_kind = hash40("punish_attack_air_hi");
            }
            else {
                attack_air_kind = hash40("attack_air_hi");
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI);
        },
        _ if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW => {
            if is_punisher {
                attack_air_kind = hash40("punish_attack_air_lw");
            }
            else {
                attack_air_kind = hash40("attack_air_lw");
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW);
        },
        _ => {
            if is_punisher {
                attack_air_kind = hash40("punish_attack_air_n");
            }
            else {
                attack_air_kind = hash40("attack_air_n");
            }
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N);
        }
    }
    attack_air_kind.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, cloud_attack_air_main_status)
    .install()
    ;
}