use super::*;

//Catch Pull Pre Status
unsafe extern "C" fn donkey_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let situation = if prev_status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO {
        *SITUATION_KIND_AIR
    }
    else {
        *SITUATION_KIND_GROUND
    };
    let kinetic_type = if prev_status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO {
        *FIGHTER_KINETIC_TYPE_RESET
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION
    };
    StatusModule::init_settings(fighter.module_accessor, SituationKind(situation), kinetic_type, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Catch Pull Main Status
unsafe extern "C" fn donkey_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_AIR_LASSO {
        let throw_hi_status_kind = fighter.global_table[THROW_HI_STATUS_KIND].get_i32();
        fighter.change_status(throw_hi_status_kind.into(), false.into());
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, donkey_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, donkey_catch_pull_main_status)
    .install()
    ;
}