use super::*;

unsafe extern "C" fn snake_special_n_hold_walk_brake_b_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_ItemShootWalkBBrake_Common();
    if !StopModule::is_stop(boma) {
        fun_710001bc90(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001bc90 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_hold_walk_brake_b_main_loop as *const () as _))
}

unsafe extern "C" fn snake_special_n_hold_walk_brake_b_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fun_710001bb20(fighter).get_bool() {
        if !fighter.sub_ItemShootWalkBBrake_Common_Main().get_bool(){
            fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();
            return 0.into();
        }
    }
    1.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_B, snake_special_n_hold_walk_brake_b_main_status)
    .install()
    ;
}