use {
    crate::functions::{
        FIGHTER_SPECIAL_STATE,
        FULL_SMASH_ATTACK,
        SITUATION_KIND,
        SPECIAL_ZOOM_GFX,
        USE_DROPKICK
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::{
            Hash40,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn armstrong_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        //Effect Clearing 
        if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_dead_light"), false, true);
        };
        //Damage Update
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_DROPKICK[entry_id] = true;
        }
        //Instadrop
        if WorkModule::is_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) == true {
            macros::EFFECT(fighter, Hash40::new("sys_mach_stomp"), Hash40::new("top"), 0, 0, 0.0, 0.0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 75, 100, 0, 50, 9.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
        if [*FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_LANDING].contains(&status_kind) {
            AttackModule::clear_all(module_accessor);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_mach_stomp"), false, false);
        }
        //Crouch Cancel
        if status_kind == *FIGHTER_STATUS_KIND_SQUAT_WAIT {
            DamageModule::set_reaction_mul(module_accessor, 0.77);
        }
        if status_kind != *FIGHTER_STATUS_KIND_SQUAT_WAIT {
            DamageModule::set_reaction_mul(module_accessor, 1.0);
        }
        //Smash Attacks
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status_kind)
        && frame > 59.0 {
            FULL_SMASH_ATTACK[entry_id] = true;
        }
        if [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            if FULL_SMASH_ATTACK[entry_id] == true {
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
        //Neutral Special
        if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
            if motion_kind == hash40("special_air_n") {
                FIGHTER_SPECIAL_STATE[entry_id] = true;
            }
            else {
                if FIGHTER_SPECIAL_STATE[entry_id] {
                    MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_lw_end"), 1.0, 1.0, 0.0, false, false);
                    AttackModule::clear_all(module_accessor);
                }
            }
        }
        else {
            FIGHTER_SPECIAL_STATE[entry_id] = false;
        }
        //Side Special
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            fighter.sub_transition_group_check_air_cliff();
            USE_DROPKICK[entry_id] = false;
            if StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                if StatusModule::is_situation_changed(fighter.module_accessor) {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                    else {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                        AttackModule::clear_all(module_accessor);
                    };
                }
                if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                    else {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                };
            }
        }
        //Down Special
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            else {
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            }
        }
        if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END {
            AttackModule::clear_all(module_accessor);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        armstrong_frame
    );
}