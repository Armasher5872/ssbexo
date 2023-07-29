use super::*;

#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn fox_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        let frame = MotionModule::frame(boma);
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
        let parry_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        let parried = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind) {
            if (0.0..5.0).contains(&frame) {
                WorkModule::set_int(boma, 1, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
                WorkModule::set_int(boma, 180, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
            }
        }
        if parry_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        }
        if parry_timer <= 0
        && parried == 1 {
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        }
        //Up Taunt Fire Fox Cancel
        if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) {
            if (41.0..=44.0).contains(&frame) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, false);
                }
            }
        }
        //Jab Cancels
        if [hash40("attack_11"), hash40("attack_12")].contains(&motion_kind)
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            } 
            else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            } 
            else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            };
        };
        //Fast Fall Blaster/Land Cancel Blaster (Not doing it in statuses yet)
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR 
            && (ControlModule::get_command_flag_cat(boma, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
            && stick_y < -0.66
            && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            };
            if StatusModule::is_situation_changed(boma) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
            };
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            if StatusModule::is_situation_changed(boma) {
                if situation_kind == *SITUATION_KIND_AIR {
                    KineticModule::unable_energy_all(fighter.module_accessor);
                    KineticModule::clear_speed_all(fighter.module_accessor);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                }
            }
        }
        if status_kind == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND {
            if frame > 10.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        fox_frame
    );
}