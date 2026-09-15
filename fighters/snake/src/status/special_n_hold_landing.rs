use super::*;

unsafe extern "C" fn snake_special_n_hold_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ItemShootLanding_Common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_hold_landing_main_loop as *const () as _))
}

unsafe extern "C" fn snake_special_n_hold_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fun_710001bb20(fighter).get_bool() {
        if !fighter.sub_ItemShootLanding_Common_Main().get_bool(){
            fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();
            return 0.into();
        }
    }
    1.into()
}

unsafe extern "C" fn snake_special_n_hold_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710001b2a0(fighter);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, snake_special_n_hold_landing_main_status)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, snake_special_n_hold_landing_end_status)
    .install()
    ;
}