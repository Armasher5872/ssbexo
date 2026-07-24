use super::*;

//Chrom Reset Initialization
unsafe extern "C" fn chrom_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Chrom Death Initialization
unsafe extern "C" fn chrom_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SPECIAL_HI_CLIFF_NUM);
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5030f78).data(chrom_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x5030f90).data(chrom_death_initialization as *const () as u64);
}