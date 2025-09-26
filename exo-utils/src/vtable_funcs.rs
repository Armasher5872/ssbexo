use super::*;

//Adds a new shield type, used for making new counters
pub unsafe fn add_shield_group(boma: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource, group_id: i32) {
    let ptr = get_module_vtable_func(boma, 0x100, 0x58);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource, i32) = std::mem::transmute(ptr);
    let shield_module = *(boma as *mut *mut u64).add(0x100/8);
    set_shield_group2(shield_module, resource, group_id);
    let count = (*resource).count as i32;
    if count > 0 {
        for x in 0..count {
            ShieldModule::set_status(boma, x, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), group_id);
        }
    }
}

//Adds a new reflector type, used for making new reflectors
pub unsafe fn add_reflector_group(boma: *mut BattleObjectModuleAccessor, resource: *mut ShieldGroupResource2, group_id: i32) {
    let ptr = get_module_vtable_func(boma, 0x108, 0x60);
    let set_shield_group2: extern "C" fn(*mut u64, *mut ShieldGroupResource2, i32) = std::mem::transmute(ptr);
    let reflector_module = *(boma as *mut *mut u64).add(0x108/8);
    set_shield_group2(reflector_module, resource, group_id);
    let count = (*resource).count as i32;
    if count > 0 {
        for x in 0..count {
            ReflectorModule::set_status(boma, x, smash::app::ShieldStatus(*SHIELD_STATUS_NONE), group_id);
        }
    }
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}