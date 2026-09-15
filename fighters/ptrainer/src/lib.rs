use {
    exo_utils::{
        common::var_reset::*,
        structs::getter_funcs::*,
    },
    smash::{
        app::Fighter,
        lib::lua_const::*,
    }
};

//Pokemon Trainer Respawn Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_PZENIGAME, 95, false, false))]
unsafe extern "C" fn ptrainer_respawn_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
}

pub fn install() {
    skyline::install_hook!(ptrainer_respawn_initialization);
}