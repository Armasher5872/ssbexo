use super::*;

//Villager & Isabelle Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MURABITO, 4, false, false))]
unsafe extern "C" fn murabito_shizue_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MURABITO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::set_int(boma, 0, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_NUM);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C8);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C9);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY_PREV);
    }
    original!()(vtable, fighter)
}

//Villager Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_MURABITO, 7, false, false))]
unsafe extern "C" fn murabito_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        murabito_shizue_reset_initialization,
        murabito_death_initialization
    );
}