/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Guard Off
#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS, symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_exitStatusEv")]
unsafe fn ft_status_uniq_process_guard_off_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
        let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    }
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_DAMAGE && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
    let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_SPECIAL_HI || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_SQUAT || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK_HI4_START {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    }
    let just_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    if 0 < just_frame {
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_CAPTAIN {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_special_h03"));
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
        }
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if (just_frame - 1) == 0 {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
            let type_of_guard = FighterUtil::get_shield_type_of_guard(fighter.global_table[0x2].get_i32()) as i32;
            ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
    }
    let cancel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    if 0 < cancel_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
        if (cancel_frame - 1) == 0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    L2CValue::I32(0)
}

pub fn install() {
	install_status_scripts!(ft_status_uniq_process_guard_off_exit_status);
}