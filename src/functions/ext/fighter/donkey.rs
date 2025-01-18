use super::*;

//Used to deal with DK's Barrels
pub unsafe extern "C" fn donkey_barrel_bool(boma: *mut BattleObjectModuleAccessor) -> bool {
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let barrel_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, (*boma).battle_object_id, smash2::app::ItemKind::Barrel);
    let timer = WorkModule::get_int(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    if barrel_count == 0 && !WorkModule::is_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) && timer <= 0 {
        return true;
    }
    return false;
}