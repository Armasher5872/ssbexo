use super::*;

unsafe extern "C" fn rockman_rockbuster_shoot_jump_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_JUMP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, true, (*FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_AIR_N | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, (*FIGHTER_STATUS_ATTR_SCALE_KINETIC_ENERGY | *FIGHTER_STATUS_ATTR_INTO_DOOR) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    rockman_rockbuster_main_helper(fighter, true.into(), false.into(), L2CValue::Void(), L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_rockbuster_shoot_jump_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let helper_ret = rockman_rockbuster_main_loop_helper(fighter, true.into(), false.into()).get_bool();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if helper_ret {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if sit == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_LANDING.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn rockman_rockbuster_shoot_jump_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if !rockman_rockbuster_pre_helper(status_kind.into()).get_bool() {
        rockman_rockbuster_end_var_reset(fighter);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
    }
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .status(Pre, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, rockman_rockbuster_shoot_jump_pre_status)
    .status(Main, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, rockman_rockbuster_shoot_jump_main_status)
    .status(End, *FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_JUMP, rockman_rockbuster_shoot_jump_end_status)
    .install()
    ;
}