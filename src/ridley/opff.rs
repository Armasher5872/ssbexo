//Credit to AParticularUser for the code
#![allow(unused_macros)]
use {
    crate::functions::variables::*,
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        }
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn ridley_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let frame = MotionModule::frame(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let pos_x_global = PostureModule::pos_x(module_accessor);
        let pos_y_global = PostureModule::pos_y(module_accessor);
        let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position(module_accessor, Hash40::new("tail8"), v3f_tail_pos, false);
        let lr = PostureModule::lr(module_accessor);
        let pos_x_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].x;
        let pos_y_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].y;
        RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id] = Vector2f{x: v3f_tail_pos.x-pos_x_global, y: v3f_tail_pos.y-pos_y_global}; //save current tail pos relative to fighter
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] > 0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] = 0;
            }
        }
        if [hash40("special_lw_stab"), hash40("special_air_lw_stab")].contains(&motion_kind) {
            if motion_kind == hash40("special_air_lw_stab") {
                FIGHTER_SPECIAL_STATE[entry_id] = true;
            }
            else {
                if FIGHTER_SPECIAL_STATE[entry_id] {
                   StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
                   AttackModule::clear_all(module_accessor);
                }
            }
        }
        else {
            FIGHTER_SPECIAL_STATE[entry_id] = false;
        }
        if motion_kind == hash40("special_air_lw_stab") {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
            if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_ALL) == true
            && POGO_OPPONENT_BOUNCE[entry_id] != true {
                POGO_OPPONENT_BOUNCE[entry_id] = true;
            }
            else if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x:pos_x_prev+pos_x_global, y: pos_y_prev+pos_y_global}, &Vector2f{x: (v3f_tail_pos.x -(pos_x_prev+pos_x_global))+(8.0*lr), y: v3f_tail_pos.y -(pos_y_prev+pos_y_global) -8.0}, ground_hit_pos, true) == 1
            && POGO_GROUND_BOUNCE[entry_id] != true
            && frame > 21.0
            && frame < 35.0 {
                POGO_GROUND_BOUNCE[entry_id] = true;
            }
        }
        if POGO_OPPONENT_BOUNCE[entry_id] == true {
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::SET_SPEED_EX(fighter, velocity_x*0.5, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            POGO_OPPONENT_BOUNCE[entry_id] = false;
        }
        if POGO_GROUND_BOUNCE[entry_id] == true {
            let mut slope_angle = 0.0;
            let slope_check_pos = &mut Vector2f{x: 0.0, y: 0.0};
            if GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x:ground_hit_pos.x+(5.0*lr), y:ground_hit_pos.y+5.0}, &Vector2f{x:0.0, y:-10.0}, slope_check_pos, true) == 1 {
                let pos_diff_y = ground_hit_pos.y-slope_check_pos.y;
                if pos_diff_y > 0.0 {
                    slope_angle = (pos_diff_y / 5.0).atan().to_degrees();
                }
                else {
                    slope_angle = 360.0 -((-pos_diff_y / 5.0).atan().to_degrees());
                }
            }
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), (ground_hit_pos.x-pos_x_global)*lr, ground_hit_pos.y -pos_y_global, 0, slope_angle, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), (ground_hit_pos.x-pos_x_global)*lr, ground_hit_pos.y -pos_y_global, 0, slope_angle, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            macros::PLAY_SE(fighter, Hash40::new("se_ridley_special_h03"));
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
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
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            macros::SET_SPEED_EX(fighter, velocity_x*0.5, velocity_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            POGO_GROUND_BOUNCE[entry_id] = false;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ridley_frame
    );
}