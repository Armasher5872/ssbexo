use super::*;

unsafe extern "C" fn snake_special_n_hold_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.status_ItemShootAir_before();
    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.sub_ItemShootAir_Common();
    fighter.sub_ItemShootJump_enable_aerial();
    if WorkModule::is_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_FLAG_CYPHER_FALL) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
    WorkModule::set_float(boma, -1.0, *FIGHTER_SNAKE_STATUS_SPECIAL_N_HOLD_WAIT_WORK_FLOAT_THROW_RATE);
    if !StopModule::is_stop(boma) {
        fun_710001bc90(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001bc90 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_hold_air_main_loop as *const () as _))
}

unsafe extern "C" fn snake_special_n_hold_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.sub_ItemShootAir_Common_Main().get_bool() {
        if !fighter.sub_ItemShootJumpCommon_Main().get_bool() {
            fighter.sub_ftStatusUniqProcessItemShoot_execFixPos_Common();
            return 0.into();
        }
    }
    1.into()
}

unsafe extern "C" fn snake_special_n_hold_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fun_710001b2a0(fighter);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR, snake_special_n_hold_air_main_status)
    .status(End, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR, snake_special_n_hold_air_end_status)
    .install()
    ;
}