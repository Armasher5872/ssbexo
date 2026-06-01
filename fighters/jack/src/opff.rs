use super::*;

unsafe extern "C" fn jack_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        WorkModule::on_flag(boma, 0x200000E4);
        FighterSpecializer_Jack::check_doyle_summon_dispatch(boma, true, false);
        StatusModule::set_status_kind_interrupt(boma, *FIGHTER_JACK_STATUS_KIND_DISPATCH);
    }
    else {
        WorkModule::on_flag(boma, 0x200000E3);
        FighterSpecializer_Jack::check_doyle_summon_dispatch(boma, true, false);
        StatusModule::set_status_kind_interrupt(boma, *FIGHTER_JACK_STATUS_KIND_SUMMON);
    }
    1.into()
}

//Set Move Customizer is accredited to WuBor Patch
unsafe extern "C" fn jack_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let waza_customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
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

unsafe extern "C" fn jack_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    set_move_customizer(fighter, jack_waza_customize);
    jack_waza_customize(fighter);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

unsafe extern "C" fn jack_doyle_frame(weapon: &mut L2CWeaponCommon) {
    WorkModule::set_float(weapon.module_accessor, WorkModule::get_float(get_owner_boma(weapon), 0x4D), 0x6);
}

pub fn install() {
    Agent::new("jack")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(jack_on_start)
    .install()
    ;
    Agent::new("jack_doyle")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, jack_doyle_frame)
    .install()
    ;
}