use super::*;

unsafe extern "C" fn snake_special_n_hold_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_ItemShootAir_Common();
    fighter.sub_ItemShootJump_enable_aerial();
    if !StopModule::is_stop(boma) {
        fun_710001bc90(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001bc90 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_hold_jump_aerial_main_loop as *const () as _))
}

unsafe extern "C" fn snake_special_n_hold_jump_aerial_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if !fighter.sub_ItemShootAir_Common_Main().get_bool() {
        if !fighter.sub_ItemShootDashF_Common_Main().get_bool() {
            if MotionModule::is_end(boma) {
                fighter.change_status(WorkModule::get_int(boma, *FIGHTER_STATUS_ITEM_SHOOT_WORK_INT_STATUS_KIND_AIR).into(), false.into());
            }
            fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();
            return 0.into();
        }
    }
    1.into()
}

unsafe extern "C" fn snake_special_n_hold_jump_aerial_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710001b2a0(fighter);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL, snake_special_n_hold_jump_aerial_main_status)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL, snake_special_n_hold_jump_aerial_end_status)
    .install()
    ;
}