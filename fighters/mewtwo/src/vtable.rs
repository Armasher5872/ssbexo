use super::*;

const MEWTWO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xd294e0; //Mewtwo only

//Mewtwo Reset Initialization
unsafe extern "C" fn mewtwo_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Mewtwo Death Initialization
#[skyline::hook(offset = MEWTWO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mewtwo_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SHADOWBALL_HAD);
    WorkModule::off_flag(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BUOYANCY);
    WorkModule::off_flag(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_REFLECT);
    WorkModule::off_flag(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_REFLECTOR_BREAK);
    WorkModule::off_flag(boma, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_SET_THROW_POS);
    WorkModule::set_int(boma, 0, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME);
    EffectModule::remove_common(boma, Hash40::new("charge_max"));
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fed120).data(mewtwo_reset_initialization as *const () as u64);
	skyline::install_hook!(mewtwo_death_initialization);
}