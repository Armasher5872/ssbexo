use super::*;

unsafe extern "C" fn donkey_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[THROW_F_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW.into());
	fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START.into());
}

pub fn install() {
    Agent::new("donkey")
    .on_start(donkey_init)
    .install()
    ;
}