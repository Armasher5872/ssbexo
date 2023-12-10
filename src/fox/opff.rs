use super::*;

unsafe extern "C" fn fox_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    //Jab Cancels
    if fighter.magic_series() == 1 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
    }
    if fighter.magic_series() == 2 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
    }
    if fighter.magic_series() == 3 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
    }
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

pub fn install() {
    Agent::new("fox")
    .on_line(Main, fox_frame)
    .install()
    ;
}