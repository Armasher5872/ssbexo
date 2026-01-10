use super::*;

//Cloud Guard Pre Status
unsafe extern "C" fn cloud_guard_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_GUARD_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY as u32, 0, 0);
    0.into()
}

//Cloud Guard Init Status
unsafe extern "C" fn cloud_guard_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let guard_hit_stop_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, guard_hit_stop_frame);
    0.into()
}

//Cloud Guard Main Status
unsafe extern "C" fn cloud_guard_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("punish_guard"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_guard_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let guard_cooldown = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if !fighter.sub_guard_cont().get_bool() {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD_OFF.into(), true.into());
                return 1.into();
            }
        }
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && guard_cooldown == 0 {
        WorkModule::set_int(fighter.module_accessor, 60, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
        ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_muzzleflash"), Hash40::new("waist"), &Vector3f::zero(), &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 1.0, false, 0, 0, 0, 0, 0, false, false);
    }
    if guard_cooldown == 40 {
        ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD.into(), true.into());
    }
    0.into()
}

//Guard Exec Status
unsafe extern "C" fn cloud_guard_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let guard_cooldown = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
    if guard_cooldown > 0 {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
    }
    0.into()
}

//Guard End Status
unsafe extern "C" fn cloud_guard_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
    0.into()
}

//Guard Exit Status
unsafe extern "C" fn cloud_guard_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ShieldModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_GUARD_COOLDOWN);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_main_status)
    .status(Exec, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_exec_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_end_status)
    .status(Exit, *FIGHTER_CLOUD_STATUS_KIND_GUARD, cloud_guard_exit_status)
    .install()
    ;
}