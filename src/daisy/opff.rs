use super::*;

unsafe extern "C" fn daisy_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_JUMP_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&false.into());
}

pub fn install() {
    Agent::new("daisy")
    .on_start(daisy_init)
    .install()
    ;
}