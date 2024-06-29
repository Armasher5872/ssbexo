//Set Move Customizer is accredited to WuBor Patch
use super::*;

unsafe extern "C" fn jack_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let waza_customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if [*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1, *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2].contains(&waza_customize_to) {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(jack_special_lw_pre_status as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

unsafe extern "C" fn jack_init(fighter: &mut L2CFighterCommon) {
    set_move_customizer(fighter, jack_waza_customize);
    jack_waza_customize(fighter);
}

unsafe extern "C" fn jack_doyle_frame(weapon: &mut L2CWeaponCommon) {
    WorkModule::set_float(weapon.module_accessor, WorkModule::get_float(get_owner_boma(weapon), 0x4D), 0x6);
}

pub fn install() {
    Agent::new("jack")
    .on_start(jack_init)
    .install()
    ;
    Agent::new("jack_doyle")
    .on_line(Main, jack_doyle_frame)
    .install()
    ;
}