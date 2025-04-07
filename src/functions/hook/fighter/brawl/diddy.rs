use super::*;

const DIDDY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const DIDDY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const DIDDY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x956360; //Diddy only
const DIDDY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x956600; //Diddy only

//Diddy Kong Startup Initialization
#[skyline::hook(offset = DIDDY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn diddy_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_DIDDY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        common_initialization_variable_reset(&mut *boma);
        BANANA_EXIST[entry_id] = false;
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Diddy Kong Reset Initialization
#[skyline::hook(offset = DIDDY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn diddy_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_DIDDY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        common_reset_variable_reset(&mut *boma);
        BANANA_EXIST[entry_id] = false;
    }
    original!()(vtable, fighter)
}

//Diddy Kong Death Initialization
#[skyline::hook(offset = DIDDY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn diddy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    common_death_variable_reset(&mut *boma);
    BANANA_EXIST[entry_id] = false;
    original!()(vtable, fighter)
}

//Diddy Once Per Fighter Frame
#[skyline::hook(offset = DIDDY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn diddy_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let boma_opponent1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
    let status_kind_opponent1 = StatusModule::status_kind(boma_opponent1);
    let boma_opponent2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
    let status_kind_opponent2 = StatusModule::status_kind(boma_opponent2);
    let boma_opponent3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
    let status_kind_opponent3 = StatusModule::status_kind(boma_opponent3);
    let boma_opponent4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
    let status_kind_opponent4 = StatusModule::status_kind(boma_opponent4);
    let boma_opponent5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
    let status_kind_opponent5 = StatusModule::status_kind(boma_opponent5);
    let boma_opponent6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
    let status_kind_opponent6 = StatusModule::status_kind(boma_opponent6);
    let boma_opponent7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
    let status_kind_opponent7 = StatusModule::status_kind(boma_opponent7);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let banana_id = WorkModule::get_int(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
    if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && !BANANA_EXIST[entry_id] {
        BANANA_EXIST[entry_id] = true;
    }
    if status_kind_opponent1 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent2 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent3 == *FIGHTER_STATUS_KIND_SLIP 
    || status_kind_opponent4 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent5 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent6 == *FIGHTER_STATUS_KIND_SLIP 
    || status_kind_opponent7 == *FIGHTER_STATUS_KIND_SLIP && BANANA_EXIST[entry_id] {
        WorkModule::set_int(boma, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        BANANA_EXIST[entry_id] = false;
        StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
    }
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_diddy_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        diddy_start_initialization,
        diddy_reset_initialization,
        diddy_death_initialization,
        diddy_opff
    );
}