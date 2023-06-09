/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Attack LW4 Hold End, declares the Fully Charged Smash Attack variable
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw4Hold)]
unsafe fn status_attacklw4hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    0.into()
}

//Attack LW4 End, clears the full smash attack flags
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackLw4)]
unsafe fn status_end_attacklw4(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attacklw4hold_end,
            status_end_attacklw4
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}