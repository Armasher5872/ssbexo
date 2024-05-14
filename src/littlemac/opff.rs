use super::*;

unsafe extern "C" fn littlemac_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let strength = WorkModule::get_int(boma, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    //Resets the meter to 0 if the values are invalid
    if ko_gauge < 0.0 || ko_gauge == f32::NAN {
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    //Sets the meter to specific values if they're not the exact bounds
    if (1.0..=33.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if (35.0..=67.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if (69.0..=99.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 68.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if ko_gauge > 100.0 {
        WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    //Different Cancel Frames for Star Punch
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2 {
        if frame < 1.0 {
            match ko_gauge {
                _ if ko_gauge == 0.0 => {
                    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ if ko_gauge == 34.0 => {
                    WorkModule::set_int(boma, 1, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ if ko_gauge == 68.0 => {
                    WorkModule::set_int(boma, 2, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ => {
                    WorkModule::set_int(boma, 3, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
        }
        match strength {
            _ if strength == 0 => {
                if frame > 40.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ if strength == 1 => {
                if frame > 55.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ if strength == 2 => {
                if frame > 65.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ => {
                if CancelModule::is_enable_cancel(boma) {
                    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
        }
    }
    //Allows Little Mac to do Side Special multiple times if he's hit
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S)
    && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
}

unsafe extern "C" fn littlemac_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
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
    //Little Mac
    WorkModule::set_flag(boma, false, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
}

pub fn install() {
    Agent::new("littlemac")
    .on_start(littlemac_init)
    .on_line(Main, littlemac_frame)
    .install()
    ;
}