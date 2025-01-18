use super::*;

unsafe extern "C" fn rockman_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_METALBLADE) {
        LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x273692b070), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(rockman_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn rockman_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut change_motion = false;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_FRAME_END) {
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        if 0.1225 < stick_x*stick_x+stick_y*stick_y {
            WorkModule::set_float(fighter.module_accessor, stick_x, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_X);
            WorkModule::set_float(fighter.module_accessor, stick_y, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_Y);
        }
    }
    else if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_ACCEPT) {
        let stick_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_X);
        let stick_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLOAT_STICK_Y);
        if 0.1225 < stick_x*stick_x+stick_y*stick_y {
            let atan = stick_y.atan2(stick_x*lr);
            let rad = 112.5f32.to_radians();
            let rad_negative = -112.5f32.to_radians();
            if rad <= atan || atan <= rad_negative {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_TURN_MOTION);
                change_motion = true;
            }
            if stick_y > 0.7 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_UP_MOTION);
                change_motion = true;
            }
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_STICK_CHECK_ACCEPT);
    }
    if StatusModule::is_changing(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) || change_motion {
        let ground_motion_kind;
        let air_motion_kind;
        let is_exist = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_ALREADY_EXIST_METALBLADE);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_TURN_MOTION) {
            if !is_exist {
                ground_motion_kind = hash40("special_n_turn");
                air_motion_kind = hash40("special_air_n_turn");
            }
            else {
                ground_motion_kind = hash40("special_n_turn_empty");
                air_motion_kind = hash40("special_air_n_turn_empty");
            }
        }
        else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_UP_MOTION) {
            ground_motion_kind = hash40("special_n_hi");
            air_motion_kind = hash40("special_air_n_hi");
        }
        else {
            if !is_exist {
                ground_motion_kind = hash40("special_n");
                air_motion_kind = hash40("special_air_n");
            }
            else {
                ground_motion_kind = hash40("special_n_empty");
                air_motion_kind = hash40("special_air_n_empty");
            }
        }
        rockman_special_motion_helper(fighter, ground_motion_kind.into(), air_motion_kind.into(), FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_FALL.into(), FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_FIRST.into(), GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into());
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn rockman_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let metalblade_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_INT_METALBLADE_ID);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_STATUS_SPECIAL_N_WORK_ID_FLAG_UP_MOTION);
    if metalblade_id != 0 {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x29b79a80a1));
    }
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, rockman_special_n_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, rockman_special_n_end_status)
    .install()
    ;
}