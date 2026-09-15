use super::*;

/*FIGHTER*/

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

/*WEAPON*/

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

pub fn get_weapon_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CWeaponCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CWeaponCommon))
    }
}

/*OTHER*/

pub fn offset_to_addr<T>(offset: usize) -> *const T {
    unsafe {
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
    }
}

//Credit to HDR. Only pulls the game state to perform actions on
pub fn get_game_state() -> *const u64 {
    unsafe {
        let p_p_p_game_state = *offset_to_addr::<*const *const *const u64>(0x52c2760);
        if p_p_p_game_state.is_null() {
            return std::ptr::null();
        }
        let p_p_game_state = *p_p_p_game_state;
        if p_p_game_state.is_null() {
            return std::ptr::null();
        }
        let p_game_state = *p_p_game_state;
        if p_game_state.is_null() {
            return std::ptr::null();
        }
        p_game_state
    }
}

//Used to get the pointer for a vtable function within a specific module.
pub unsafe fn get_module_vtable_func(boma: *mut BattleObjectModuleAccessor, module_offset: usize, func_offset: u64) -> u64 {
    let module = (boma as *mut u64).add(module_offset/0x8);
    let vtable = *module as *const u64;
    *((*vtable + func_offset) as *const u64)
}

/*
Credited to IncrediblePlays for the original logic of the function (Of which this is a modified form of). Used to get either the address or pointer address of an agent (fighter/weapon/item) virtual function. 

kind: The respective fighter/weapon kind, takes the dereferenced lua_const value (E.G. *FIGHTER_KIND_SNAKE)
entry: The entry of the virtual function you want to hook. Documentation can be found here: https://github.com/theincredibleplayer/smash-vtables/blob/main/Vtable/fighter_vtable_documentation.txt
is_weapon: Indicates if the agent is a weapon
is_pointer: Determines whether or not it should get the address hook itself, or the pointer to the address hook. The pointer can be seen as the "index" of the agent's virtual table.

An example of its usage would be: get_agent_virtual_function(*FIGHTER_KIND_SNAKE, 13, false, false); This would get the 13th (0-Indexed) virtual function of Snake, which is his OPFF.

This function also has additional checks that will forcibly close the game if the function params are out of bounds
*/
pub fn get_agent_virtual_function(kind: i32, entry: usize, is_weapon: bool, is_pointer: bool) -> usize {
    if kind < 0 {
        std::process::abort()
    }
    unsafe {
        if is_weapon {
            if kind >= 0x267 || entry >= 104 {
                std::process::abort();
            }
        }
        else{
            if kind >= 0x5E || entry >= 146 {
                std::process::abort();
            }
        }
        let vtable = if is_weapon {get_weapon_vtable(kind as u32)} else {get_fighter_vtable(kind as u32)};
        let first_entry_ptr = *(vtable as *const u64) as *const usize;
        let main = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text);
        if is_pointer {
            return first_entry_ptr.add(entry) as usize-main as usize;
        }
        else {
            return *first_entry_ptr.add(entry)-main as usize;
        }
    }
}