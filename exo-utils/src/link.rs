#![allow(improper_ctypes_definitions)] //Addresses warning: `extern` fn uses type `Vector2`, which is not FFI-safe
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

pub unsafe extern "C" fn set_arrow_fuse_params(boma: *mut BattleObjectModuleAccessor, item_kind: i32, fuse_kind: i32) {
    if ![*ITEM_KIND_NONE, *ITEM_KIND_ASSIST, *ITEM_KIND_LINKARROW].contains(&item_kind) || [*ITEM_KIND_BANANAGUN, *ITEM_KIND_FIREFLOWER].contains(&item_kind) {
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

pub unsafe extern "C" fn link_decide_arrow(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
        let bow_arrow_agent = get_weapon_common_from_accessor(&mut *bow_arrow_boma);
        let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if arrow_type != *WN_LINK_BOWARROW_ICE_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_ice_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(bow_arrow_agent, 0.0, 0.0, 1.0);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_ICE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_light_arrow_charge"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_LIGHT_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if situation_kind != *SITUATION_KIND_GROUND {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 0.19, false, 0.0, false, false);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 0.19, false, 0.0, false, false);
                        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if arrow_type != *WN_LINK_BOWARROW_FIRE_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_fire_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.6, true);
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(bow_arrow_agent, 1.0, 0.0, 0.0);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_FIRE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                if arrow_type != *WN_LINK_BOWARROW_SHOCK_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(bow_arrow_agent, 1.0, 0.0, 1.0);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_SHOCK_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
        }
    }
}

pub unsafe extern "C" fn link_guard_cancel(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        if situation_kind == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
        else {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
        }
    }
}

pub unsafe extern "C" fn link_change_motion(fighter: &mut L2CFighterCommon, situation_kind: i32, ground_motion_kind: &str, ground_bow_motion_kind: &str, air_motion_kind: &str, air_bow_motion_kind: &str, keep: bool) {
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new(ground_bow_motion_kind), true, -1.0);
        if keep {
            if hash40(ground_motion_kind) == hash40("special_n_start") {
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(ground_motion_kind), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new(air_bow_motion_kind), true, -1.0);
        if keep {
            if hash40(air_motion_kind) == hash40("special_air_n_start") {
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new(air_motion_kind), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

pub unsafe extern "C" fn link_shoot_arrow(fighter: &mut L2CFighterCommon) {
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_FIRST), true);
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

#[derive(Default, Copy, Clone)]
pub struct LinkStamina {
    pub number: u64,
    pub value: i32,
    pub enabled: bool,
}

impl LinkStamina {
    pub fn new(layout_data: u64) -> Self {
        let number = get_pane_from_layout(layout_data, "link_wheel\0").expect("Couldn't find link_wheel");
        return Self {
            number: number,
            value: 0,
            enabled: false
        };
    }
    pub fn reset(&mut self) {
        set_pane_visible(self.number, true);
        self.value = 0;
    }
    pub fn set_meter_info(&mut self, value: i32) {
        self.value = value;
    }
    pub fn update_icon(&mut self) {
        let offset = match self.value {
            0 => 0.0,
            1 => 1.0,
            2 => 2.0,
            3 => 3.0,
            4 => 4.0,
            5 => 5.0,
            6 => 6.0,
            7 => 7.0,
            8 => 8.0,
            9 => 9.0,
            10 => 10.0,
            11 => 11.0,
            12 => 12.0,
            13 => 13.0,
            14 => 14.0,
            15 => 15.0,
            16 => 16.0,
            _ => -1.0
        };
        if offset < 0.0 {
            set_pane_visible(self.number, false);
            return;
        }
        let offset = offset/17.0;
        let len = 1.0/17.0;
        set_pane_visible(self.number, true);
        set_tex_coords(self.number, [offset, 0.0, offset+len, 0.0, offset, 1.0, offset+len, 1.0]);
    }
}