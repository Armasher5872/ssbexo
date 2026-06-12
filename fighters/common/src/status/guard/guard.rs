/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Sub Ft Status Uniq Process Guard On Init Status Common
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuard_initStatus_common)]
unsafe extern "C" fn sub_ftstatusuniqprocessguard_initstatus_common(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let shield_setoff_mul = WorkModule::get_param_float(boma, hash40("common"), 0x20d241cd64);
    ShieldModule::set_status(boma, *FIGHTER_STATUS_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
    ShieldModule::set_hit_stop_mul(boma, shield_setoff_mul);
}

//Status Guard Main Common, handles shield special transitioning
#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
unsafe extern "C" fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_GUARD) {
        let min_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if min_frame <= 0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

//Sub ftStatusUniqProcessGuardFunc_updateShield. Removes shield tilting
#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardFunc_updateShield)]
unsafe extern "C" fn sub_ftstatusuniqprocessguardfunc_updateshield(fighter: &mut L2CFighterCommon, _param_1: L2CValue) {
    let boma = fighter.module_accessor;
    let shield_hp = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    let shield_eff = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID) as u32;
    let shield_max = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ModelModule::set_joint_scale(boma, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    if EffectModule::is_exist_effect(boma, shield_eff) {
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
        EffectModule::set_scale(boma, shield_eff, &Vector3f{x: ratio, y: ratio, z: ratio});
    }
}

//FighterStatusGuard set_shield_scale. Removes shield tilting, and makes shields no longer decrease in size
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__set_shield_scale)]
unsafe extern "C" fn fighterstatusguard_set_shield_scale(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let shield_hp = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
    let shield_eff = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID) as u32;
    let shield_max = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    ModelModule::set_joint_scale(boma, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    if EffectModule::is_exist_effect(boma, shield_eff) {
        let ratio = (shield_hp/shield_max).clamp(0.1, 1.0)*0.1;
        EffectModule::set_scale(boma, shield_eff, &Vector3f{x: ratio, y: ratio, z: ratio});
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguard_initstatus_common,
            status_guard_main_common,
            sub_ftstatusuniqprocessguardfunc_updateshield,
            fighterstatusguard_set_shield_scale
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}