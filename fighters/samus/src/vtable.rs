use super::*;

const SAMUS_SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10f3630; //Shared
const SAMUS_SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10f3650; //Shared
const SAMUS_SAMUSD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10f37a0; //Shared

//Samus & Dark Samus Reset Initialization
#[skyline::hook(offset = SAMUS_SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samus_samusd_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_SAMUS {
        WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_HANG_DATA);
    }
    if kind == *FIGHTER_KIND_SAMUSD {
        WorkModule::set_int(boma, 0, *FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    }
    common_reset_variable_reset(&mut *boma);
}

//Samus & Dark Samus Death Initialization
#[skyline::hook(offset = SAMUS_SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samus_samusd_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_SAMUSD {
        WorkModule::set_int(boma, 0, *FIGHTER_SAMUSD_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    }
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Samus & Dark Samus Once Per Fighter Frame
#[skyline::hook(offset = SAMUS_SAMUSD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn samus_samusd_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUS as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind) && StatusModule::is_situation_changed(boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            WorkModule::set_float(boma, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        samus_samusd_reset_initialization,
        samus_samusd_death_initialization,
        samus_samusd_opff
    );
}