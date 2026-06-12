/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardDamage_initStatus)]
unsafe extern "C" fn sub_ftstatusuniqprocessguarddamage_initstatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        fighter.FighterStatusGuard__set_just_shield_scale();
    }
    else {
        let prev_shield_scale_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_GUARD_DAMAGE_WORK_INT_PREV_SHIELD_SCALE_FRAME);
        let shield_hp_const = if 0 < prev_shield_scale_frame {*FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLOAT_PREV_SHIELD} else {*FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD};
        let shield_hp = WorkModule::get_float(boma, shield_hp_const);
        let scale = fighter.FighterStatusGuard__calc_shield_scale(shield_hp.into()).get_f32();
        ModelModule::set_joint_scale(boma, Hash40::new("throw"), &Vector3f{x: scale, y: scale, z: scale});
    }
    0.into()
}

//FighterStatusGuard__check_hit_stop_delay_flick. Removes shield SDI
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__check_hit_stop_delay_flick)]
unsafe extern "C" fn fighterstatusguard_check_hit_stop_delay_flick(_fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    false.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_ftstatusuniqprocessguarddamage_initstatus,
            fighterstatusguard_check_hit_stop_delay_flick
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}