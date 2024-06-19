use super::*;

unsafe extern "C" fn ness_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
}

pub fn install() {
    Agent::new("ness")
    .on_start(ness_init)
    .install()
    ;
}