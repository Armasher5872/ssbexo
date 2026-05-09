use super::*;

const DOLLY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x970900; //Terry only
const DOLLY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x970e40; //Terry only
const DOLLY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x971c80; //Terry only

//Terry Reset Initialization
#[skyline::hook(offset = DOLLY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dolly_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    original!()(vtable, fighter)
}

//Terry Death Initialization
#[skyline::hook(offset = DOLLY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn dolly_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    original!()(vtable, fighter)
}

//Terry Once Per Fighter Frame
#[skyline::hook(offset = DOLLY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn dolly_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED)
    && situation_kind != *SITUATION_KIND_AIR {
        if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND].contains(&status_kind)
        && frame >= 9.0
        && !cancel_statuses {
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND
        && frame >= 11.0
        && !cancel_statuses {
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND
        && frame >= 5.0
        && !cancel_statuses {
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
        if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
        && frame >= 13.0
        && !cancel_statuses {
            WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        dolly_reset_initialization,
        dolly_death_initialization,
        dolly_opff
    );
}