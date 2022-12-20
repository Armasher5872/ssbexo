#![allow(unused_macros)]
use {
    crate::functions::{
        FULL_SMASH_ATTACK,
        OFFENSE_UP_ACTIVE,
        OFFENSE_UP_GFX_COUNTER,
        OFFENSE_UP_TIMER,
        PK_FLASH_TIMER,
        SHIELD_SPECIAL,
        SITUATION_KIND
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::*,
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_NESS )]
fn ness_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let sticky = ControlModule::get_stick_y(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
        let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
        //SMASH!
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
        && frame > 59.0 {
            FULL_SMASH_ATTACK[entry_id] = true;
            if FULL_SMASH_ATTACK[entry_id] == true {
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_catch"), false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_eye"), false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_head"), false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_talk"), false);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_patterna"), true);
                ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
            }
        }
        if FULL_SMASH_ATTACK[entry_id] == true
        && status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
        && frame > 12.0 {
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_catch"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_eye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_head"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_talk"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_facen"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_patterna"), true);
            ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        if ![*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
            FULL_SMASH_ATTACK[entry_id] = false;
        }
        if (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ATTACK_S4)
        && status_kind == *FIGHTER_STATUS_KIND_WAIT {
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_eye"), true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_facen"), true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_head"), true);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_patterna"), false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_black_face"), false, false);
            ColorBlendModule::cancel_main_color(module_accessor, 0);
        }
        //Jump Cancel Down Throw
        if status_kind == *FIGHTER_STATUS_KIND_THROW
        && motion_kind == hash40("throw_lw")
        && (39.0..=45.0).contains(&frame)
        && ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        };
        //Shield Special
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) 
        && SHIELD_SPECIAL[entry_id] == true {
            if OFFENSE_UP_ACTIVE[entry_id] == true {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 107.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_shield") {
            SHIELD_SPECIAL[entry_id] = false;
            if frame > 120.0
            && OFFENSE_UP_ACTIVE[entry_id] != true {
                OFFENSE_UP_ACTIVE[entry_id] = true;
                OFFENSE_UP_TIMER[entry_id] = 336;
            }
            if frame > 157.0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            };
        }
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            //Graphics
            OFFENSE_UP_GFX_COUNTER[entry_id] += 1;
            if OFFENSE_UP_GFX_COUNTER[entry_id] >= 8 {
                ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
                macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
                OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
            }
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_attack"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_blink"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_catch"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_cliff"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_down"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_downeye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_escape"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_eye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_facen"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_final"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_furafura"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_furafuraeye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_halfblink1"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_halfblink2"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_head"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_heavyattack"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_heavyattackeye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_heavyouch2eye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_hot"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_ouch"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_oucheye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_steppose"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_stepposeeye"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_talk"), false);
            ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_patterna"), true);
            //Duration
            if OFFENSE_UP_TIMER[entry_id] > 0 {
                OFFENSE_UP_TIMER[entry_id] -= 1;
            }
            if OFFENSE_UP_TIMER[entry_id] <= 0 {
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_eye"), true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_facen"), true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_head"), true);
                ModelModule::set_mesh_visibility(module_accessor, Hash40::new("ness_patterna"), false);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_black_face"), false, false);
                OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
                OFFENSE_UP_ACTIVE[entry_id] = false;
            }
            //Mechanics
            AttackModule::set_power_up(module_accessor, 1.15);
            DamageModule::set_damage_mul(module_accessor, 1.15);
            DamageModule::set_reaction_mul(module_accessor, 1.15);
        }
        if OFFENSE_UP_ACTIVE[entry_id] == false {
            AttackModule::set_power_up(module_accessor, 1.0);
            DamageModule::set_damage_mul(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
            ColorBlendModule::cancel_main_color(fighter.module_accessor, 0);
        }
        //Neutral Special
        if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD {
            if PK_FLASH_TIMER[entry_id] >= 0
            && PK_FLASH_TIMER[entry_id] < 120 {
                PK_FLASH_TIMER[entry_id] += 1;
            }
            if PK_FLASH_TIMER[entry_id] > 120 {
                PK_FLASH_TIMER[entry_id] -= 1;
            }
        }
        if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) {
            PK_FLASH_TIMER[entry_id] = 0;
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_bomb_max"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_pkfl_hold"), false, false);
        }
        //Fast Fall PK Fire
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
        && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
        && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
        && sticky < -0.66
        && KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
            WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        };
        //Up Special Land Cancel
        if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN].contains(&status_kind) {
            fighter.sub_transition_group_check_air_cliff();
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            if StatusModule::is_situation_changed(module_accessor) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(ness_frame);
}