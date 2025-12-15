use super::*;

unsafe extern "C" fn link_special_s_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, link_special_s_init_status)
    .install()
    ;
}