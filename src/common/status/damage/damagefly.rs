use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFly_Main)]
unsafe fn status_damagefly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    asdi_check(fighter);
    asdi_function(fighter);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    }
    //Fighter Specific
    if fighter_kind == *FIGHTER_KIND_PICKEL {
        if fighter.global_table[IS_STOP].get_bool() {
            macros::COL_NORMAL(fighter);
            macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.25);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if !fighter.status_DamageFinishCamera_exec().get_bool() {
            return 0.into();
        }
        fighter.status_DamageFly_Common();
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ADJUST_VECTOR);
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) {
            if MotionModule::is_end(fighter.module_accessor) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                    fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
                }
            }
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(fighter.module_accessor) {
            if fighter.sub_AirChkDamageReflectWall().get_bool() || fighter.sub_AirChkDamageReflectCeil().get_bool() || fighter.sub_AirChkDamageReflectFloor().get_bool() {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageFly)]
unsafe fn status_end_damagefly(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    //Fighter Specific
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY {
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            fighter.sub_end_damage_fly_roll_to_dead();
        }
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_DISABLE_PASSIVE);
    }
    fighter.check_ryu_final_damage_03(true.into());
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damagefly_main,
            status_end_damagefly
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}