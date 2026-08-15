use super::*;

//Roy & Chrom Reset Initialization
unsafe extern "C" fn roy_chrom_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_ROY {
        roy_var(&mut *boma);
    }
    common_reset_variable_reset(&mut *boma);
}

//Roy & Chrom Death Initialization
unsafe extern "C" fn roy_chrom_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_ROY {
        roy_var(&mut *boma);
    }
    WorkModule::set_int(boma, 0, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SPECIAL_HI_CLIFF_NUM);
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5030f78).data(roy_chrom_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x5030f90).data(roy_chrom_death_initialization as *const () as u64);
}