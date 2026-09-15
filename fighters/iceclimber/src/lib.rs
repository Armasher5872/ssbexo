use {
    exo_utils::{
        common::var_reset::*,
        structs::{
            getter_funcs::*,
            ui_manager::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    }
};

//Popo & Nana Reset Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_POPO, 4, false, false))]
unsafe extern "C" fn iceclimber_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_reset_variable_reset(&mut *boma);
    UiManager::set_iceclimber_meter_info(entry_id, 0, 0, 0);
    original!()(vtable, fighter)
}

//Popo & Nana Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_POPO, 7, false, false))]
unsafe extern "C" fn iceclimber_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    UiManager::set_iceclimber_meter_info(entry_id, 0, 0, 0);
    original!()(vtable, fighter, param_3)
}

//Popo & Nana Once Per Fighter Frame
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_POPO, 13, false, false))]
unsafe extern "C" fn iceclimber_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let nana_id = WorkModule::get_int(boma, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID);
    if nana_id != *BATTLE_OBJECT_ID_INVALID {
        let nana_boma = sv_battle_object::module_accessor(nana_id as u32);
        let nana_status_kind = StatusModule::status_kind(nana_boma);
        let nana_damage = DamageModule::damage(nana_boma, 0) as i32;
        let nana_one_damage = nana_damage%10;
        let nana_ten_damage = (nana_damage%100)/10;
        let nana_hundred_damage = (nana_damage%1000)/100;
        if ![*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_DEMO, *FIGHTER_STATUS_KIND_STANDBY].contains(&nana_status_kind) {
            UiManager::set_iceclimber_meter_info(entry_id, nana_one_damage, nana_ten_damage, nana_hundred_damage);
            UiManager::set_iceclimber_meter_color(entry_id, DamageModule::damage(nana_boma, 0));
            UiManager::set_iceclimber_meter_enable_1(entry_id, true);
            if nana_ten_damage != 0 || (nana_ten_damage == 0 && nana_hundred_damage != 0) {
                UiManager::set_iceclimber_meter_enable_2(entry_id, true);
            }
            else {
                UiManager::set_iceclimber_meter_enable_2(entry_id, false);
            }
            if nana_hundred_damage == 0 {
                UiManager::set_iceclimber_meter_enable_3(entry_id, false);
            }
            else {
                UiManager::set_iceclimber_meter_enable_3(entry_id, true);
            }
        }
        else {
            UiManager::set_iceclimber_meter_enable_1(entry_id, false);
            UiManager::set_iceclimber_meter_enable_2(entry_id, false);
            UiManager::set_iceclimber_meter_enable_3(entry_id, false);
        }
    }
    else {
        UiManager::set_iceclimber_meter_enable_1(entry_id, false);
        UiManager::set_iceclimber_meter_enable_2(entry_id, false);
        UiManager::set_iceclimber_meter_enable_3(entry_id, false);
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        iceclimber_reset_initialization,
        iceclimber_death_initialization,
        iceclimber_opff
    );
}

