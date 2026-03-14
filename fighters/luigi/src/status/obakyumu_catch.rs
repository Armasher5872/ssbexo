use super::*;

unsafe extern "C" fn luigi_obakyumu_catch_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    if owner_lr == -1.0 {
        MotionModule::set_flip(weapon.module_accessor, false, true, true);
    }
    weapon.fastshift(L2CValue::Ptr(luigi_obakyumu_catch_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_obakyumu_catch_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("luigi_obakyumu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_LUIGI_OBAKYUMU_STATUS_KIND_CATCH, luigi_obakyumu_catch_main_status)
    .install()
    ;
}