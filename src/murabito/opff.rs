use super::*;

unsafe extern "C" fn murabito_frame(fighter: &mut L2CFighterCommon) {
    ac_common(fighter);
}

pub fn install() {
    Agent::new("murabito")
    .on_line(Main, murabito_frame)
    .install()
    ;
}