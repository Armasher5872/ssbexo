#![allow(improper_ctypes_definitions)] //Addresses Building warning: `extern` fn uses type `Vector4`, which is not FFI-safe
use super::*;

//Obtains the battle_object from an id
#[skyline::from_offset(0x3ac560)]
pub unsafe extern "C" fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[skyline::from_offset(0x58250)]
pub unsafe fn pane_append_child(pane: *mut Pane, child: *const Pane);

#[skyline::from_offset(0x58290)]
pub unsafe fn pane_remove_child(pane: *mut Pane, child: *const Pane);

#[skyline::from_offset(0x583c0)]
pub unsafe fn find_pane_by_name(pane: *const Pane, s: *const c_char, recursive: bool) -> *mut Pane;

#[skyline::from_offset(0x59970)]
pub unsafe fn find_pane_by_name_recursive(pane: *const Pane, s: *const c_char) -> *mut Pane;

#[skyline::from_offset(0x646fe0)]
pub unsafe extern "C" fn singleton_fighter_hook(fighter: &mut smash::app::Fighter) -> u64;

//Deals damage to characters who are in water
#[skyline::from_offset(0x6924e0)]
pub unsafe extern "C" fn add_water_damage(boma: *mut BattleObjectModuleAccessor, work_id_const: i32);

//This function references BattleObjectWorld, which is defo for the ledge positions
#[skyline::from_offset(0x6941e0)]
pub unsafe extern "C" fn handle_cliff(boma: &mut BattleObjectModuleAccessor, vec: &Vector4f) -> smash2::cpp::simd::Vector4;

//Calls the Special Zoom function
#[skyline::from_offset(0x696720)]
pub unsafe extern "C" fn call_special_zoom(boma: *mut BattleObjectModuleAccessor, collision_log: u64, fighter_kind: i32, vl_params: u64, param_5: i32, param_6: i32, param_7: i32, param_8: i32, param_9: i32) -> u64;

//A function related to stat changes
#[skyline::from_offset(0x75d8f0)]
pub unsafe extern "C" fn set_lightweight_data(group_ptr: *mut StatChangeGroup, stat_change_vec: *mut StatChange, end_ptr: *mut StatChange);

//Handles the visual tank for Inkling
#[skyline::from_offset(0xb0bbd0)]
pub unsafe extern "C" fn inkling_handle_tank_fill(boma: *mut BattleObjectModuleAccessor, height: f32, radius: f32, ink_remaining: f32, unk4: f32, unk5: f32);

//Disables stage visibility
#[skyline::from_offset(0x159fb20)]
pub unsafe extern "C" fn set_stage_visibility(module_accessor: *mut BattleObjectModuleAccessor, param_2: u32);

//The common on hit function for weapons
#[skyline::from_offset(0x33bd9c0)]
pub unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64;

#[skyline::from_offset(0x37a22f0)]
pub unsafe fn pane_set_text_string(pane: *mut TextBox, s: *const c_char);

//The memory allocator for Ultimate
#[skyline::from_offset(0x392dce0)]
pub unsafe extern "C" fn allocator(align: i32, size: i32) -> *mut u64;

//Another function related to stat changes
#[skyline::from_offset(0x392e590)]
pub unsafe extern "C" fn set_lightweight_data_post(stat_change_vec: *mut StatChange);