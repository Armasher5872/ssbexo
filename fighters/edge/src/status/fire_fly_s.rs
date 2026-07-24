use super::*;

unsafe extern "C" fn edge_fire_s_fly_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let charge_kind = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    if charge_kind == *FIGHTER_EDGE_SPECIAL_N_XL {
        weapon.change_status(WEAPON_EDGE_FIRE_STATUS_KIND_FLY_XL.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, edge_fire_s_fly_init_status)
    .install()
    ;
}