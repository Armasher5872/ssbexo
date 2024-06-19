//The reason all the opffs are in this file instead of their respective fighters is because for some reason, the game won't allow the auras to work if I put them in there
use super::*;

unsafe extern "C" fn plizardon_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
}

pub fn install() {
    Agent::new("plizardon")
    .on_start(plizardon_init)
    .install()
    ;
}