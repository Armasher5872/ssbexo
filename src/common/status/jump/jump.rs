/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Pre Jump Sub Param, handles momentum transfer
#[skyline::hook(replace = L2CFighterCommon_status_pre_Jump_sub_param)]
unsafe extern "C" fn status_pre_jump_sub_param(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue, param_4: L2CValue, param_5: L2CValue) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    call_original!(fighter, param_1, param_2, param_3, param_4, param_5)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(
            status_pre_jump_sub_param
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}