use super::*;

const GANON_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0;

//Ganondorf On Search
#[skyline::hook(offset = GANON_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn ganon_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLogScuffed;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
            let opponent_id = (*collision_log).opponent_object_id;
            let opponent_battle_object = get_battle_object_from_id(opponent_id);
            let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
            let opponent_boma = (*opponent_battle_object).module_accessor;
            let opponent_kind = utility::get_kind(&mut *opponent_boma);
            if opponent_battle_object_id >> 0x1C == 1 {
                if opponent_kind == *WEAPON_KIND_KROOL_IRONBALL {
                    let owner_id = WorkModule::get_int(opponent_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                    if owner_id == fighter.battle_object.battle_object_id {
                        WorkModule::set_int(opponent_boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, false);
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hook!(ganon_on_search);
}