use super::*;

unsafe extern "C" fn jack_doyle_frame(weapon: &mut L2CWeaponCommon) {
    WorkModule::set_float(weapon.module_accessor, WorkModule::get_float(get_owner_boma(weapon), 0x4D), 0x6);
}

pub fn install() {
    Agent::new("jack_doyle")
    .on_line(Main, jack_doyle_frame)
    .install()
    ;
}