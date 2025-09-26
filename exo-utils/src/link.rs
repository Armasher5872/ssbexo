#![allow(improper_ctypes_definitions)]
use super::*;

pub struct FuseKind();

impl FuseKind {
    pub const FUSE: i32 = 0;
    pub const REFUSE: i32 = 1;
}

pub struct FuseType();

impl FuseType {
    pub const NORMAL: i32 = 0;
    pub const POWER: i32 = 1;
    pub const ELEMENTAL: i32 = 2;
}

pub unsafe extern "C" fn set_arrow_fuse_params(boma: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE, *ITEM_KIND_ASSIST, *ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE, *ITEM_TRAIT_FLAG_SHOOT, *ITEM_TRAIT_FLAG_SWING].contains(&trait_type)) 
    || [*ITEM_KIND_BANANAGUN, *ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        WorkModule::set_int(boma, item_kind, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_kind == FuseKind::FUSE {
            if owner_kind == *FIGHTER_KIND_LINK {
                WorkModule::set_int(owner_boma, item_kind, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            else if owner_kind == *FIGHTER_KIND_KIRBY {
                WorkModule::set_int(owner_boma, item_kind, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
            }
            let item_id = ItemModule::get_have_item_id(owner_boma, 0) as i32;
            WorkModule::set_int(boma, item_id, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let pos_z = PostureModule::pos_z(boma);
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_pos: Vector4f{x: pos_x, y: pos_y, z: pos_z, w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(boma),
                owner_id: (*(boma)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager, &mut params, false, false, false);
            let item_boma = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL, *ITEM_KIND_CHEWING, *ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_boma, *ITEM_STATUS_KIND_HAVE, false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma, 1.35, false);
            }
            let item_id = (*(item_boma)).battle_object_id as i32;
            WorkModule::set_int(boma, item_id, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            if [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_SHIZUE].contains(&owner_kind) {
                WorkModule::set_int(owner_boma, *ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(boma, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_BOMBER_STATUS_KIND_BORN2, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if [*ITEM_KIND_KILLER, *ITEM_KIND_BANANAGUN, *ITEM_KIND_DOLPHINBOMB].contains(&item_kind) {
            WorkModule::set_int(boma, FuseType::POWER, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_THROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(boma, FuseType::ELEMENTAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_LOST, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(boma, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_BORN, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(boma, FuseType::NORMAL, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG);
            WorkModule::set_int(boma, *ITEM_STATUS_KIND_THROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn set_boomerang_fuse_params(boma: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32, trait_type: i32) {
    if (![*ITEM_KIND_NONE,*ITEM_KIND_ASSIST,*ITEM_KIND_LINKARROW].contains(&item_kind) && ![*ITEM_TRAIT_FLAG_NONE,*ITEM_TRAIT_FLAG_SHOOT,*ITEM_TRAIT_FLAG_SWING].contains(&trait_type))
    || [*ITEM_KIND_FIREFLOWER].contains(&item_kind) {
        WorkModule::on_flag(boma,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    else {
        WorkModule::off_flag(boma,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED);
    }
    if WorkModule::is_flag(boma,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_ITEM_FUSED) {
        let owner_boma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        WorkModule::set_int(boma,item_kind,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
        if fuse_kind == FuseKind::FUSE {
            WorkModule::set_int(owner_boma,item_kind,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
            let item_id = ItemModule::get_have_item_id(owner_boma,0) as i32;
            WorkModule::set_int(boma,item_id,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_boma,item_id,*FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
        }
        else if fuse_kind == FuseKind::REFUSE {
            let mut params = CreateItemParam {
                founder_pos: Vector4f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma), w: 0.0},
                item_pos: Vector4f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma), w: 0.0},
                item_kind: smash::app::ItemKind(item_kind),
                another_battle_object_id: *BATTLE_OBJECT_ID_INVALID as u32,
                variation_kind: *ITEM_VARIATION_NONE,
                lr_dir: PostureModule::lr(boma),
                owner_id: (*(boma)).battle_object_id,
                unk_20: 20,
                pokeball_or_assist_kind: *ITEM_KIND_NONE,
                unk_0: 0,
                weird_flag: 0x633F800000,
                unk_1_weird: 1,
                unk_approx_0: 0.0,
                unk_02: 0.0
            };
            let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
            let battle_object = create_item(item_manager,&mut params,false,false,false);
            let item_boma = (*battle_object).module_accessor;
            if ![*ITEM_KIND_HEALBALL,*ITEM_KIND_CHEWING,*ITEM_KIND_BOOMERANG].contains(&item_kind) {
                StatusModule::change_status_request(item_boma,*ITEM_STATUS_KIND_HAVE,false);
            }
            if item_kind == *ITEM_KIND_LINKBOMB {
                PostureModule::set_scale(item_boma,1.3,false);
            }
            let item_id = (*(item_boma)).battle_object_id as i32;
            WorkModule::set_int(boma,item_id, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
            WorkModule::set_int(owner_boma,item_id, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
            if owner_kind == *FIGHTER_KIND_MURABITO
            || owner_kind == *FIGHTER_KIND_SHIZUE {
                WorkModule::set_int(owner_boma,*ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
            }
        }
        if item_kind == *ITEM_KIND_BOMBER {
            WorkModule::set_int(boma,*ITEM_BOMBER_STATUS_KIND_BORN2, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_FIREFLOWER {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_LOST, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else if item_kind == *ITEM_KIND_LINKBOMB {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_BORN, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
        else {
            WorkModule::set_int(boma,*ITEM_STATUS_KIND_THROW, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_STATUS);
        }
    }
}

pub unsafe extern "C" fn link_change_angle(fighter: &mut L2CFighterCommon, current_degree: f32, max_degree: f32, motion_kind_max: &str, motion_kind_min: &str) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind_2nd = MotionModule::motion_kind_2nd(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    let motion = if current_degree <= 0.0 {hash40(motion_kind_min)} else {hash40(motion_kind_max)};
    if motion_kind_2nd != motion {
        if current_degree <= 0.0 {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new(motion_kind_min), frame, rate, true, -(current_degree/max_degree));
            MotionModule::set_weight(fighter.module_accessor, 1.0+(current_degree/max_degree), true);
        }
        else {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new(motion_kind_max), frame, rate, true, current_degree/max_degree);
            MotionModule::set_weight(fighter.module_accessor, 1.0-(current_degree/max_degree), true);
        }
    }
    else {
        if current_degree < 0.0 {
            MotionModule::set_weight(fighter.module_accessor, 1.0+(current_degree/max_degree), true);
        }
        else if current_degree > 0.0 {
            MotionModule::set_weight(fighter.module_accessor, 1.0-(current_degree/max_degree), true);
        }
        else {
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        }
    }
}

pub unsafe extern "C" fn find_ascendable_ground(boma: *mut BattleObjectModuleAccessor, pos_x: f32, min_pos_y: f32, pos_y: f32, height: f32) -> f32 {
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    if GroundModule::ray_check_hit_pos(boma, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -100.0}, ground_hit_pos, true) {
        if ground_hit_pos.y < min_pos_y {
            return pos_y;
        }
        return find_ascendable_ground(boma, pos_x, min_pos_y, ground_hit_pos.y-height, height);
    }
    else {
        return pos_y;
    }
}