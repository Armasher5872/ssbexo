use super::*;

const FOX_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa61650; //Shared
const FOX_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa617c0; //Shared
const FOX_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa62210; //Shared
const FOX_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa62480; //Shared

//Fox Startup Initialization
#[skyline::hook(offset = FOX_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Fox Reset Initialization
#[skyline::hook(offset = FOX_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Fox Death Initialization
#[skyline::hook(offset = FOX_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Fox Once Per Fighter Frame
#[skyline::hook(offset = FOX_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn fox_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    if fighter.battle_object.kind == *FIGHTER_KIND_FOX as u32 {    
        //Jab Cancels
        if fighter.battle_object.magic_series() == 1 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        }
        if fighter.battle_object.magic_series() == 2 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        }
        if fighter.battle_object.magic_series() == 3 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        }
        //Fast Fall Blaster/Land Cancel Blaster (Not doing it in statuses yet)
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if situation_kind == *SITUATION_KIND_AIR && fighter.battle_object.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            };
            if StatusModule::is_situation_changed(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
            };
        }
        //Cancelable Up Special Bonk
        if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND {
            if frame > 10.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        fox_start_initialization,
        fox_reset_initialization,
        fox_death_initialization,
        fox_opff
    );
}