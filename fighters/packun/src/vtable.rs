use super::*;

const PACKUN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe09f70; //Piranha Plant only

//Piranha Plant Reset Initialization
unsafe extern "C" fn packun_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Piranha Plant Death Initialization
#[skyline::hook(offset = PACKUN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn packun_death_initialization(_vtable: u64, fighter: &mut Fighter, param_3: i32) {
    let boma = fighter.battle_object.module_accessor;
    let special_s_charge_max_effect_handle = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE_MAX_EFFECT_HANDLE);
    if param_3 == 7 && special_s_charge_max_effect_handle != 0 {
        EffectModule::remove(boma, special_s_charge_max_effect_handle as u32, 0);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE_MAX_EFFECT_HANDLE);
    WorkModule::off_flag(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_LANDING);
    WorkModule::off_flag(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_WAIT_END);
    WorkModule::set_int(boma, 0, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT);
    EffectModule::remove_common(boma, Hash40::new("charge_max"));
    VisibilityModule::set_visibility_mode(boma, VisibilityMode{_address: 0});
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fff248).data(packun_reset_initialization as *const () as u64);
	skyline::install_hook!(packun_death_initialization);
}