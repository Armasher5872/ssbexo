use super::*;

unsafe extern "C" fn pichu_dengeki_functions(fighter: &mut L2CFighterBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
    let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
    if weapon_kind == WEAPON_KIND_PICHU_DENGEKI {
        *tjolt_check = true;
    }
}

unsafe extern "C" fn pichu_dengekidama_functions(fighter: &mut L2CFighterBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
    let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
    if weapon_kind == WEAPON_KIND_PICHU_DENGEKIDAMA {
        *tjolt_check = true;
    }
}

pub fn install() {
    Agent::new("pichu_dengeki")
    .on_line(Main, pichu_dengeki_functions)
    .install()
    ;
    Agent::new("pichu_dengekidama")
    .on_line(Main, pichu_dengekidama_functions)
    .install()
    ;
}