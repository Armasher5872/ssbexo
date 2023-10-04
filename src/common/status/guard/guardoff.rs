/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Guard Off Uniq
#[skyline::hook(replace = L2CFighterCommon_sub_guard_off_uniq)]
unsafe fn sub_guard_off_uniq(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        let just_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if 0 < just_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
            if just_frame == 0 {
                ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
                let guard_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
                ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(guard_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
                /*Change from vanilla script: Removed the FighterUtil::is_valid_just_shield_reflector check, permitting parry reflecting*/
                ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
            }
        }
        let cancel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
        if 0 < cancel_frame {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
            if cancel_frame == 0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        }
    }
    0.into()
}

//sub_ftStatusUniqProcessGuardOff_exitStatus
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_exitStatus)]
unsafe fn sub_ftstatusuniqprocessguardoff_exitstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
        /*Change from vanilla script: Removed the FighterUtil::is_valid_just_shield_reflector check, permitting parry reflecting*/
        ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    }
    if status_kind == *FIGHTER_STATUS_KIND_GUARD_DAMAGE && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
    ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ATTACK_HI4_START].contains(&status_kind) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_guard_off_uniq,
            sub_ftstatusuniqprocessguardoff_exitstatus
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}
