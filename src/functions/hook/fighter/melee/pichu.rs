use super::*;

const PICHU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf2a520; //Shared
const PICHU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared
const PICHU_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0xf2a630; //Shared

//Pichu Startup Initialization
#[skyline::hook(offset = PICHU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
}

//Pichu Reset Initialization
#[skyline::hook(offset = PICHU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
}

//Pichu Death Initialization
#[skyline::hook(offset = PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    original!()(vtable, fighter)
}

//Pichu Once Per Fighter Frame
#[skyline::hook(offset = PICHU_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn pichu_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        if frame < 2.0 && [hash40("attack_s3_s"), hash40("attack_s4_s"), hash40("attack_lw4"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_lw"), hash40("special_n"), hash40("special_air_n"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && !DISCHARGE_ACTIVE[entry_id] {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        };
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)  && [hash40("attack_s3_s"), hash40("attack_s4_s"), hash40("attack_lw4"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_lw"), hash40("special_n"), hash40("special_air_n"), hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) && !DISCHARGE_ACTIVE[entry_id] {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
            ELECTRIC_HIT[entry_id] += 1;
        };
        if ELECTRIC_HIT[entry_id] > 5 {
            ELECTRIC_HIT[entry_id] = 5;
        }
        //Self Damage/Healing
        if motion_kind == hash40("attack_s3_s") && frame == 6.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 1.05, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 2.1, 0);
            }
        }
        if motion_kind == hash40("attack_s4_s") && frame == 17.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 2.1, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 4.2, 0);
            }
        }
        if motion_kind == hash40("attack_hi4") && frame == 10.0 {
            if DISCHARGE_ACTIVE[entry_id] {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                        DamageModule::add_damage(boma, 1.75, 0);
                    }
                    else {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                }
                else {
                    DamageModule::add_damage(boma, 3.5, 0);
                }
            }
            else {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                    else {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
        }
        if motion_kind == hash40("attack_lw4") && frame == 9.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 1.3, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 2.6, 0);
            }
        }
        if motion_kind == hash40("attack_air_n") && frame == 4.0 {
            if DISCHARGE_ACTIVE[entry_id] {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                        DamageModule::add_damage(boma, 1.0, 0);
                    }
                    else {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                }
                else {
                    DamageModule::add_damage(boma, 2.0, 0);
                }
            }
            else {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                    else {
                        DamageModule::add_damage(boma, 0.0, 0);
                    }
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
        }
        if motion_kind == hash40("attack_air_f") && frame == 11.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 1.6, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 3.2, 0);
            }
        }
        if motion_kind == hash40("attack_air_b") && frame == 6.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 1.6, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 3.2, 0);
            }
        }
        if motion_kind == hash40("attack_air_lw") && frame == 15.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 1.6, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 3.2, 0);
            }
        }
        if motion_kind == hash40("landing_air_lw") && frame <= 1.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 0.45, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 0.9, 0);
            }
        }
        if [hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && frame == 19.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 0.7, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 1.4, 0);
            }
        }
        if [hash40("special_lw_hit"), hash40("special_air_lw_hit")].contains(&motion_kind) && frame <= 1.0 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                    DamageModule::add_damage(boma, 3.0625, 0);
                }
                else {
                    DamageModule::add_damage(boma, 0.0, 0);
                }
            }
            else {
                DamageModule::add_damage(boma, 6.125, 0);
            }
        }
        //Shield Special
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL) {
            MotionModule::change_motion(boma, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
        if motion_kind == hash40("special_shield") {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
            if ELECTRIC_HIT[entry_id] >= 5 {
                DISCHARGE_ACTIVE[entry_id] = true;
                ELECTRIC_HIT[entry_id] = 0;
            }
        }
        //Discharge Effect
        DISCHARGE_DAMAGE_TIMER[entry_id] -= 1;
        if DISCHARGE_ACTIVE[entry_id] {
            DISCHARGE_GFX[entry_id] += 1;
            if DISCHARGE_GFX[entry_id] == 10 {
                let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                let head_aura = EffectModule::req_follow(boma, Hash40::new("pichu_cheek"), Hash40::new("head"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: -90.0, z: -90.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::set_rgb(boma, waist_aura as u32, 1.0, 1.0, 0.0);
                EffectModule::set_rgb(boma, bust_aura as u32, 1.0, 1.0, 0.0);
                EffectModule::set_rgb(boma, head_aura as u32, 1.0, 1.0, 0.0);
            }
            if DISCHARGE_GFX[entry_id] == 20 {
                EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, true);
                EffectModule::kill_kind(boma, Hash40::new("pichu_cheek"), false, true);
                let waist_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("waist"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                let bust_aura = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                let head_aura = EffectModule::req_follow(boma, Hash40::new("pichu_cheek"), Hash40::new("head"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: -90.0, z: -90.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                EffectModule::set_rgb(boma, waist_aura as u32, 1.0, 1.0, 0.0);
                EffectModule::set_rgb(boma, bust_aura as u32, 1.0, 1.0, 0.0);
                EffectModule::set_rgb(boma, head_aura as u32, 1.0, 1.0, 0.0);
                DISCHARGE_GFX[entry_id] = 0;
            }
            if DISCHARGE_DAMAGE_TIMER[entry_id] <= 0 && DamageModule::damage(boma, 0) < 100.0 {
                DamageModule::add_damage(boma, 1.0, 0);
                DISCHARGE_DAMAGE_TIMER[entry_id] = 60;
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if situation_kind == *SITUATION_KIND_GROUND {
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
                };
                USE_TACKLE[entry_id] = false;
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, true);
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_kind) {
            DISCHARGE_ACTIVE[entry_id] = false;
        }
        if [*FIGHTER_STATUS_KIND_CLIFF_WAIT, *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CLIMB, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_CLIFF_ATTACK, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE].contains(&status_kind) && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_END].contains(&prev_status_kind) {
            DISCHARGE_ACTIVE[entry_id] = false;
        }
        if !DISCHARGE_ACTIVE[entry_id] {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if situation_kind == *SITUATION_KIND_GROUND {
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s_tackle"), -1.0, 1.0, 0.0, false, false);
                }
                if situation_kind == *SITUATION_KIND_AIR {
                    MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s_tackle"), -1.0, 1.0, 0.0, false, false);
                };
                USE_TACKLE[entry_id] = false;
            }
        }
        //Neutral Special
        let tjolt_check = &mut FIGHTER_BOOL_1[get_player_number(&mut fighter.battle_object.boma())];
        if ![hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) {
            *tjolt_check = false;
        }
        else {
            if frame == 0.0 {
                *tjolt_check = false;
            }
            if CancelModule::is_enable_cancel(boma) {
                *tjolt_check = false;
            }
        }
        //Side Special
        if situation_kind == *SITUATION_KIND_GROUND || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_TACKLE[entry_id] = true;
        }
        //Tackle
        if motion_kind == hash40("special_s_tackle") {
            //fighter.sub_transition_group_check_air_cliff();
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            if frame > 40.0 {
                CancelModule::enable_cancel(boma);
            }
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackS3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::Catch) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, true);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                } 
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
            if MotionModule::end_frame(boma) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        if motion_kind == hash40("special_air_s_tackle") {
            //fighter.sub_transition_group_check_air_cliff();
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            if frame > 40.0 {
                CancelModule::enable_cancel(boma);
            }
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    };
                }
                else if fighter.battle_object.is_cat_flag(Cat1::AttackAirN) || fighter.battle_object.is_cat_flag(Cat1::AttackAirF) || fighter.battle_object.is_cat_flag(Cat1::AttackAirB) || fighter.battle_object.is_cat_flag(Cat1::AttackAirHi) || fighter.battle_object.is_cat_flag(Cat1::AttackAirLw){
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                }
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                } 
                else if fighter.battle_object.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
            if MotionModule::end_frame(boma) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        //Wild Charge
        if motion_kind == hash40("special_air_s_start") {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, true);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
        if motion_kind == hash40("special_air_s") {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            //fighter.sub_transition_group_check_air_cliff();
            if MotionModule::end_frame(boma) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                DISCHARGE_ACTIVE[entry_id] = false;
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        pichu_start_initialization,
        pichu_reset_initialization,
        pichu_death_initialization,
        pichu_opff
    );
}