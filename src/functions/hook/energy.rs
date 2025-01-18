//The following code is credited to HDR
use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EnergyControllerResetType {
    FallAdjust = 0x0,
    FallAdjustNoCap = 0x1,
    StopCeil = 0x2,
    WallJump = 0x3,
    FlyAdjust = 0x4,
    Dash = 0x5,
    ShootDash = 0x6,
    ShootBackDash = 0x7,
    TurnRun = 0x8,
    RevolveSlashAir = 0x9,
    Turn = 0xA,
    Free = 0xB,
    FreeTest = 0xC,
    ItemLift = 0xD,
    SwimRise = 0xE,
    Swim = 0xF,
    SwimDrown = 0x10,
    MoveGround = 0x11,
    MoveAir = 0x12,
    TurnNoStop = 0x13,
    TurnNoStopAir = 0x14,
    Ladder = 0x15,
    DashBack = 0x16,
}

#[skyline::hook(offset = 0x6d3630)]
unsafe extern "C" fn control_update(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
    let fighter = get_fighter_common_from_accessor(&mut *boma);
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let status_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x8);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let get_stick_x = ControlModule::get_stick_x(boma);
    let get_stick_y = ControlModule::get_stick_y(boma);
    let get_sub_stick_x = ControlModule::get_sub_stick_x(boma);
    let get_sub_stick_y = ControlModule::get_sub_stick_y(boma);
    let lr = PostureModule::lr(boma);
    let reset_type = std::mem::transmute(energy.energy_reset_type);
    let mut stick = if Buttons::from_bits_unchecked(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride) {Vector2f {x: get_sub_stick_x, y: get_sub_stick_y}} else {Vector2f {x: get_stick_x, y: get_stick_y}};
    let damage_reaction_frame = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
    let stick_rate = WorkModule::get_float(boma, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLOAT_STICK_RATE);
    let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    let wall_jump_disable_cont_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_WALL_JUMP_WORK_INT_DISABLE_CONT_FRAME);
    let dash_frame = WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
    let keep_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("dash_speed_keep_frame"));
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let run_brake_brake_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("run_brake_brake_mul"));
    let turn_run_brake = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stop_brake_mul"));
    let swim_speed_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("swim_speed_mul"));
    let swim_drown_speed_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("swim_drown_speed_x_mul"))*swim_speed_mul; 
    let turn_speed_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_speed_mul"));
    let ladder_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("ladder_stick_y"));
    let ladder_speed_d_max = WorkModule::get_param_float(boma, hash40("common"), hash40("ladder_speed_d_max"));
    let ladder_attack_speed_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("ladder_attack_speed_mul"));
    let common_air_speed_x_limit = WorkModule::get_param_float(boma, hash40("common"), hash40("common_air_speed_x_limit"));
    let vertical_limit = if energy.speed.y <= 0.0 {WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_down_limit"))} else {WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_up_limit"))};
    let ground_speed_limit = WorkModule::get_param_float(boma, hash40("common"), hash40("ground_speed_limit"));
    let turn_dash_from_dash_count = WorkModule::get_int(boma, *FIGHTER_STATUS_DASH_WORK_INT_TURN_DASH_FROM_DASH_COUNT);
    let backup_max = energy.speed_max;
    let backup_brake = energy.speed_brake;
    if damage_reaction_frame > 0.0 {
        stick.x = 0.0;
    }
    let accel_add_x = if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR 
    && WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) 
    && !WorkModule::is_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL) {
        stick.x = 0.0;
        0.0
    } 
    else if stick.x == 0.0 {
        0.0
    } 
    else {
        energy.accel_add_x
    };
    let accel_add_y = if stick.y != 0.0 {energy.accel_add_y} else {0.0};
    let mut do_standard_accel = true;
    use EnergyControllerResetType::*;
    let accel_diff = match reset_type {
        FallAdjust | FallAdjustNoCap | FlyAdjust | ShootDash | ShootBackDash | RevolveSlashAir | MoveGround | MoveAir => {
            accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x
        },
        WallJump => {
            if wall_jump_disable_cont_frame == 0 {accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x} else {0.0}
        },
        Dash | DashBack => loop {
            // Don't apply or change the speed by any amount during the first keep frames of dash
            if status_kind == *FIGHTER_STATUS_KIND_DASH || reset_type == DashBack {
                if dash_frame < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                    break 0.0;
                }
                else if dash_frame == keep_frame {
                    let dash_speed = if reset_type == DashBack {
                        -energy.lr*dash_speed
                    } 
                    else {
                        energy.lr*dash_speed
                    };
                    energy.speed.x = if energy.speed.x*energy.lr >= 0.0 {
                        if reset_type == DashBack {
                            dash_speed+energy.speed.x
                        } 
                        else {
                            dash_speed
                        }
                    } 
                    else {
                        if reset_type == DashBack {
                            dash_speed
                        } 
                        else {
                            dash_speed+energy.speed.x
                        }
                    };
                }
            } 
            else if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
                if turn_dash_from_dash_count < keep_frame {
                    energy.speed_max.x = 0.0;
                    energy.speed_brake.x = 0.0;
                    stick.x = accel_add_x;
                    break 0.0;
                }
            }
            // accel add
            break stick.x*energy.accel_mul_x+stick.x.signum()*energy.accel_add_x;
        },
        TurnRun => {
            let mut mul = stick.x*energy.accel_mul_x+accel_add_x*stick.x.signum();
            let mut brake = ground_brake*run_brake_brake_mul;
            let ground_module = *(boma as *const BattleObjectModuleAccessor as *const u64).add(0x88/0x8);
            let some_float = *(ground_module as *const f32).add(0x130/0x8);
            if some_float*energy.lr <= -0.1 {
                mul *= turn_run_brake;
                brake *= turn_run_brake;
            }
            energy.speed_brake.x = brake;
            if 0.0 <= mul*energy.lr {
                // If stick is at neutral
                do_standard_accel = false;
                energy.accel.x = 0.0;
                energy.accel.y = 0.0;
                energy.speed_max.x = 0.0;
                0.0
            } 
            else {
                mul
            }
        },
        Free => {
            energy.accel.y = accel_add_y*stick.y.signum()+stick.y*energy.accel_mul_y;
            energy.speed_max.y = stick.y.abs()*energy.speed_max.y;
            accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x
        },
        ItemLift => loop {
            if WorkModule::is_flag(boma, *FIGHTER_STATUS_ITEM_LIFT_WORK_FLAG_STOP) {
                stick.x = 0.0;
                break 0.0;
            }
            if accel_add_x*lr <= 0.0 {
                energy.speed_max.x = 0.0;
                energy.speed_brake.x = 0.0;
                stick.x = accel_add_x; // not sure if this is accurate but it's what I think I saw in the code
                break 0.0;
            }
            energy.speed_max.x *= stick_rate;
            break (accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x)*stick_rate;
        },
        Swim => {
            energy.speed_max.x = stick.x.abs()*swim_speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x
        },
        SwimDrown => {
            energy.speed_max.x = stick.x*swim_drown_speed_mul;
            energy.speed_max.y = -1.0;
            accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x
        },
        Turn | TurnNoStop | TurnNoStopAir => {
            if reset_type == TurnNoStop || reset_type == TurnNoStopAir {
                if (!TurnModule::is_turn(boma) || energy.accel_mul_x == 0.0) && energy.speed.x == 0.0 {
                    energy.parent.enable = false;
                    return;
                }
                if ControlModule::reverse_x_frame(boma) != 0 {
                    stick.x = -stick.x;
                }
            }
            (accel_add_x*stick.x.signum()+stick.x*energy.accel_mul_x)*turn_speed_mul
        },
        Ladder => {
            let (speed_max, accel_y) = if ladder_stick_y <= stick.y.abs() {
                if stick.y <= 0.0 {
                    // lerp the down_max
                    let down_max = ((stick.y.abs()-ladder_stick_y)/(1.0-ladder_stick_y))*ladder_speed_d_max;
                    (down_max*ladder_attack_speed_mul, -down_max*ladder_attack_speed_mul)
                } 
                else {
                    // lerp the down_max
                    let up_max = ((stick.y-ladder_stick_y)/(1.0-ladder_stick_y))*ladder_speed_d_max;
                    (up_max*ladder_attack_speed_mul, up_max*ladder_attack_speed_mul)
                }
            } 
            else {
                (0.0, 0.0)
            };
            do_standard_accel = false;
            energy.accel.x = 0.0;
            energy.accel.y = accel_y;
            energy.speed_max.x = 0.0;
            energy.speed_max.y = speed_max;
            0.0
        }
        _ => 0.0
    };
    if do_standard_accel {
        energy.accel.x = accel_diff;
        let speed_max = energy.speed_max.x*stick.x.abs();
        if energy.unk[1] != 0 {
            if !(((energy._x9c != 0.0 && (stick.x <= 0.0 || energy._xa0 <= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && (stick.x >= 0.0 || energy._xa0 >= 0.0 || speed_max.abs() <= energy._x9c.abs()))
            && ((stick.x <= 0.0 || 0.0 <= energy._xa0) && (0.0 <= stick.x || energy._xa0 <= 0.0))) {
                energy._x9c = speed_max;
                energy._xa0 = stick.x;
            }
        }
        energy.speed_max.x = speed_max;
    }
    /*
    //Double air brake value when above max horizontal jump speed
    if current_frame > 0.0 {
        let jump_speed_x_max = run_speed_max*ratio;
        if situation_kind == *SITUATION_KIND_AIR && energy.speed.x.abs() >= jump_speed_x_max {
            energy.speed_brake.x *= 2.0;
        }
    }
    */
    energy.process(boma);
    if !*(status_module as *const bool).add(0x12a) {
        if situation_kind == *SITUATION_KIND_AIR {
            if common_air_speed_x_limit < energy.speed.x.abs() {
                energy.speed.x = vertical_limit*energy.speed.x.signum();
            }
            if vertical_limit < energy.speed.y.abs() {
                energy.speed.y = vertical_limit*energy.speed.y.signum();
            }
        } 
        else if situation_kind == *SITUATION_KIND_GROUND {
            if ground_speed_limit < energy.speed.x.abs() {
                energy.speed.x = ground_speed_limit*energy.speed.x.signum();
            }
        }
    }
    energy.speed_max = backup_max;
    energy.speed_brake = backup_brake;
}

#[skyline::hook(offset = 0x6d4060)]
unsafe extern "C" fn control_initialize(energy: &mut FighterKineticEnergyControl, boma: &mut BattleObjectModuleAccessor) {
    let kind = utility::get_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let scale = PostureModule::scale(boma);
    let walk_accel_mul = WorkModule::get_param_float(boma, hash40("walk_accel_mul"), 0);
    let walk_accel_add = WorkModule::get_param_float(boma, hash40("walk_accel_add"), 0);
    let walk_speed_max = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
    let run_accel_mul = WorkModule::get_param_float(boma, hash40("run_accel_mul"), 0);
    let run_accel_add = WorkModule::get_param_float(boma, hash40("run_accel_add"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let ground_speed_limit = WorkModule::get_param_float(boma, hash40("common"), hash40("ground_speed_limit"));
    let air_accel_x_mul = WorkModule::get_param_float(boma, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(boma, hash40("air_accel_x_add"), 0);
    let air_brake_x = WorkModule::get_param_float(boma, hash40("air_brake_x"), 0);
    let stop_ceil_speed_x_stable_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("stop_ceil_speed_x_stable_mul"));
    let mut air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
    let air_speed_x_limit = WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_x_limit"));
    let brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0)*WorkModule::get_param_float(boma, hash40("common"), hash40("run_brake_brake_mul"));
    let rslash_air_max_x_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("rslash_air_max_x_mul"));
    let item_lift_accel_mul = WorkModule::get_param_float(boma, hash40("item_lift_accel_mul"), 0);
    let item_lift_accel_add = WorkModule::get_param_float(boma, hash40("item_lift_accel_add"), 0);
    let item_lift_speed_max = WorkModule::get_param_float(boma, hash40("item_lift_speed_max"), 0);
    let swim_brake = WorkModule::get_param_float(boma, hash40("common"), hash40("swim_brake"));
    let swim_accel_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("swim_accel_mul"));
    let swim_drown_speed_x_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("swim_drown_speed_x_mul"));
    let ladder_speed_u_max = WorkModule::get_param_float(boma, hash40("common"), hash40("ladder_speed_u_max"));
    let ladder_speed_d_max = WorkModule::get_param_float(boma, hash40("common"), hash40("ladder_speed_d_max"));
    let reset_type = std::mem::transmute(energy.energy_reset_type);
    use EnergyControllerResetType::*;
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            if reset_type == StopCeil {
                air_speed_x_stable *= stop_ceil_speed_x_stable_mul;
            }
            let ratio = WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
            let jump_speed_x_max = run_speed_max*ratio;
            energy.speed_max = PaddedVec2::new(air_speed_x_stable, -1.0);
            energy.speed_brake = PaddedVec2::new(air_brake_x, 0.0);
            let air_x_speed_max = if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) && energy.unk[2] == 0 {
                air_speed_x_limit
            } 
            else {
                if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                    jump_speed_x_max
                } 
                else {
                    -1.0
                }
            };
            energy.speed_limit = PaddedVec2::new(air_x_speed_max, 0.0);
            energy.accel_mul_x = air_accel_x_mul;
            energy.accel_add_x = air_accel_x_add;
        },
        FlyAdjust => {
            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } 
            else {
                return;
            };
            energy.speed_max = PaddedVec2::new(WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0), -1.0);
            energy.speed_brake = PaddedVec2::new(air_brake_x, 0.0);
            energy.speed_limit = PaddedVec2::new(air_speed_x_limit, 0.0);
            energy.accel_mul_x = air_accel_x_mul*fly_data.speed_x_mul;
            energy.accel_add_x = air_accel_x_add;
        },
        Dash | TurnRun | DashBack => {
            energy.speed_limit = PaddedVec2::new(ground_speed_limit, 0.0);
            energy.speed_max = PaddedVec2::new(run_speed_max, -1.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = run_accel_mul;
            energy.accel_add_x = run_accel_add;
        },
        ShootDash | ShootBackDash => {
            energy.speed_limit = PaddedVec2::new(ground_speed_limit, 0.0);
            energy.speed_max = PaddedVec2::new(run_speed_max, -1.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
        },
        RevolveSlashAir => {
            let speed_max = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0)*rslash_air_max_x_mul;
            energy.speed_max = PaddedVec2::new(speed_max, -1.0);
            energy.speed_brake = PaddedVec2::new(air_brake_x, 0.0);
            energy.speed_limit = PaddedVec2::new(air_speed_x_limit, 0.0);
            energy.accel_mul_x = rslash_air_max_x_mul;
        },
        Turn | TurnNoStop => {
            energy.speed_max = PaddedVec2::new(walk_speed_max, -1.0);
            energy.speed_limit = PaddedVec2::new(ground_speed_limit, 0.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = walk_accel_mul;
            energy.accel_add_x = walk_accel_add;
        },
        Free => {
            energy.speed_max = PaddedVec2::new(WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0), WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0));
            energy.speed_brake = PaddedVec2::new(air_brake_x, air_brake_x);
            energy.speed_limit = PaddedVec2::new(air_speed_x_limit, air_speed_x_limit);
            energy.accel_mul_x = air_accel_x_mul;
            energy.accel_mul_y = air_accel_x_mul;
            energy.accel_add_x = air_accel_x_add;
            energy.accel_add_y = air_accel_x_add;
        },
        ItemLift => {
            energy.speed_max = PaddedVec2::new(scale*item_lift_speed_max, -1.0);
            energy.speed_limit = PaddedVec2::new(scale*ground_speed_limit, 0.0);
            energy.speed_brake = PaddedVec2::new(brake, 0.0);
            energy.accel_mul_x = scale*item_lift_accel_mul;
            energy.accel_add_x = scale*item_lift_accel_add;
        },
        Swim => {
            energy.speed_brake = PaddedVec2::new(swim_brake, 0.0);
            energy.accel_mul_x = swim_accel_mul;
        },
        SwimDrown => {
            energy.speed_brake = PaddedVec2::new(swim_brake, 0.0);
            energy.accel_mul_x = swim_accel_mul*swim_drown_speed_x_mul;
        },
        TurnNoStopAir => {
            energy.speed_max = PaddedVec2::new(WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0), -1.0);
            energy.speed_limit = PaddedVec2::new(air_speed_x_limit, 0.0);
            energy.speed_brake = PaddedVec2::new(air_brake_x, 0.0);
            energy.accel_mul_x = air_accel_x_mul;
            energy.accel_add_x = air_accel_x_add;
        },
        Ladder => {
            energy.speed_brake = PaddedVec2::new(0.0, ladder_speed_u_max.max(ladder_speed_d_max));
        }
        _ => {}
    }
}

