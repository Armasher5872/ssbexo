use super::*;

pub unsafe extern "C" fn beam_removal(weapon: &mut L2CWeaponCommon) {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("metaknight_beam"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
}