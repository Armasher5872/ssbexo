use super::*;

unsafe extern "C" fn sonic_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(boma) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn sonic_check_special_n_uniq(_fighter: &mut L2CFighterCommon) -> L2CValue {
    false.into()
}

unsafe extern "C" fn sonic_check_ground_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    0.into()
}

unsafe extern "C" fn sonic_check_air_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    0.into()
}

unsafe extern "C" fn sonic_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    sonic_var(&mut *boma);
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(sonic_check_special_n_uniq as *const () as _));
    fighter.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(sonic_check_ground_special_uniq as *const () as _));
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(sonic_check_air_special_uniq as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sonic_end_control as *const () as _));
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(sonic_on_start)
    .install()
    ;
}