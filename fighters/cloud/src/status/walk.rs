use super::*;

unsafe extern "C" fn cloud_walk_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let fast_walk_motion = if is_punisher {"punish_walk_fast"} else {"walk_fast"};
    let middle_walk_motion = if is_punisher {"punish_walk_middle"} else {"walk_middle"};
    let slow_walk_motion = if is_punisher {"punish_walk_slow"} else {"walk_slow"};
    fighter.sub_walk_uniq_process_init_common(hash40(fast_walk_motion).into(), hash40(middle_walk_motion).into(), hash40(slow_walk_motion).into(), false.into());
    0.into()
}

unsafe extern "C" fn cloud_walk_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_punisher = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE);
    let fast_walk_motion = if is_punisher {"punish_walk_fast"} else {"walk_fast"};
    let middle_walk_motion = if is_punisher {"punish_walk_middle"} else {"walk_middle"};
    let slow_walk_motion = if is_punisher {"punish_walk_slow"} else {"walk_slow"};
    fighter.sub_walk_uniq_process_main_common(hash40(fast_walk_motion).into(), hash40(middle_walk_motion).into(), hash40(slow_walk_motion).into(), L2CValue::Ptr(cloud_walk_exec_mot_helper as *const () as _));
    0.into()
}

unsafe extern "C" fn cloud_walk_exec_mot_helper(walk_motion: L2CValue, current_motion: L2CValue) -> L2CValue {
    let curr = current_motion.get_u64();
    let walk = walk_motion.get_u64();
    if curr == hash40("walk_fast")
    && walk == hash40("punish_walk_fast") {
        return true.into();
    }
    if curr == hash40("walk_middle")
    && walk == hash40("punish_walk_middle") {
        return true.into();
    }
    if curr == hash40("walk_slow")
    && walk == hash40("punish_walk_slow") {
        return true.into();
    }
    false.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_WALK, cloud_walk_init_status)
    .status(Exec, *FIGHTER_STATUS_KIND_WALK, cloud_walk_exec_status)
    .install()
    ;
}