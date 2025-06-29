use super::*;

#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x159fb20)]
pub unsafe extern "C" fn set_stage_visibility(module_accessor: *mut smash::app::BattleObjectModuleAccessor, param_2: u32);

#[skyline::from_offset(0x6dd2a0)]
pub unsafe extern "C" fn waza_customize(lua_module: u64, status: i32, customize_to: i32);

#[skyline::from_offset(0x37ae1a0)]
pub unsafe extern "C" fn kill_dead_event_listeners(arg: *mut u32);

#[skyline::from_offset(0x6924e0)]
pub unsafe extern "C" fn add_water_damage(boma: *mut BattleObjectModuleAccessor, work_id_const: i32);

#[skyline::from_offset(0xb0bbd0)]
pub unsafe extern "C" fn inkling_handle_tank_fill(boma: *mut BattleObjectModuleAccessor, height: f32, radius: f32, ink_remaining: f32, unk4: f32, unk5: f32);

#[skyline::from_offset(0x62b830)]
pub unsafe extern "C" fn add_final_smash_meter(meter_add: f32, fighter: *mut Fighter);

#[skyline::hook(offset = 0x15db0b0)]
pub unsafe extern "C" fn create_item(item_manager: *mut smash::app::ItemManager, create_item_param: *mut CreateItemParam, unk: bool, unk2: bool, unk3: bool) -> *mut BattleObject {
    if (*create_item_param).variation_kind > 7 {
        (*create_item_param).variation_kind = 0;
    }
    original!()(item_manager, create_item_param, unk, unk2, unk3)
}

//Calls the Special Zoom function
#[skyline::from_offset(0x696720)]
pub unsafe extern "C" fn call_special_zoom(boma: *mut BattleObjectModuleAccessor, collision_log: u64, fighter_kind: i32, vl_params: u64, param_5: i32, param_6: i32, param_7: i32, param_8: i32, param_9: i32) -> u64;

#[skyline::from_offset(0x33bd9c0)]
pub unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) -> u64;