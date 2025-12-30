use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_end_CliffJump2)]
unsafe extern "C" fn status_end_cliffjump2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        if situation_kind != *SITUATION_KIND_LADDER {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
            FighterUtil::set_pickelblock_mode_normal(fighter.module_accessor);
            GroundModule::set_ignore_boss(fighter.module_accessor, false);
        }
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CLIFF_XLU);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    FighterUtil::set_pickelblock_mode_normal(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(
            status_end_cliffjump2
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}