use super::*;

const RIDLEY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1043a20; //Ridley only
const RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1043d20; //Ridley only
const RIDLEY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x1044440; //Ridley only

//Ridley Startup Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RIDLEY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Ridley Reset Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ridley Death Initialization
#[skyline::hook(offset = RIDLEY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ridley_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
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
    let agent = get_fighter_common_from_accessor(&mut *boma);
    if RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] > 0 {
        if situation_kind == *SITUATION_KIND_GROUND {
            RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] = 0;
        }
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
        SET_SPEED_EX(agent, velocity_x*0.5, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
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
        SET_SPEED_EX(agent, velocity_x*0.5, velocity_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        POGO_GROUND_BOUNCE[entry_id] = false;
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        ridley_start_initialization,
        ridley_reset_initialization,
        ridley_death_initialization,
        ridley_opff
    );
}