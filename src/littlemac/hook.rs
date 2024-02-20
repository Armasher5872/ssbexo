use super::*;

//Collision Log
pub struct CollisionLog {
    next: *mut CollisionLog,
    end: *mut CollisionLog,
    location: Vector3f,
    padding_0: u32,
    padding_1: u32,
    opponent_battle_object_id: u32,
    padding_2: [u8;7],
    collision_kind: u8,
    receiver_part_id: u8,
    collider_part_id: u8,
    receiver_id: u8,
    collider_id: u8,
    padding_3: [u8;10]
}

//Updates Battle UI
#[skyline::from_offset(0x068cd80)]
pub unsafe fn update_battle_ui(fighter_data: *const u64, param_2: u32);

//Little Mac KO Meter Update
#[skyline::hook(offset = 0xc45680)]
pub unsafe extern "C" fn ko_meter_update(vtable: u64, battle_object: *mut BattleObject, collision_log: CollisionLog, damage: f32) -> u64 {
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
    && meter < 100.0 {
        EffectModule::req_on_joint(boma, Hash40::new("sys_hit_normal_l"), Hash40::new("handr"), &NONE_VECTOR, &NONE_VECTOR, 0.8, &NONE_VECTOR, &NONE_VECTOR, false, 0, 0, 0);
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

pub fn install() {
	skyline::install_hooks!(
        ko_meter_update
    );
}