use super::*;

const SAMUS_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const SAMUS_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10f3630; //Shared
const SAMUS_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10f3650; //Shared
const SAMUS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10f37a0; //Shared

//Samus Startup Initialization
#[skyline::hook(offset = SAMUS_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samus_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUS as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Samus Reset Initialization
#[skyline::hook(offset = SAMUS_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samus_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_HANG_DATA);
    }
    original!()(vtable, fighter)
}

//Samus Death Initialization
#[skyline::hook(offset = SAMUS_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samus_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUS as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Samus Once Per Fighter Frame
#[skyline::hook(offset = SAMUS_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn samus_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUS as u32 {
        if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind) && StatusModule::is_situation_changed(boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            WorkModule::set_float(boma, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        samus_start_initialization,
        samus_reset_initialization,
        samus_death_initialization,
        samus_opff
    );
}