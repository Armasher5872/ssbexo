use super::*;

const POPO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xfb6750; //Shared
const POPO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xfb6a50; //Shared
const POPO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfb6c40; //Shared
const POPO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xfb5a10; //Shared

//Popo Startup Initialization
#[skyline::hook(offset = POPO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn popo_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_POPO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_initialization_variable_reset(&mut *boma);
        UiManager::set_iceclimber_meter_info(entry_id, 0, 0, 0);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Popo Reset Initialization
#[skyline::hook(offset = POPO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn popo_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_POPO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_reset_variable_reset(&mut *boma);
        UiManager::set_iceclimber_meter_info(entry_id, 0, 0, 0);
    }
    original!()(vtable, fighter)
}

//Popo Death Initialization
#[skyline::hook(offset = POPO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn popo_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_POPO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_death_variable_reset(&mut *boma);
        UiManager::set_iceclimber_meter_info(entry_id, 0, 0, 0);
    }
    original!()(vtable, fighter, param_3)
}

//Popo Once Per Fighter Frame
#[skyline::hook(offset = POPO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn popo_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_POPO as u32 {
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
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        popo_start_initialization,
        popo_reset_initialization,
        popo_death_initialization,
        popo_opff
    );
}