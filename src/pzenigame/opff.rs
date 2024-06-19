use super::*;

unsafe extern "C" fn pzenigame_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
}

pub fn install() {
    Agent::new("pzenigame")
    .on_start(pzenigame_init)
    .install()
    ;
}