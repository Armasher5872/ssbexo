use super::*;

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } 
        else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

pub fn get_active_battle_object_id_from_entry_id(entry_id: u32) -> Option<u32> {
    let object = get_battle_object_from_entry_id(entry_id)?;
    if object.is_null() {return None;}
    let object = unsafe {&mut *object};
    let kind = object.kind as i32;
    let status = unsafe {StatusModule::status_kind(object.module_accessor)};
    if ![*FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_STANDBY].contains(&status) {
        return Some(object.battle_object_id);
    }
    if [*FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_EFLAME].contains(&kind) {
        Some(object.battle_object_id+0x10000)
    } 
    else if [*FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU,  *FIGHTER_KIND_PLIZARDON].contains(&kind) {
        let next_id = object.battle_object_id+0x10000;
        let next_object = unsafe {get_battle_object_from_id(next_id)};
        if !next_object.is_null() {
            let next_object = unsafe {&mut *next_object};
            let next_status = unsafe {StatusModule::status_kind(next_object.module_accessor)};
            if ![*FIGHTER_STATUS_KIND_NONE, *FIGHTER_STATUS_KIND_STANDBY].contains(&next_status) {
                Some(next_id)
            } 
            else {
                Some(next_id+0x10000)
            }
        }
        else {
            Some(object.battle_object_id)
        }
    } 
    else {
        Some(object.battle_object_id)
    }
}

//Obtains all active battle_object_ids, including both Ice Climbers and only the active character of Pokemon Trainer and Aegis
pub unsafe fn get_all_active_battle_object_ids() -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for entry_id in 0..8 {
        let id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
        vec.push(id);
        let object = get_battle_object_from_id(id);
        if object.is_null() {continue;}
        let object = unsafe {&mut *object};
        let kind = object.kind as i32;
        if kind != *FIGHTER_KIND_POPO {continue;}
        let boma = &mut *(*object).module_accessor;
        let nana_id = WorkModule::get_int(boma, *FIGHTER_POPO_INSTANCE_WORK_ID_INT_PARTNER_OBJECT_ID) as u32;
        let nana_object = get_battle_object_from_id(nana_id);
        if nana_object.is_null() {continue;}
        let nana_object = unsafe {&mut *nana_object};
        vec.push(nana_object.battle_object_id);
    }
    return vec;
}