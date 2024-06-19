use super::*;

unsafe extern "C" fn shizue_frame(fighter: &mut L2CFighterCommon) {
    ac_common(fighter);
}

pub fn install() {
    Agent::new("shizue")
    .on_line(Main, shizue_frame)
    .install()
    ;
}