use super::*;

unsafe extern "C" fn dolly_dash_back_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

unsafe extern "C" fn dolly_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE, false, -1);
            if situation_kind != *SITUATION_KIND_GROUND {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE) {
                    let hop_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("hop_speed_y"));
                    let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
                    let gravity_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_max"));
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, hop_speed_y);
                    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_accel);
                    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_max);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
                }
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE) {
            if situation_kind != *SITUATION_KIND_GROUND {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let gravity = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
                let gravity_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_max"));
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, gravity, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -gravity_accel);
                sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_max);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        else {
            fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        }
        if cancel_statuses {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED)
        && fighter.global_table[CURRENT_FRAME].get_f32() >= 11.0
        && situation_kind != *SITUATION_KIND_AIR {
            sv_animcmd::STOP_SE(fighter.lua_state_agent);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn dolly_special_n_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    0.into()
}

unsafe extern "C" fn dolly_super_special_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_super_special2") as u64, hash40("speed_mul") as u64);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("super_special2_start"), 0.0, 1.0, false, 0.0, false, false);
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
    }
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_CATEGORY_INVALID, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_CATEGORY);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_HIT_OBJECT_ID);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_super_special_2_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_super_special_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let cancel_statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&prev_status_kind);
    let speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_super_special2") as u64, hash40("speed_mul") as u64);
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
            && !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_GROUND
            && prev_situation_kind == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
            }
            if situation_kind == *SITUATION_KIND_AIR
            && prev_situation_kind == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_mul);
            }
            if cancel_statuses {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_NONE);
                sv_kinetic_energy!(set_speed_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0);
                if frame >= 13.0 {
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
                    sv_animcmd::STOP_SE(fighter.lua_state_agent);
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_FLAG_HIT) {
                return 0.into();
            }
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_HIT_OBJECT_ID);
        }
    }
    0.into()
}

unsafe extern "C" fn dolly_super_special_2_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE, ArticleOperationTarget(0));
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_punch"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_dash"), true, true);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
    0.into()
}

pub fn install() {
    Agent::new("dolly")
    .status(Main, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, dolly_dash_back_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, dolly_special_n_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, dolly_special_n_end_status)
    .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, dolly_super_special_2_main_status)
    .status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, dolly_super_special_2_end_status)
    .install()
    ;
}