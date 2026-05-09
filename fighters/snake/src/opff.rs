use super::*;

unsafe extern "C" fn snake_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(snake_on_start)
    .install()
    ;
}