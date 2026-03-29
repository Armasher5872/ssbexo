/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Attack Air, used to permit momentum transfer for aerials
#[skyline::hook(replace = L2CFighterCommon_status_pre_AttackAir)]
unsafe extern "C" fn status_pre_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_pre_attackair);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}