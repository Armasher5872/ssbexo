use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_CliffJump1)]
unsafe extern "C" fn status_end_cliffjump1(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_STATUS_KIND_CLIFF_JUMP2 {
        if situation_kind != *SITUATION_KIND_CLIFF {
            if situation_kind != *SITUATION_KIND_AIR {
                WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_COUNT);
            }
        }
        if situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind != *SITUATION_KIND_LADDER {
                WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
                WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
                FighterUtil::set_pickelblock_mode_normal(boma);
                GroundModule::set_ignore_boss(boma, false);
            }
        }
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    }
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    FighterUtil::set_pickelblock_mode_normal(boma);
    GroundModule::set_ignore_boss(boma, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(
            status_end_cliffjump1
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}