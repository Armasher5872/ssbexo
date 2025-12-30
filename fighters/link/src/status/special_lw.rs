use super::*;

//Special Lw
unsafe extern "C" fn link_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bomb_object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
    let active_item_from_id = smash::app::lua_bind::ItemManager::find_active_item_from_id(singletons::ItemManager(), bomb_object_id as u32);
    if active_item_from_id != 0 {
        let object_id = fighter.global_table[OBJECT_ID].get_u64();
        let lua_bind_item = active_item_from_id as *mut smash::app::Item;
        let item_owner_id = smash::app::lua_bind::Item::owner_id(lua_bind_item);
        if object_id == item_owner_id {
            let item_boma = sv_battle_object::module_accessor(bomb_object_id as u32);
            let item_status_kind = StatusModule::status_kind(item_boma);
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
                let boomerang_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG);
                let fuse_item_id = WorkModule::get_int(boomerang_boma, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                if fuse_item_id != *BATTLE_OBJECT_ID_INVALID as u32 && sv_battle_object::is_active(fuse_item_id) && fuse_item_id == bomb_object_id as u32 {
                    if item_status_kind != *ITEM_STATUS_KIND_STANDBY {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST);
                        return 1.into();
                    }
                }
                else {
                    if !smash::app::lua_bind::Item::is_had(lua_bind_item, false) {
                        if item_status_kind != *ITEM_STATUS_KIND_STANDBY {
                            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST);
                            return 1.into();
                        }
                    }
                }
            }
            else {
                if !smash::app::lua_bind::Item::is_had(lua_bind_item, false) {
                    if item_status_kind != *ITEM_STATUS_KIND_STANDBY {
                        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST);
                        return 1.into();
                    }
                }
            }
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, link_special_lw_pre_status)
    .install()
    ;
}