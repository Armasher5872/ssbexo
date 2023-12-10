use super::*;

unsafe extern "C" fn ness_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let sticky = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    //Jump Cancel Down Throw
    if status_kind == *FIGHTER_STATUS_KIND_THROW
    && motion_kind == hash40("throw_lw")
    && (39.0..=45.0).contains(&frame)
    && fighter.jump_cancel()
    && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
    };
    //Shield Special
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END].contains(&status_kind) 
    && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
        if OFFENSE_UP_ACTIVE[entry_id] == true {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 107.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    if motion_kind == hash40("special_shield") {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        if frame > 120.0
        && OFFENSE_UP_ACTIVE[entry_id] != true {
            OFFENSE_UP_ACTIVE[entry_id] = true;
            OFFENSE_UP_TIMER[entry_id] = 336;
        }
        if frame > 157.0 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        };
    }
    if OFFENSE_UP_ACTIVE[entry_id] == true {
        //Graphics
        OFFENSE_UP_GFX_COUNTER[entry_id] += 1;
        if OFFENSE_UP_GFX_COUNTER[entry_id] >= 8 {
            ColorBlendModule::set_main_color(boma, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("ness_black_face"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("haver"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            macros::EFFECT(fighter, Hash40::new("ness_psi_atk"), Hash40::new("havel"), 0, 0, 0.0, 0.0, -90, 0, 0.75, 0, 0, 0, 0, 0, 360, true);
            OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
        }
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_attack"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_blink"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_catch"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_cliff"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_down"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_downeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_escape"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_final"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_furafura"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_furafuraeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_halfblink1"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_halfblink2"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyattack"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyattackeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_heavyouch2eye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_hot"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_ouch"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_oucheye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_steppose"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_stepposeeye"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_talk"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), true);
        //Duration
        if OFFENSE_UP_TIMER[entry_id] > 0 {
            OFFENSE_UP_TIMER[entry_id] -= 1;
        }
        if OFFENSE_UP_TIMER[entry_id] <= 0 {
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_eye"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_facen"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_head"), true);
            ModelModule::set_mesh_visibility(boma, Hash40::new("ness_patterna"), false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("ness_black_face"), false, false);
            OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
            OFFENSE_UP_ACTIVE[entry_id] = false;
        }
        //Mechanics
        AttackModule::set_power_up(boma, 1.15);
        DamageModule::set_damage_mul(boma, 1.15);
        DamageModule::set_reaction_mul(boma, 1.15);
    }
    if OFFENSE_UP_ACTIVE[entry_id] == false {
        AttackModule::set_power_up(boma, 1.0);
        DamageModule::set_damage_mul(boma, 1.0);
        DamageModule::set_reaction_mul(boma, 1.0);
        ColorBlendModule::cancel_main_color(boma, 0);
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
    && (ControlModule::get_command_flag_cat(boma, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
    && sticky < -0.66
    && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
        WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
    };
    //Up Special Land Cancel
    if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_AGAIN].contains(&status_kind) {
        fighter.sub_transition_group_check_air_cliff();
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        if StatusModule::is_situation_changed(boma) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}

pub fn install() {
    Agent::new("ness")
    .on_line(Main, ness_frame)
    .install()
    ;
}