#![allow(unused_macros)]
use {
    crate::functions::{
        BARREL_ACTIVE,
        BARREL_DIRECTION,
        BARREL_TIMER,
        DONKEY_DASH_ATTACK_JUMP,
        DONKEY_DASH_ATTACK_POWER_DOWN,
        DONKEY_GIANT_PUNCH_STAGE,
        FIGHTER_SPECIAL_STATE,
        SPEED_X,
        SPEED_Y,
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn donkey_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let kinetic_type = KineticModule::get_kinetic_type(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let forward = [30.0, 31.0, 32.0, 33.0, 77.0, 78.0, 79.0, 80.0, 81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 137.0, 138.0, 139.0, 140.0, 141.0, 142.0, 143.0, 144.0, 145.0, 146.0, 147.0, 196.0, 197.0, 198.0, 199.0, 200.0, 201.0, 202.0, 203.0, 204.0].contains(&frame);
        let forward_diagonal = [34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 88.0, 89.0, 90.0, 91.0, 92.0, 93.0, 94.0, 148.0, 149.0, 150.0, 151.0, 152.0, 205.0, 206.0, 207.0, 208.0, 209.0].contains(&frame);
        let down = [40.0, 41.0, 42.0, 43.0, 95.0, 96.0, 97.0, 98.0, 99.0, 100.0, 153.0, 154.0, 155.0, 156.0, 157.0, 158.0, 159.0, 160.0].contains(&frame);
        let down_diagonal = [40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 161.0, 162.0, 163.0, 164.0, 165.0, 166.0, 167.0, 168.0, 169.0].contains(&frame);
        let backward = [46.0, 47.0, 48.0, 49.0, 50.0, 110.0, 111.0, 112.0, 113.0, 114.0, 115.0, 116.0, 117.0, 118.0, 170.0, 171.0, 172.0, 173.0, 174.0, 175.0, 176.0, 177.0].contains(&frame);
        let backward_diagonal = [51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0, 60.0, 119.0, 120.0, 121.0, 122.0, 123.0, 178.0, 179.0, 180.0, 181.0, 182.0].contains(&frame);
        let up = [61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0, 70.0, 124.0, 125.0, 126.0, 127.0, 128.0, 129.0, 130.0, 131.0, 183.0, 184.0, 185.0].contains(&frame);
        let up_diagonal = [71.0, 72.0, 73.0, 74.0, 75.0, 76.0, 132.0, 133.0, 134.0, 135.0, 136.0, 186.0, 187.0, 188.0, 189.0, 190.0, 191.0, 192.0].contains(&frame);
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind) 
        && MotionModule::end_frame(module_accessor) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind)
            && frame >= 48.0 {
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                    MotionModule::set_frame_sync_anim_cmd(module_accessor, 32.0, true, true, false);
                };
            }
            if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind)
            && frame >= 43.0 {
                if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)  {
                    MotionModule::set_frame_sync_anim_cmd(module_accessor, 15.0, true, true, false);
                };
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                macros::SET_SPEED_EX(fighter, 1.0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }   
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) 
            && frame >= 9.0
            && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                DONKEY_DASH_ATTACK_JUMP[entry_id] = 1;
                DONKEY_DASH_ATTACK_POWER_DOWN[entry_id] = true;
            }
        }
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == true
        && status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH {
            DONKEY_DASH_ATTACK_POWER_DOWN[entry_id] = false;
        }
        if DONKEY_DASH_ATTACK_POWER_DOWN[entry_id] == true {
            AttackModule::set_power_up(module_accessor, 0.85);
        }
        else {
            AttackModule::set_power_up(module_accessor, 1.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        && DONKEY_DASH_ATTACK_JUMP[entry_id] == 1 {
            MotionModule::set_rate(fighter.module_accessor, 0.375);
            DONKEY_DASH_ATTACK_JUMP[entry_id] = 0;
        }
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
            DONKEY_DASH_ATTACK_JUMP[entry_id] = 0;
        }
        if [*FIGHTER_KINETIC_TYPE_JUMP, *FIGHTER_KINETIC_TYPE_JUMP_ICE, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_CLIFF_VERTICAL, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND].contains(&kinetic_type)
        && DONKEY_DASH_ATTACK_JUMP[entry_id] == 1 {
            DONKEY_DASH_ATTACK_JUMP[entry_id] = 0;
        }
        if DONKEY_DASH_ATTACK_JUMP[entry_id] == 1 {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
        }
        if [hash40("special_n_loop"), hash40("special_air_n_loop")].contains(&motion_kind) {
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                DONKEY_GIANT_PUNCH_STAGE[entry_id] += 1;
            }
        }
        if [hash40("special_s"), hash40("special_air_s"), hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) {
            if [hash40("special_air_s"), hash40("special_air_hi")].contains(&motion_kind) {
                FIGHTER_SPECIAL_STATE[entry_id] = true;
            }
            else {
                if FIGHTER_SPECIAL_STATE[entry_id] {
                   StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                   AttackModule::clear_all(module_accessor);
                }
            }
        }   
        else {
            FIGHTER_SPECIAL_STATE[entry_id] = false;
        }
        if motion_kind == hash40("special_air_hi") {
            if frame >= 29.0
            && frame <= 209.0 {
                if forward {
                    SPEED_X[entry_id] = 8.0;
                    BARREL_DIRECTION[entry_id] = 0;
                }
                else if forward_diagonal {
                    SPEED_X[entry_id] = 8.0;
                    SPEED_Y[entry_id] = -8.0;
                    BARREL_DIRECTION[entry_id] = 1;
                }
                else if down {
                    SPEED_Y[entry_id] = -8.0;
                    BARREL_DIRECTION[entry_id] = 2;
                }
                else if down_diagonal {
                    SPEED_X[entry_id] = -8.0;
                    SPEED_Y[entry_id] = -8.0;
                    BARREL_DIRECTION[entry_id] = 3;
                }
                else if backward {
                    SPEED_X[entry_id] = -8.0;
                    BARREL_DIRECTION[entry_id] = 4;
                }
                else if backward_diagonal {
                    SPEED_X[entry_id] = -8.0;
                    SPEED_Y[entry_id] = 8.0;
                    BARREL_DIRECTION[entry_id] = 5;
                }
                else if up {
                    SPEED_Y[entry_id] = 8.0;
                    BARREL_DIRECTION[entry_id] = 6;
                }
                else if up_diagonal {
                    SPEED_X[entry_id] = 8.0;
                    SPEED_Y[entry_id] = 8.0;
                    BARREL_DIRECTION[entry_id] = 7;
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(module_accessor, Hash40::new("special_air_hi"), 209.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        if BARREL_TIMER[entry_id] > 0 {
            BARREL_TIMER[entry_id] -= 1;
        }
        if BARREL_TIMER[entry_id] <= 0
        && BARREL_ACTIVE[entry_id] == true {
            BARREL_ACTIVE[entry_id] = false;
        }
        println!("Speed_X: {}", SPEED_X[entry_id]);
        println!("Speed_Y: {}", SPEED_Y[entry_id]);
    }
}

pub fn install() {
    install_agent_frames!(
        donkey_frame
    );
}