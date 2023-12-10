use super::*;

unsafe extern "C" fn pichu_frame(fighter: &mut L2CFighterCommon) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let status_kind = StatusModule::status_kind(module_accessor);
    let frame = MotionModule::frame(module_accessor);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Attack Addition Check
    if frame < 2.0
    && [hash40("attack_s3_s"), hash40("attack_s4_s"), hash40("attack_lw4"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_lw"), hash40("special_n"), hash40("special_air_n"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
    && DISCHARGE_ACTIVE[entry_id] != true { // resets at the start of a move the inability to add further charge
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    };
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD)
    && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) 
    && [hash40("attack_s3_s"), hash40("attack_s4_s"), hash40("attack_lw4"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_lw"), hash40("special_n"), hash40("special_air_n"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
    && DISCHARGE_ACTIVE[entry_id] != true {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        ELECTRIC_HIT[entry_id] += 1;
    };
    if ELECTRIC_HIT[entry_id] > 5 {
        ELECTRIC_HIT[entry_id] = 5;
    }
    //Self Damage/Healing
    if motion_kind == hash40("attack_s3_s")
    && frame == 6.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 1.05, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 2.1, 0);
        }
    }
    if motion_kind == hash40("attack_s4_s")
    && frame == 17.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 2.1, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 4.2, 0);
        }
    }
    if motion_kind == hash40("attack_hi4")
    && frame == 10.0 {
        if DISCHARGE_ACTIVE[entry_id] == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                    DamageModule::add_damage(fighter.module_accessor, 1.75, 0);
                }
                else {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 3.5, 0);
            }
        }
        if DISCHARGE_ACTIVE[entry_id] == false {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
                else {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
    }
    if motion_kind == hash40("attack_lw4")
    && frame == 9.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 1.3, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 2.6, 0);
        }
    }
    if motion_kind == hash40("attack_air_n")
    && frame == 4.0 {
        if DISCHARGE_ACTIVE[entry_id] == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                    DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
                }
                else {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 2.0, 0);
            }
        }
        if DISCHARGE_ACTIVE[entry_id] == false {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
                else {
                    DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
    }
    if motion_kind == hash40("attack_air_f")
    && frame == 11.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 1.6, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 3.2, 0);
        }
    }
    if motion_kind == hash40("attack_air_b")
    && frame == 6.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 1.6, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 3.2, 0);
        }
    }
    if motion_kind == hash40("attack_air_lw")
    && frame == 15.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 1.6, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 3.2, 0);
        }
    }
    if motion_kind == hash40("landing_air_lw")
    && frame <= 1.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 0.45, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 0.9, 0);
        }
    }
    if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind)
    && frame == 19.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 0.7, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 1.4, 0);
        }
    }
    if [hash40("special_lw_hit"), hash40("special_air_lw_hit")].contains(&motion_kind)
    && frame <= 1.0 {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) == true  {
                DamageModule::add_damage(fighter.module_accessor, 3.0625, 0);
            }
            else {
                DamageModule::add_damage(fighter.module_accessor, 0.0, 0);
            }
        }
        else {
            DamageModule::add_damage(fighter.module_accessor, 6.125, 0);
        }
    }
    //Shield Special
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
    }
    if motion_kind == hash40("special_shield") {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        if ELECTRIC_HIT[entry_id] >= 5 {
            DISCHARGE_ACTIVE[entry_id] = true;
            ELECTRIC_HIT[entry_id] = 0;
        }
    }
    //Discharge Effect
    DISCHARGE_DAMAGE_TIMER[entry_id] -= 1;
    if DISCHARGE_ACTIVE[entry_id] == true {
        DISCHARGE_GFX[entry_id] += 1;
        if DISCHARGE_GFX[entry_id] == 10 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        }
        if DISCHARGE_GFX[entry_id] == 20 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
            macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
            DISCHARGE_GFX[entry_id] = 0;
        }
        if DISCHARGE_DAMAGE_TIMER[entry_id] <= 0
        && DamageModule::damage(module_accessor, 0) < 100.0 {
            DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
            DISCHARGE_DAMAGE_TIMER[entry_id] = 60;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
            }
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            };
            USE_TACKLE[entry_id] = false;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_kind) {
        DISCHARGE_ACTIVE[entry_id] = false;
    }
    if [*FIGHTER_STATUS_KIND_CLIFF_WAIT, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE].contains(&status_kind)
    && ((StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_S) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END)) {
        DISCHARGE_ACTIVE[entry_id] = false;
    }
    if DISCHARGE_ACTIVE[entry_id] == false {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_s_tackle"), -1.0, 1.0, 0.0, false, false);
            }
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_s_tackle"), -1.0, 1.0, 0.0, false, false);
            };
            USE_TACKLE[entry_id] = false;
        }
    }
    //Neutral Special
    let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
    if ![hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
        *tjolt_check = false;
    }
    else {
        if frame == 0.0 {
            *tjolt_check = false;
        }
        if CancelModule::is_enable_cancel(module_accessor) {
            *tjolt_check = false;
        }
    }
    //Side Special
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
    || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
        USE_TACKLE[entry_id] = true;
    }
    //Tackle
    if motion_kind == hash40("special_s_tackle") {
        fighter.sub_transition_group_check_air_cliff();
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        if frame > 40.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            } 
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
    if motion_kind == hash40("special_air_s_tackle") {
        fighter.sub_transition_group_check_air_cliff();
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if frame > 40.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 
            || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 
            || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 
            || (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
            }
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            } 
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }
        }
        if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        }
    }
    //Wild Charge
    if motion_kind == hash40("special_air_s_start") {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, true);
        MotionModule::change_motion_inherit_frame(module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
    }
    if motion_kind == hash40("special_air_s") {
        KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.sub_transition_group_check_air_cliff();
        if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            DISCHARGE_ACTIVE[entry_id] = false;
        }
    }
}

unsafe extern "C" fn pichu_dengeki_functions(fighter: &mut L2CFighterBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
    let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
    if weapon_kind == WEAPON_KIND_PICHU_DENGEKI {
        *tjolt_check = true;
    }
}

unsafe extern "C" fn pichu_dengekidama_functions(fighter: &mut L2CFighterBase) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_kind = smash::app::utility::get_kind(module_accessor) as i32;
    let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
    if weapon_kind == WEAPON_KIND_PICHU_DENGEKIDAMA {
        *tjolt_check = true;
    }
}

pub fn install() {
    Agent::new("pichu")
    .on_line(Main, pichu_frame)
    .install()
    ;
    Agent::new("pichu_dengeki")
    .on_line(Main, pichu_dengeki_functions)
    .install()
    ;
    Agent::new("pichu_dengekidama")
    .on_line(Main, pichu_dengekidama_functions)
    .install()
    ;
}