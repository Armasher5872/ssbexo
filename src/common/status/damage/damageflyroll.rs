use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_DamageFlyRoll_Main)]
unsafe fn status_damageflyroll_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
        if (ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
        && (ControlModule::get_stick_y(fighter.module_accessor) < -0.66) 
        && (KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= -0.5) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_DamageFlyRoll)]
unsafe fn status_end_damageflyroll(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    //Fighter Specific
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    if status_kind == *FIGHTER_STATUS_KIND_DEAD {
        fighter.sub_end_damage_fly_roll_to_dead();
    }
    fighter.check_ryu_final_damage_03(true.into());
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_damageflyroll_main,
            status_end_damageflyroll
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}