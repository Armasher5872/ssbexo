#![allow(improper_ctypes_definitions)]
use super::*;

const LITTLEMAC_UI_UPDATE_INTERNAL_OFFSET: usize = 0x68cda0; //Little Mac only
const LITTLEMAC_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc44790; //Little Mac only
const LITTLEMAC_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc44830; //Little Mac only
const LITTLEMAC_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc448c0; //Little Mac only
const LITTLEMAC_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0xc44b80; //Little Mac only
const LITTLEMAC_VTABLE_ON_ATTACK_OFFSET: usize = 0xc456a0; //Little Mac only
const LITTLEMAC_VTABLE_ON_DAMAGE_OFFSET: usize = 0xc45d70; //Little Mac only

#[skyline::from_offset(LITTLEMAC_UI_UPDATE_INTERNAL_OFFSET)]
fn update_littlemac_ui_internal(manager_offset: *mut u32, total_gauge: i32);

//Updates Battle UI, credit to HDR
unsafe extern "C" fn update_littlemac_ui(entry_id: i32, total_gauge: f32) {
    let manager = singletons::FighterManager() as *mut u64;
    let offset = (*manager + (entry_id as u64 * 8) + 0x20) as *mut u64;
    update_littlemac_ui_internal((*offset + 0x41e4) as *mut u32, total_gauge as i32);
}

unsafe extern "C" fn littlemac_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
}

//Little Mac Startup Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Little Mac Reset Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE_MAX_VALUE);
}

//Little Mac Death Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    original!()(vtable, fighter)
}

//Little Mac Once Per Fighter Frame
#[skyline::hook(offset = LITTLEMAC_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn littlemac_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let strength = WorkModule::get_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    //Resets the meter to 0 if the values are invalid
    if ko_gauge < 0.0 || ko_gauge.is_nan() {
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
    update_littlemac_ui(entry_id, ko_gauge);
    //Different Cancel Frames for Star Punch
    if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2 {
        if frame < 1.0 {
            match ko_gauge {
                _ if ko_gauge == 0.0 => {
                    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ if ko_gauge == 34.0 => {
                    WorkModule::set_int(boma, 1, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ if ko_gauge == 68.0 => {
                    WorkModule::set_int(boma, 2, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
                _ => {
                    WorkModule::set_int(boma, 3, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
        }
        match strength {
            _ if strength == 0 => {
                if frame > 40.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ if strength == 1 => {
                if frame > 55.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ if strength == 2 => {
                if frame > 65.0 {
                    CancelModule::enable_cancel(boma);
                    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
            _ => {
                if CancelModule::is_enable_cancel(boma) {
                    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
                }
            }
        }
    }
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_littlemac_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    original!()(vtable, fighter)
}

//Little Mac On Attack
#[skyline::hook(offset = LITTLEMAC_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn littlemac_on_attack(vtable: u64, battle_object: *mut BattleObject, collision_log: CollisionLog, _damage: f32) -> u64 {
    let boma = (&mut *(battle_object)).boma();
    let opponent_boma = &mut *(sv_battle_object::module_accessor(collision_log.opponent_battle_object_id));
    let mut meter_gain = 0.0;
    let meter = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let status_checks = [
		*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
		*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, 
		*FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, 
		*FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR, *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START, *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, 
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING, 
		*FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, 
		*FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER, 
		*FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3, *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3
	];
	let smashes = [*FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4];
    //Removes critical zoom if meter isn't full
    if boma.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2)
    && meter != 100.0 {
        EffectModule::req_on_joint(boma, Hash40::new("sys_hit_normal_l"), Hash40::new("handr"), &Vector3f::zero(), &Vector3f::zero(), 0.8, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        return 1;
    }
    //Adds a third of the meter if Little Mac lands a counterhit
    for i in 0..TOTAL_FIGHTER {
        if COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] && get_attacker_number(&mut *get_boma(i)) == get_player_number(boma) && opponent_boma.is_status_one_of(&status_checks) {
            if boma.is_status_one_of(&smashes) {
                COUNTERHIT_SUCCESS[get_player_number(boma)] = true;
            }
            COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] = false;
            meter_gain = 34.0;
        }
    }
    //Adds a third of the meter if Little Mac lands Down Special
    if boma.is_status(*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT) {
        meter_gain = 34.0;
    }
    call_original!(vtable, battle_object, collision_log, meter_gain)
}

//Little Mac On Damage
#[skyline::hook(offset = LITTLEMAC_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn littlemac_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = fighter.battle_object.status();
    if [*FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&status_kind) {
        if !WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION) {
            WorkModule::sub_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
            WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
        }
    }
    //Allows Little Mac to do Side Special multiple times if he's hit
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
    call_original!(vtable, fighter, on_damage)
}

pub fn install() {
	skyline::install_hooks!(
        littlemac_start_initialization,
        littlemac_reset_initialization,
        littlemac_death_initialization,
        littlemac_opff,
        littlemac_on_attack,
        littlemac_on_damage
    );
}