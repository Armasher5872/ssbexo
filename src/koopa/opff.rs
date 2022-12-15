use {
    crate::functions::{
        KOOPA_EXCELLENT_SMASH,
        KOOPA_EXCELLENT_SMASH_GFX,
        SPECIAL_ZOOM_GFX,
        KOOPA_GOOD_SMASH,
        KOOPA_GOOD_SMASH_GFX,
        KOOPA_GREAT_SMASH,
        KOOPA_GREAT_SMASH_GFX,
        KOOPA_OK_SMASH,
        KOOPA_OK_SMASH_GFX
    },
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{
            Hash40,
            Vector3f
        },
        app::{
            lua_bind::*,
            *
        },
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn koopa_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_START].contains(&status_kind) {
            KOOPA_OK_SMASH[entry_id] = false;
            KOOPA_OK_SMASH_GFX[entry_id] = 0;
            KOOPA_GOOD_SMASH[entry_id] = false;
            KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
            KOOPA_GREAT_SMASH[entry_id] = false;
            KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
            KOOPA_EXCELLENT_SMASH[entry_id] = false;
            KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
            SPECIAL_ZOOM_GFX[entry_id] = 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {
            if frame >= 0.0
            && frame < 19.0 {
                KOOPA_OK_SMASH[entry_id] = true;
            }
            else if frame >= 19.0
            && frame < 37.0 {
                KOOPA_GOOD_SMASH[entry_id] = true;
                KOOPA_OK_SMASH[entry_id] = false;
            }
            else if frame >= 37.0
            && frame < 54.0 {
                KOOPA_GREAT_SMASH[entry_id] = true;
                KOOPA_GOOD_SMASH[entry_id] = false;
            }
            else if frame >= 54.0
            && frame < 58.0 {
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
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    SPECIAL_ZOOM_GFX[entry_id] += 1;
                    if SPECIAL_ZOOM_GFX[entry_id] < 2 {
                        SlowModule::set_whole(module_accessor, 8, 80);
                        macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                        EffectModule::req_follow(module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                        macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                    }
                    if SPECIAL_ZOOM_GFX[entry_id] >= 4 {
                        SlowModule::clear_whole(module_accessor);
                        CameraModule::reset_all(module_accessor);
                        EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                        macros::CAM_ZOOM_OUT(fighter);
                    }
                }
            }
        }
        if KOOPA_OK_SMASH[entry_id] == true {
            KOOPA_OK_SMASH_GFX[entry_id] += 1;
            if KOOPA_OK_SMASH_GFX[entry_id] == 1 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.2, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.2, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
            }
            if KOOPA_OK_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
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
            if KOOPA_GOOD_SMASH_GFX[entry_id] == 1 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.4, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.4, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
            }
            if KOOPA_GOOD_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
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
            if KOOPA_GREAT_SMASH_GFX[entry_id] == 1 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.6, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);
            }
            if KOOPA_GREAT_SMASH_GFX[entry_id] == 6 {
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
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
            if KOOPA_EXCELLENT_SMASH_GFX[entry_id] == 1 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 7.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 1.0);
            }
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
                PostureModule::set_lr(module_accessor, 1.0);
                PostureModule::update_rot_y_lr(module_accessor);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        koopa_frame
    );
}