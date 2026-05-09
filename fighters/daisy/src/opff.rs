use super::*;

unsafe extern "C" fn daisy_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    fighter.global_table[CHECK_AIR_JUMP_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&false.into());
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("daisy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(daisy_on_start)
    .install()
    ;
}