use super::*;

const LINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const LINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const LINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

unsafe extern "C" fn link_var(boma: &mut BattleObjectModuleAccessor) {
    let team_no = TeamModule::team_no(boma) as i32;
    WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
    WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
    WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
    WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_int(boma, team_no, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
    WorkModule::set_int(boma, 0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
}

//Link Startup Initialization
#[skyline::hook(offset = LINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_initialization_variable_reset(&mut *boma);
        link_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

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
        common_death_variable_reset(&mut *boma);
        link_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Link Once Per Fighter Frame
#[skyline::hook(offset = LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn link_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        if !ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOOMERANG) {
            if WorkModule::is_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            }
            else {
                let boomerang_fuse_item_id = WorkModule::get_int(boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID) as u32;
                if sv_battle_object::is_active(boomerang_fuse_item_id) {
                    let item_boma = smash::app::sv_battle_object::module_accessor(boomerang_fuse_item_id);
                    if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
                        if smash::app::utility::get_kind(&mut *item_boma) != *ITEM_KIND_LINKBOMB {
                            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
                            smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, boomerang_fuse_item_id);
                        }
                        else {
                            StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_THROW, false);
                            WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
                            WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
                        }
                    }
                }
            }
            if ItemModule::is_have_item(boma, 0) {
                WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
            }
        }
        if WorkModule::is_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
            let item_id = WorkModule::get_int(boma, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            if !sv_battle_object::is_active(item_id) {
                ModelModule::set_mesh_visibility(boma, Hash40::new("link_ken"), true);
                WorkModule::off_flag(boma, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        link_start_initialization,
        link_reset_initialization,
        link_death_initialization,
        link_opff
    );
}