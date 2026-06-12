use super::*;

unsafe extern "C" fn link_bowarrow_haved_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ModelModule::set_scale(boma, 1.0);
    MotionModule::change_motion(boma, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(link_bowarrow_haved_main_loop as *const () as _))
}

unsafe extern "C" fn link_bowarrow_haved_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let boma = weapon.module_accessor;
    let arrow_type = WorkModule::get_int(boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
        if frame % 15.0 == 0.0 {
            EFFECT_FOLLOW(weapon, Hash40::new("link_light_arrow_charge"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WN_LINK_BOWARROW_STATUS_KIND_HAVED, link_bowarrow_haved_main_status)
    .install()
    ;
}