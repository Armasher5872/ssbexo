use super::*;

//Peach & Daisy Reset Initialization
unsafe extern "C" fn peach_daisy_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
}

//Peach & Daisy Death Initialization
unsafe extern "C" fn peach_daisy_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_PEACH {
        WorkModule::off_flag(boma, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT_RAY_CHECK);
        WorkModule::off_flag(boma, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_JUMP_FROM_WATER);
        WorkModule::set_float(boma, 0.0, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLOAT_JUMP_START_Y_FROM_WATER);
        WorkModule::set_int(boma, 0, *FIGHTER_PEACH_INSTANCE_WORK_ID_INT_KINOPIOSPORE_SHOOT_NUM);
    }
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x5008a38).data(peach_daisy_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x5008a50).data(peach_daisy_death_initialization as *const () as u64);
}