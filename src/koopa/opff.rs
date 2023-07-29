//Credit to Ultimate S
use super::*;

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn koopa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let end_frame = MotionModule::end_frame(boma);
        let lr = PostureModule::lr(boma);
        let special_zoom_gfx = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) {
            KOOPA_OK_SMASH[entry_id] = false;
            KOOPA_OK_SMASH_GFX[entry_id] = 0;
            KOOPA_GOOD_SMASH[entry_id] = false;
            KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
            KOOPA_GREAT_SMASH[entry_id] = false;
            KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
            KOOPA_EXCELLENT_SMASH[entry_id] = false;
            KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
            if (0.0..=19.0).contains(&frame) {
                KOOPA_OK_SMASH[entry_id] = true;
            }
            else if (19.0..=37.0).contains(&frame) {
                KOOPA_GOOD_SMASH[entry_id] = true;
                KOOPA_OK_SMASH[entry_id] = false;
            }
            else if (37.0..=54.0).contains(&frame) {
                KOOPA_GREAT_SMASH[entry_id] = true;
                KOOPA_GOOD_SMASH[entry_id] = false;
            }
            else if (54.0..=58.0).contains(&frame) {
                KOOPA_EXCELLENT_SMASH[entry_id] = true;
                KOOPA_GREAT_SMASH[entry_id] = false;
            }
            else {
                KOOPA_OK_SMASH[entry_id] = true;
                KOOPA_GOOD_SMASH[entry_id] = false;
                KOOPA_GREAT_SMASH[entry_id] = false;
                KOOPA_EXCELLENT_SMASH[entry_id] = false;
            }
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            if KOOPA_EXCELLENT_SMASH[entry_id] == true {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
                    if special_zoom_gfx < 2 {
                        SlowModule::set_whole(boma, 8, 80);
                        macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                        EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                        macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                    }
                    if special_zoom_gfx >= 4 {
                        SlowModule::clear_whole(boma);
                        CameraModule::reset_all(boma);
                        EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                        macros::CAM_ZOOM_OUT(fighter);
                    }
                }
            }
        }
        if KOOPA_OK_SMASH[entry_id] == true {
            KOOPA_OK_SMASH_GFX[entry_id] += 1;
            if KOOPA_OK_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.2, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.2, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                KOOPA_OK_SMASH_GFX[entry_id] = 0;
            }
        }
        else if KOOPA_GOOD_SMASH[entry_id] == true {
            KOOPA_GOOD_SMASH_GFX[entry_id] += 1;
            KOOPA_OK_SMASH_GFX[entry_id] = 0;
            if KOOPA_GOOD_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.4, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.4, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
            }
        }
        else if KOOPA_GREAT_SMASH[entry_id] == true {
            KOOPA_GREAT_SMASH_GFX[entry_id] += 1;
            KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
            if KOOPA_GREAT_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
            }
        }
        else if KOOPA_EXCELLENT_SMASH[entry_id] == true {
            KOOPA_EXCELLENT_SMASH_GFX[entry_id] += 1;
            KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
            if KOOPA_EXCELLENT_SMASH_GFX[entry_id] == 3 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
            }
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
            KOOPA_OK_SMASH[entry_id] = false;
            KOOPA_OK_SMASH_GFX[entry_id] = 0;
            KOOPA_GOOD_SMASH[entry_id] = false;
            KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
            KOOPA_GREAT_SMASH[entry_id] = false;
            KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
            KOOPA_EXCELLENT_SMASH[entry_id] = false;
            KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
        };
        if motion_kind == hash40("attack_air_lw") {
            if lr <= 0.0 {
                PostureModule::set_lr(boma, 1.0);
                PostureModule::update_rot_y_lr(boma);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
        && [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || FIREBALL_TIMER[entry_id] > 0 {
                CAN_FIREBALL[entry_id] = false;
            }
            else {
                CAN_FIREBALL[entry_id] = true;
            }
        }
        if motion_kind == hash40("special_n") {
            if CAN_FIREBALL[entry_id] == true {
                if end_frame - frame < 5.0 {
                    FIREBALL_TIMER[entry_id] = 180;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                };
                if frame >= 19.0 {
                    FIREBALL_TIMER[entry_id] = 180;
                    CancelModule::enable_cancel(boma);
                };
                MotionModule::set_rate(boma, 0.775);
            }
            else {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("special_n_end"), 1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
		if motion_kind == hash40("special_air_n") {
            if CAN_FIREBALL[entry_id] == true {
                if end_frame-frame < 5.0 {
                    FIREBALL_TIMER[entry_id] = 180;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                };
                if frame >= 19.0 {
                    FIREBALL_TIMER[entry_id] = 180;
                    CancelModule::enable_cancel(boma);
                };
                MotionModule::set_rate(boma, 0.775);
            }
            else {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("special_air_n_end"), 1.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if motion_kind == hash40("special_n_end") {
            if CAN_FIREBALL[entry_id] == true {
                FIREBALL_TIMER[entry_id] = 180;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            else {
                if end_frame - frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                };
            }
        }
		if motion_kind == hash40("special_air_n_end") {
            if CAN_FIREBALL[entry_id] == true {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
            else {
                if end_frame-frame < 5.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                };
            }
        }
        if FIREBALL_TIMER[entry_id] > 0 {
            FIREBALL_TIMER[entry_id] -= 1;
        }
        if ArticleModule::is_exist(boma, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH) {
            if CAN_FIREBALL[entry_id] == true {
                FIREBALL_GFX[entry_id] += 1;
            }
            else {
                FIREBALL_GFX[entry_id] = 0;
            };
        }
        if CAN_FIREBALL[entry_id] == true {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_breath_m_fire"), false, true);
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A].contains(&status_kind) && (StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI | *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G) {
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            fighter.set_back_cliff_hangdata(20.0, 10.0);
            fighter.set_front_cliff_hangdata(20.0, 10.0);
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_KOOPA_BREATH )]
pub fn fireball_gfx_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let entry_id = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let color = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        if CAN_FIREBALL[entry_id] == true {
            AttackModule::set_power_up(weapon.module_accessor, 1.0);
            if FIREBALL_GFX[entry_id] % 14 == 0 {
                if color == 7 {
                    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                    let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, f1, 1.0, 1.0, 0.333);
                    EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
                }
                else if color == 3 {
                    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                    let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, f1, 0.502, 0.494, 0.929);
                    EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
                }
                else {
                    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("koopa_breath_m_fire"), false, true);
                    let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rgb(boma, f1, 1.5, 0.5, 0.5);
                    EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("koopa_breath_m_fire"), smash::phx::Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
                }
            };
        }
        if CAN_FIREBALL[entry_id] == false {
            AttackModule::set_power_up(weapon.module_accessor, 0.2);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        fireball_gfx_frame,
        koopa_frame
    );
}