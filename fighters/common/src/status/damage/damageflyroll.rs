use super::*;

//Enables fastfalling during tumble
#[skyline::hook(replace = L2CFighterCommon_status_DamageFlyRoll_Main)]
unsafe extern "C" fn status_damageflyroll_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
        if (ControlModule::get_command_flag_cat(boma, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
        && (ControlModule::get_stick_y(boma) < -0.66) 
        && (KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= -0.5) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
        if !fighter.status_DamageFinishCamera_exec().get_bool() {
            return 0.into();
        }
        fighter.status_DamageFly_Common();
        WorkModule::off_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_ADJUST_VECTOR);
    }
    else {
        if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL) {
            if MotionModule::is_end(boma) {
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                    fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
                }
            }
        }
        if fighter.sub_DamageFlyCommon().get_bool() {
            return 0.into();
        }
        if !FighterStopModuleImpl::is_damage_stop(boma) {
            if fighter.sub_AirChkDamageReflectWall().get_bool() || fighter.sub_AirChkDamageReflectCeil().get_bool() || fighter.sub_AirChkDamageReflectFloor().get_bool() {
                return 0.into();
            }
        }
        fighter.FighterStatusDamage__correctDamageVectorEffect(false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(status_damageflyroll_main);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}