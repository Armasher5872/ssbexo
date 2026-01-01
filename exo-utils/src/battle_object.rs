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