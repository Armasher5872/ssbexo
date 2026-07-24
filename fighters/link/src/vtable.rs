use super::*;

const LINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const LINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

//Link Reset Initialization
#[skyline::hook(offset = LINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        link_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Link Death Initialization
#[skyline::hook(offset = LINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_death_variable_reset(&mut *boma);
        link_var(&mut *boma);
        UiManager::set_link_wheel_info(entry_id, 0);
    }
    original!()(vtable, fighter)
}

//Link Once Per Fighter Frame
#[skyline::hook(offset = LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn link_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let status_kind = agent.global_table[STATUS_KIND].get_i32();
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        let stamina = WorkModule::get_int(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
        let wheel_value = if stamina >= 283 {0} else if stamina >= 264 {1} else if stamina >= 246 {2} else if stamina >= 228 {3} else if stamina >= 210 {4} else if stamina >= 192 {5} else if stamina >= 174 {6} else if stamina >= 156 {7} 
        else if stamina >= 138 {8} else if stamina >= 120 {9} else if stamina >= 102 {10} else if stamina >= 84 {11} else if stamina >= 66 {12} else if stamina >= 48 {13} else if stamina >= 30 {14} else if stamina >= 1 {15} else {16};
        if [
            *FIGHTER_STATUS_KIND_ATTACH_WALL, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAUNCH, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE
        ].contains(&status_kind) {
            UiManager::set_link_wheel_enable(entry_id, true);
            UiManager::set_link_wheel_info(entry_id, wheel_value);
        }
        else {
            UiManager::set_link_wheel_enable(entry_id, false);
            UiManager::set_link_wheel_info(entry_id, 0);
        }
    }
    original!()(vtable, fighter)
}

unsafe extern "C" fn link_swordbeam_on_attack_event(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_EDGE {
        *(weapon as *mut bool).add(0x90) = true;
        StatusModule::change_status_request(boma, *WEAPON_EDGE_ZANSHIN_SHOT_STATUS_KIND_HIT, false);
    }
    normal_weapon_hit_handler(vtable, weapon, collision_bitmask)
}

unsafe extern "C" fn link_boomerang_on_search_event(_vtable: u64, weapon: &mut smash::app::Weapon, log: *mut CollisionLogScuffed) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let opponent_object_id = (*log).opponent_object_id;
    let opponent_category = (*log).opponent_object_category;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_boma = sv_battle_object::module_accessor(opponent_object_id);
        if opponent_category == 4 {
            WorkModule::set_int(boma, opponent_object_id as i32, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            LinkModule::remove_model_constraint(opponent_boma, true);
            if LinkModule::is_link(opponent_boma, *ITEM_LINK_NO_HAVE) {
                LinkModule::unlink(opponent_boma, *ITEM_LINK_NO_HAVE);
            }
            if !LinkModule::is_link(opponent_boma, *ITEM_LINK_NO_HAVE) {
                VisibilityModule::set_whole(opponent_boma, true);
                LinkModule::link(opponent_boma, *ITEM_LINK_NO_HAVE, (*weapon).battle_object.battle_object_id);
                LinkModule::set_model_constraint_pos_ort(opponent_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("top"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32, true);
            }
        }
        if opponent_category == 0 {
            if opponent_object_id == owner_id {
                if owner_kind == *FIGHTER_KIND_LINK {
                    let fuse_item_id = WorkModule::get_int(boma, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
                    let item_boma = sv_battle_object::module_accessor(fuse_item_id);
                    if fuse_item_id != *BATTLE_OBJECT_ID_INVALID as u32 && sv_battle_object::is_active(fuse_item_id) {
                        LinkModule::remove_model_constraint(item_boma, true);
                        StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_FALL, false);
                    }
                }
            }
        }
    }
}

pub fn install() {
    weapon_initialise_module(*WEAPON_KIND_LINK_BOOMERANG, ModuleInitModules::SearchModule);
    let _ = skyline::patching::Patch::in_text(0x51dcca8).data(link_swordbeam_on_attack_event as *const () as u64); //029
    let _ = skyline::patching::Patch::in_text(0x51dbb10).data(link_boomerang_on_search_event as *const () as u64); //035
	skyline::install_hooks!(
        link_reset_initialization,
        link_death_initialization,
        link_opff
    );
}