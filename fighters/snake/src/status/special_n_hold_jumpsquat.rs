use super::*;

unsafe extern "C" fn snake_special_n_hold_jumpsquat_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_ItemShootJumpSquat_Common();
    if !StopModule::is_stop(boma) {
        fighter.sub_item_shoot_jump_squat_uniq_check();
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_item_shoot_jump_squat_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_hold_jumpsquat_main_loop as *const () as _))
}

unsafe extern "C" fn snake_special_n_hold_jumpsquat_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_ItemShootJumpSquat_Common_Main().get_bool() {
        fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();
    }
    0.into()
}

unsafe extern "C" fn snake_special_n_hold_jumpsquat_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710001b2a0(fighter);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT, snake_special_n_hold_jumpsquat_main_status)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT, snake_special_n_hold_jumpsquat_end_status)
    .install()
    ;
}