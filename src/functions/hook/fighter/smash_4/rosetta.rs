use super::*;

const ROSETTA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10a81a0; //Rosalina & Luma only
const ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10a88e0; //Rosalina & Luma only
const ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x10a8f60; //Rosalina & Luma only

unsafe extern "C" fn rosetta_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::set_float(boma, 0.0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_X);
    WorkModule::set_float(boma, 0.0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_Y);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID);
    WorkModule::set_int(boma, 0, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
}

//Rosalina & Luma Startup Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    rosetta_var(&mut *boma);
    original!()(vtable, fighter)
}

//Rosalina & Luma Reset Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROSETTA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        rosetta_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Rosalina & Luma Death Initialization
#[skyline::hook(offset = ROSETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn rosetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    rosetta_var(&mut *boma);
    original!()(vtable, fighter)
}

//Rosalina & Luma Once Per Fighter Frame
#[skyline::hook(offset = ROSETTA_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn rosetta_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let transit_timer = WorkModule::get_int(boma, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if transit_timer > 0 {
        WorkModule::dec_int(boma, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    }
    if transit_timer == 1 {
        fighter.battle_object.gimmick_flash();
    }
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let obj_id = WorkModule::get_int(boma, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let weapon = get_weapon_common_from_accessor(&mut *obj_boma);
        let item_id = if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            WorkModule::get_int64(obj_boma, WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
            WorkModule::get_int64(obj_boma, WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else {
            *BATTLE_OBJECT_ID_INVALID as u32
        };
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
        if is_slash(obj_boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
            slash_removal(weapon);
        }
        if is_galaxia(obj_boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
            galaxia_beam_removal(weapon);
        }
        if is_sludge(obj_boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
            sludge_removal(weapon);
        }
        if is_disarming_voice(obj_boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
            disarming_voice_removal(weapon);
        }
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