use super::*;

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn donkey_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let kinetic_type = KineticModule::get_kinetic_type(boma);
        let frame = MotionModule::frame(boma);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), false) as f32;
        let forward = [30.0, 31.0, 32.0, 33.0, 77.0, 78.0, 79.0, 80.0, 81.0, 82.0, 83.0, 84.0, 85.0, 86.0, 87.0, 137.0, 138.0, 139.0, 140.0, 141.0, 142.0, 143.0, 144.0, 145.0, 146.0, 147.0, 196.0, 197.0, 198.0, 199.0, 200.0, 201.0, 202.0, 203.0, 204.0].contains(&frame);
        let forward_diagonal = [34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 88.0, 89.0, 90.0, 91.0, 92.0, 93.0, 94.0, 148.0, 149.0, 150.0, 151.0, 152.0, 205.0, 206.0, 207.0, 208.0, 209.0].contains(&frame);
        let down = [40.0, 41.0, 42.0, 43.0, 95.0, 96.0, 97.0, 98.0, 99.0, 100.0, 153.0, 154.0, 155.0, 156.0, 157.0, 158.0, 159.0, 160.0].contains(&frame);
        let down_diagonal = [40.0, 41.0, 42.0, 43.0, 44.0, 45.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 161.0, 162.0, 163.0, 164.0, 165.0, 166.0, 167.0, 168.0, 169.0].contains(&frame);
        let backward = [46.0, 47.0, 48.0, 49.0, 50.0, 110.0, 111.0, 112.0, 113.0, 114.0, 115.0, 116.0, 117.0, 118.0, 170.0, 171.0, 172.0, 173.0, 174.0, 175.0, 176.0, 177.0].contains(&frame);
        let backward_diagonal = [51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0, 59.0, 60.0, 119.0, 120.0, 121.0, 122.0, 123.0, 178.0, 179.0, 180.0, 181.0, 182.0].contains(&frame);
        let up = [61.0, 62.0, 63.0, 64.0, 65.0, 66.0, 67.0, 68.0, 69.0, 70.0, 124.0, 125.0, 126.0, 127.0, 128.0, 129.0, 130.0, 131.0, 183.0, 184.0, 185.0].contains(&frame);
        let up_diagonal = [71.0, 72.0, 73.0, 74.0, 75.0, 76.0, 132.0, 133.0, 134.0, 135.0, 136.0, 186.0, 187.0, 188.0, 189.0, 190.0, 191.0, 192.0].contains(&frame);
        let timer = WorkModule::get_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
        //Taunts
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind)
            && frame >= 48.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
                || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                    MotionModule::set_frame_sync_anim_cmd(boma, 32.0, true, true, false);
                };
            }
            if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind)
            && frame >= 43.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
                || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
                || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)  {
                    MotionModule::set_frame_sync_anim_cmd(boma, 15.0, true, true, false);
                };
            }
        };
        //Dash Attack
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && (MotionModule::is_end(boma) || (cancel_frame > 0.0 && frame >= cancel_frame) || (CancelModule::is_enable_cancel(boma))) {
            WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
        }
        if WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN) {
            AttackModule::set_power_up(boma, 0.85);
        }
        else {
            AttackModule::set_power_up(boma, 1.0);
        }
        if [hash40("special_hi"), hash40("special_air_hi")].contains(&motion_kind) {
            if motion_kind == hash40("special_air_hi") {
                WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
            }
            else {
                if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE) {
                   StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                   AttackModule::clear_all(boma);
                }
            }
        }
        else {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        }
        //Barrel Cannon
        if motion_kind == hash40("special_air_hi") {
            if (29.0..=209.0).contains(&frame) {
                if forward {
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if forward_diagonal {
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 1, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if down {
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 2, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if down_diagonal {
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 3, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if backward {
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_int(boma, 4, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if backward_diagonal {
                    WorkModule::set_float(fighter.module_accessor, -8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 5, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if up {
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 6, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                else if up_diagonal {
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
                    WorkModule::set_float(fighter.module_accessor, 8.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
                    WorkModule::set_int(boma, 7, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
                }
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("special_air_hi"), 209.0, 1.0, false, 0.0, false, false);
                }
            }
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
        }
        //Down Special
        if timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
        }
        if timer <= 0
        && WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) {
            WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
            fighter.gimmick_flash();
        }
        if [
            *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_TURN, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_WALK, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_PASS, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT,
            *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP_SQUAT_B, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_LANDING
        ].contains(&status_kind) {
            if WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE)
            && timer > 0 {
                if ItemModule::get_have_item_kind(boma, 0) == *ITEM_KIND_BARREL {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY, true);
                }
            }
        }
        if [hash40("special_lw_start"), hash40("special_lw_loop")].contains(&motion_kind)
        || [*FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        donkey_frame
    );
}