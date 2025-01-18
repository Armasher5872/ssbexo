use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageSong_Main)]
unsafe fn status_damagesong_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    asdi_check(fighter);
    asdi_function(fighter);
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            macros::COL_NORMAL(fighter);
            macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if situation_kind != *SITUATION_KIND_AIR
    && ControlModule::get_clatter_time(fighter.module_accessor, 0) <= 0.0 {
        /*
        if situation_kind != *SITUATION_KIND_GROUND {
            return 0.into();
        }
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_SONG_END.into(), false.into());  
        */
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_SONG_END.into(), false.into());  
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageSong)]
unsafe fn status_end_damagesong(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x15a773446b), false);
    fighter.sub_DamageSongExit();
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damagesong_main,
            status_end_damagesong
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}