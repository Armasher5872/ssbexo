use super::*;

unsafe extern "C" fn eflame_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("eflame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(eflame_on_start)
    .install()
    ;
}