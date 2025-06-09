use super::*;

unsafe extern "C" fn cloud_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_sub_attack_s4(fighter, true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4_Main as *const () as _))
}

unsafe extern "C" fn cloud_sub_attack_s4(fighter: &mut L2CFighterCommon, angled_smash_attack: L2CValue) {
    let get_stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
    let attack_s4_stick_dir_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s4_stick_dir_hi"));
    let attack_s4_stick_dir_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s4_stick_dir_lw"));
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_s4_s"} else {"attack_s4_s"};
    let motion_hi = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_s4_hi"} else {"attack_s4_hi"};
    let motion_lw = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_attack_s4_lw"} else {"attack_s4_lw"};
    if !angled_smash_attack.get_bool() {
        WorkModule::set_int64(fighter.module_accessor, hash40(motion) as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
    }
    else {
        if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new(motion_hi)) {
            if attack_s4_stick_dir_hi < get_stick_dir {
                WorkModule::set_int64(fighter.module_accessor, hash40(motion_hi) as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
            }
        }
        if MotionModule::is_anim_resource(fighter.module_accessor, Hash40::new(motion_lw)) {
            if get_stick_dir < attack_s4_stick_dir_lw {
                WorkModule::set_int64(fighter.module_accessor, hash40(motion_lw) as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
            }
        }
        WorkModule::set_int64(fighter.module_accessor, hash40(motion) as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_MOTION_KIND);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, cloud_attack_s4_main_status)
    .install()
    ;
}