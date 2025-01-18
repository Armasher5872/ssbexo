use super::*;

const MARTH_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xcd98a0; //Shared
const MARTH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc732a0; //Shared
const MARTH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcd99a0; //Shared
const MARTH_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x68d670; //Shared

//Marth Startup Initialization
#[skyline::hook(offset = MARTH_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Marth Reset Initialization
#[skyline::hook(offset = MARTH_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
    }
    original!()(vtable, fighter)
}

//Marth Death Initialization
#[skyline::hook(offset = MARTH_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn marth_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
    }
    original!()(vtable, fighter)
}

//Marth Once Per Fighter Frame
#[skyline::hook(offset = MARTH_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn marth_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let long_sword_scale = Vector3f{x: 1.0, y: 1.2, z: 1.0};
    if fighter.battle_object.kind == *FIGHTER_KIND_MARTH as u32 {
        ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        marth_start_initialization,
        marth_reset_initialization,
        marth_death_initialization,
        marth_opff
    );
}