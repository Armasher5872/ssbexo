use super::*;

const ROSETTA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10a81a0; //Rosalina & Luma only
const ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10a88e0; //Rosalina & Luma only
const ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x10a8f60; //Rosalina & Luma only

//Rosalina & Luma Startup Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Rosalina & Luma Reset Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROSETTA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Rosalina & Luma Death Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Rosalina & Luma Once Per Fighter Frame
#[skyline::hook(offset = ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn rosetta_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let obj_id = WorkModule::get_int(boma, 0x11000006) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let item_id = if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            WorkModule::get_int64(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else {
            *BATTLE_OBJECT_ID_INVALID as u32
        };
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        rosetta_start_initialization,
        rosetta_reset_initialization,
        rosetta_death_initialization,
        rosetta_opff
    );
}