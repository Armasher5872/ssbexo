//Credited to WuBor Patch (Does the team set code even work???)
use super::*;

pub static mut SNAKE_GRENADE_FALL_STATUS_OFFSET: usize = 0x7c9ae0;
pub static mut SNAKE_GRENADE_LANDING_STATUS_OFFSET: usize = 0x7c9d10;
pub static mut SNAKE_GRENADE_THROWN_STATUS_OFFSET: usize = 0x7c9fc0;

//Snake Grenade Fall Status Hook
#[skyline::hook(offset = SNAKE_GRENADE_FALL_STATUS_OFFSET)]
unsafe extern "C" fn snake_grenade_fall_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, *TEAM_NONE);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

//Snake Grenade Landing Status Hook
#[skyline::hook(offset = SNAKE_GRENADE_LANDING_STATUS_OFFSET)]
unsafe extern "C" fn snake_grenade_landing_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, *TEAM_NONE);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    disable_area(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    original!()(item)
}

//Snake Grenade Thrown Status Hook
#[skyline::hook(offset = SNAKE_GRENADE_THROWN_STATUS_OFFSET)]
unsafe extern "C" fn snake_grenade_thrown_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, *TEAM_NONE);
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
            skyline::patching::patch_pointer((0x7ca48c + item_offset) as *const u8, &0x529FE608u32);
            SNAKE_GRENADE_FALL_STATUS_OFFSET += item_offset;
            SNAKE_GRENADE_LANDING_STATUS_OFFSET += item_offset;
            SNAKE_GRENADE_THROWN_STATUS_OFFSET += item_offset;
            skyline::install_hooks!(
                snake_grenade_fall_status,
                snake_grenade_landing_status,
                snake_grenade_thrown_status
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}