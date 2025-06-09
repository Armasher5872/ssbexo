use super::*;

pub unsafe extern "C" fn ac_common(boma: *mut BattleObjectModuleAccessor) {
    let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let obj_id = WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if obj_id != *BATTLE_OBJECT_ID_INVALID as u32 || obj_id != 0 {
            let obj_boma = sv_battle_object::module_accessor(obj_id);
            let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
            let weapon = get_weapon_common_from_accessor(&mut *obj_boma);
            let owner_boma = get_owner_boma(weapon);
            let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
            if is_barrel(obj_boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
                barrel_removal(weapon);
            }
            if is_slash(obj_boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
                slash_removal(weapon);
            }
            if is_galaxia(obj_boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
                galaxia_beam_removal(weapon);
            }
            if is_sludge(obj_boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
                sludge_removal(weapon);
            }
            if is_disarming_voice(obj_boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);
                disarming_voice_removal(weapon);
            }
            if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
                let item_id = WorkModule::get_int64(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                let fused_item = if owner_kind == *FIGHTER_KIND_KIRBY {
                    WorkModule::get_int(owner_boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
                }
                else if [*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO].contains(&owner_kind) {
                    WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM)
                }
                else if WorkModule::is_flag(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_FUSE_DEDEDE_SWALLOW) {
                    smash::app::utility::get_kind(&mut *item_boma)
                }
                else {
                    WorkModule::get_int(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE)
                };
                WorkModule::set_int(boma, fused_item, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
                if sv_battle_object::is_active(item_id) {
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
                }
            }
            /*
            else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
                let item_id = WorkModule::get_int(obj_boma, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(item_id);
                let fused_item = if StatusModule::status_kind(obj_boma) == *WN_LINK_BOOMERANG_STATUS_KIND_SWALLOWED {
                    smash::app::utility::get_kind(&mut *item_boma)
                }
                else if[*FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_MURABITO].contains(&owner_kind) {
                    WorkModule::get_int(owner_boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM)
                }
                else {
                    WorkModule::get_int(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE)
                };
                WorkModule::set_int(boma, fused_item, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
                if sv_battle_object::is_active(item_id) {
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
                }
            }
            if !ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
                let boomerang_fuse_item_id = WorkModule::get_int(boma, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID) as u32;
                let item_boma = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
                if smash::app::sv_battle_object::is_active(boomerang_fuse_item_id) && StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
                    smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
                }
            }
            */
        }
    }
}