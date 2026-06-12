#![allow(improper_ctypes_definitions)] //Addresses warning: `extern` fn uses type `Vector2`, which is not FFI-safe
use super::*;

pub unsafe extern "C" fn link_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_JUMP);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_GAIN);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    WorkModule::set_int(boma, 300, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_1);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_2);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_3);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_ID_4);
}

pub unsafe extern "C" fn link_decide_arrow(fighter: &mut L2CFighterCommon) {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
        let bow_arrow_boma = get_article_boma(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
        let bow_arrow_agent = get_weapon_common_from_accessor(&mut *bow_arrow_boma);
        let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if arrow_type != *WN_LINK_BOWARROW_ICE_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_ice_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(bow_arrow_agent, 0.0, 0.0, 1.0);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_ICE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_light_arrow_charge"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_LIGHT_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if situation_kind != *SITUATION_KIND_GROUND {
                        MotionModule::change_motion(boma, Hash40::new("special_air_n_start"), 0.0, 0.19, false, 0.0, false, false);
                        ArticleModule::change_motion(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_air_start"), true, -1.0);
                        ArticleModule::set_rate(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                    else {
                        MotionModule::change_motion(boma, Hash40::new("special_n_start"), 0.0, 0.19, false, 0.0, false, false);
                        ArticleModule::change_motion(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new("n_start"), true, -1.0);
                        ArticleModule::set_rate(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                if arrow_type != *WN_LINK_BOWARROW_FIRE_ARROW {
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_fire_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.6, true);
                    EFFECT_FOLLOW(bow_arrow_agent, Hash40::new("link_final_arrow_hold"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 1.0, true);
                    LAST_EFFECT_SET_COLOR(bow_arrow_agent, 1.0, 0.0, 0.0);
                    WorkModule::set_int(bow_arrow_boma, *WN_LINK_BOWARROW_FIRE_ARROW, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                }
            }
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
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
    let boma = fighter.module_accessor;
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        if situation_kind == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
        else {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, false);
        }
    }
}

pub unsafe extern "C" fn link_change_motion(fighter: &mut L2CFighterCommon, situation_kind: i32, ground_motion_kind: &str, ground_bow_motion_kind: &str, air_motion_kind: &str, air_bow_motion_kind: &str, keep: bool) {
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        ArticleModule::change_motion(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new(ground_bow_motion_kind), true, -1.0);
        if keep {
            if hash40(ground_motion_kind) == hash40("special_n_start") {
                if ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(boma, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(boma, Hash40::new(ground_motion_kind), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        ArticleModule::change_motion(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, Hash40::new(air_bow_motion_kind), true, -1.0);
        if keep {
            if hash40(air_motion_kind) == hash40("special_air_n_start") {
                if ArticleModule::is_exist(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW) {
                    let bow_arrow_boma = get_article_boma(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW);
                    let arrow_type = WorkModule::get_int(bow_arrow_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
                    if arrow_type != *WN_LINK_BOWARROW_LIGHT_ARROW {
                        MotionModule::change_motion_inherit_frame(boma, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion_inherit_frame_keep_rate(boma, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0);
                        ArticleModule::set_rate(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, 0.19);
                    }
                }
            }
            else {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        else {
            MotionModule::change_motion(boma, Hash40::new(air_motion_kind), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

pub unsafe extern "C" fn link_shoot_arrow(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    ArticleModule::set_visibility_whole(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::set_visibility_whole(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::shoot(boma, *FIGHTER_LINK_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_FIRST), true);
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