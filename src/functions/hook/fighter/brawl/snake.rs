use super::*;

const SNAKE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11b5710; //Snake only
const SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x11b5720; //Snake only
const SNAKE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11b5890; //Snake only
pub static mut SNAKE_GRENADE_STATUS_FALL_STATUS: usize = 0x7c9ae0;
pub static mut SNAKE_GRENADE_STATUS_LANDING_STATUS: usize = 0x7c9d10;
pub static mut SNAKE_GRENADE_STATUS_THROWN_STATUS: usize = 0x7c9fc0;

//Snake Startup Initialization
#[skyline::hook(offset = SNAKE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    original!()(vtable, fighter)
}

//Snake Reset Initialization
#[skyline::hook(offset = SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    original!()(vtable, fighter)
}

//Snake Death Initialization
#[skyline::hook(offset = SNAKE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    original!()(vtable, fighter)
}

//Credited to WuBoyTH
#[skyline::hook(replace = SNAKE_GRENADE_STATUS_FALL_STATUS)]
unsafe extern "C" fn snake_grenade_status_fall_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

#[skyline::hook(replace = SNAKE_GRENADE_STATUS_LANDING_STATUS)]
unsafe extern "C" fn snake_grenade_status_landing_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

#[skyline::hook(replace = SNAKE_GRENADE_STATUS_THROWN_STATUS)]
unsafe extern "C" fn snake_grenade_status_thrown_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let item_offset = (*info.module.ModuleObject).module_base as usize;
            let _ = skyline::patching::patch_pointer((0x7ca48c+item_offset) as *const u8, &0x529FE608u32);
            SNAKE_GRENADE_STATUS_FALL_STATUS += item_offset;
            SNAKE_GRENADE_STATUS_LANDING_STATUS += item_offset;
            SNAKE_GRENADE_STATUS_THROWN_STATUS += item_offset;
            skyline::install_hooks!(
                snake_grenade_status_fall_status,
                snake_grenade_status_landing_status,
                snake_grenade_status_thrown_status
            );
        }
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        snake_start_initialization,
        snake_reset_initialization,
        snake_death_initialization
    );
}