#[skyline::hook(offset = 0x6d4bc0)]
unsafe extern "C" fn control_setup(energy: &mut FighterKineticEnergyControl, reset_type: EnergyControllerResetType, initial_speed: &Vector3f, unk: u64, boma: &mut BattleObjectModuleAccessor) {
    let fighter = get_fighter_common_from_accessor(&mut *boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let kind = smash::app::utility::get_kind(boma);
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
    let shoot_dash_speed_f = WorkModule::get_param_float(boma, hash40("shoot_dash_speed_f"), 0);
    let shoot_dash_speed_b = WorkModule::get_param_float(boma, hash40("shoot_dash_speed_b"), 0);
    let rslash_air_spd_x_mul = WorkModule::get_param_float(boma, hash40("rslash_air_spd_x_mul"), 0);
    let new_speed = crate::functions::ext::utility::misc::KineticEnergy::adjust_speed_for_ground_normal(&energy.speed, boma);
    energy.clear_energy();
    energy.accel = PaddedVec2::zeros();
    energy.speed_max = PaddedVec2::zeros();
    energy.speed_brake = PaddedVec2::zeros();
    energy.speed_limit = PaddedVec2::new(-1.0, -1.0);
    energy.speed = PaddedVec2::new(initial_speed.x, initial_speed.y);
    energy.energy_reset_type = reset_type as u32;
    energy.accel_mul_x = 0.0;
    energy.accel_add_x = 0.0;
    energy.accel_mul_y = 0.0;
    energy.accel_add_y = 0.0;
    energy.lr = PostureModule::lr(boma);
    energy.unk[3] = 1;
    //The following permits characters to retain momentum during specials
    if situation_kind == *SITUATION_KIND_AIR {
        if kind == *FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        }
    }
    use EnergyControllerResetType::*;
    match reset_type {
        FallAdjust | FallAdjustNoCap | StopCeil | WallJump => {
            energy.unk[2] = if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE) {
                1
            } 
            else {
                0
            };
            if reset_type != FallAdjustNoCap
            && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT)
            && energy.unk[2] == 0 {
                if air_speed_x_stable < energy.speed.x.abs() {
                    energy.speed = PaddedVec2::new(air_speed_x_stable*energy.speed.x.signum(), 0.0);
                }
            }
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        },
        FlyAdjust => {
            let fly_data = if let Some(data) = FlyData::get_from_fighter_kind(kind) {
                data
            } 
            else {
                return;
            };
            let speed = *energy.get_speed();
            let sum = speed.x+speed.y;
            let cap = initial_speed.x*fly_data.speed_x_max_mul;
            if cap.abs() < sum.abs() {
                energy.speed.x = cap.abs()*speed.x.signum();
            }
        }, // not reached in game afaik
        Dash | TurnRun | DashBack => {
            let lr = if reset_type == DashBack {
                -energy.lr
            }
            else {
                energy.lr
            };
            energy.speed.y = 0.0;
            energy.speed.x = lr*dash_speed;
        },
        ShootDash => {
            energy.speed.x = if 0.0 <= energy.speed.x*energy.lr {
                energy.lr*shoot_dash_speed_f
            } 
            else {
                energy.speed.x+energy.lr*shoot_dash_speed_f
            };
        },
        ShootBackDash => {
            energy.speed.x = if 0.0 <= energy.speed.x*energy.lr {
                -energy.lr*shoot_dash_speed_b
            } 
            else {
                energy.speed.x-energy.lr*shoot_dash_speed_b
            };
        },
        RevolveSlashAir => {
            energy.speed.x *= rslash_air_spd_x_mul;
        },
        Free => {
            energy.speed = PaddedVec2::zeros();
        },
        MoveGround => {
            energy.speed = new_speed;
        }        
        _ => {}
    }
    energy.initialize(boma);
}

pub fn install() {
    skyline::install_hooks!(
        control_update,
        control_initialize,
        control_setup
    );
}