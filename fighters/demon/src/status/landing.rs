use super::*;

unsafe extern "C" fn demon_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingSub();
    fighter.status_LandingStiffness();
    fighter.sub_landing_start_check_damage_face();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_landing_main_loop as *const () as _))
}

unsafe extern "C" fn demon_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let is_attack = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
    if is_attack {
        fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7.into(), false.into());
        return 0.into();
    }
    fighter.status_Landing_MainSub()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_LANDING, demon_landing_main_status)
    .install()
    ;
}