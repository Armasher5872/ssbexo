use super::*;

const RIDLEY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1043a20; //Ridley only
const RIDLEY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1044440; //Ridley only

//Ridley Startup Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_RIDLEY as u32 {
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

//Ridley Reset Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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
    original!()(vtable, fighter)
}

//Ridley Once Per Fighter Frame
#[skyline::hook(offset = RIDLEY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ridley_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let pos_x_global = PostureModule::pos_x(boma);
    let pos_y_global = PostureModule::pos_y(boma);
    let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(boma, Hash40::new("tail8"), v3f_tail_pos, false);
    let lr = PostureModule::lr(boma);
    let pos_x_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].x;
    let pos_y_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].y;
    RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id] = Vector2f{x: v3f_tail_pos.x-pos_x_global, y: v3f_tail_pos.y-pos_y_global}; //save current tail pos relative to fighter
    let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    if RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] > 0 {
        if situation_kind == *SITUATION_KIND_GROUND {
            RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] = 0;
        }
    }
    if [hash40("special_lw_stab"), hash40("special_air_lw_stab")].contains(&motion_kind) {
        if motion_kind == hash40("special_air_lw_stab") {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        }
        else {
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE) {
               StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
               AttackModule::clear_all(boma);
            }
        }
    }
    else {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    }
    if motion_kind == hash40("special_air_lw_stab") {
        WorkModule::set_int64(boma, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL) && !POGO_OPPONENT_BOUNCE[entry_id] {
            POGO_OPPONENT_BOUNCE[entry_id] = true;
        }
        else if GroundModule::ray_check_hit_pos(boma, &Vector2f{x:pos_x_prev+pos_x_global, y: pos_y_prev+pos_y_global}, &Vector2f{x: (v3f_tail_pos.x -(pos_x_prev+pos_x_global))+(8.0*lr), y: v3f_tail_pos.y -(pos_y_prev+pos_y_global) -8.0}, ground_hit_pos, true) && !POGO_GROUND_BOUNCE[entry_id] && (21.0..35.0).contains(&frame) {
            POGO_GROUND_BOUNCE[entry_id] = true;
        }
    }
    if POGO_OPPONENT_BOUNCE[entry_id] {
        let velocity_x = lr*KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //macros::SET_SPEED_EX(fighter, velocity_x*0.5, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        POGO_OPPONENT_BOUNCE[entry_id] = false;
    }
    if POGO_GROUND_BOUNCE[entry_id] {
        let mut slope_angle = 0.0;
        let slope_check_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(boma, &Vector2f{x:ground_hit_pos.x+(5.0*lr), y:ground_hit_pos.y+5.0}, &Vector2f{x:0.0, y:-10.0}, slope_check_pos, true) {
            let pos_diff_y = ground_hit_pos.y-slope_check_pos.y;
            if pos_diff_y > 0.0 {
                slope_angle = (pos_diff_y / 5.0).atan().to_degrees();
            }
            else {
                slope_angle = 360.0 -((-pos_diff_y / 5.0).atan().to_degrees());
            }
        }
        EffectModule::req_follow(boma, Hash40::new("sys_crown"), Hash40::new("top"), &Vector3f{x: (ground_hit_pos.x-pos_x_global)*lr, y: ground_hit_pos.y-pos_y_global, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: slope_angle, z: 0.0} as *const Vector3f, 0.2, false, 0, 0, 0, 0, 0, false, false);
        EffectModule::req_follow(boma, Hash40::new("sys_quake"), Hash40::new("top"), &Vector3f{x: (ground_hit_pos.x-pos_x_global)*lr, y: ground_hit_pos.y-pos_y_global, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: slope_angle, z: 0.0} as *const Vector3f, 0.5, false, 0, 0, 0, 0, 0, false, false);
        SoundModule::play_se(boma, Hash40::new("se_ridley_special_h03"), true, false, false, false, enSEType(0));
        CameraModule::req_quake(boma, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_impact"), 0, false, 0);
        let bounce_vel_max = 2.5;
        let bounce_vel_min = 0.0;
        let bounce_vel_mul = 0.04;
        let mut velocity_y = (-bounce_vel_mul*(pos_y_global -ground_hit_pos.y))+bounce_vel_max;
        if pos_y_global -ground_hit_pos.y <= 0.0 {
            velocity_y = bounce_vel_max;
        }
        else if velocity_y < bounce_vel_min {
            velocity_y = bounce_vel_min;
        }
        let velocity_x = PostureModule::lr(boma) * KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //macros::SET_SPEED_EX(fighter, velocity_x*0.5, velocity_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        POGO_GROUND_BOUNCE[entry_id] = false;
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ridley_start_initialization,
        ridley_reset_initialization,
        ridley_opff
    );
}