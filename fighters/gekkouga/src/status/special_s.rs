use super::*;

unsafe extern "C" fn gekkouga_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let end_frame_from_hash = MotionModule::end_frame_from_hash(boma, Hash40::new("special_s"));
    let chance = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("chance")) as f32;
    let rate = end_frame_from_hash/chance;
    WorkModule::set_int(boma, 0, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_BACK);
    MotionModule::change_motion(boma, Hash40::new("special_s"), 0.0, rate, false, 0.0, false, false);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP);
    WorkModule::set_int(boma, 0, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let special_s_warp_count = WorkModule::get_int(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let stick_play = WorkModule::get_param_float(boma, hash40("param_special_s"), hash40("stick_play"));
    let chance = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("chance"));
    if !WorkModule::is_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP) {
        if chance-1 <= special_s_warp_count {
            WorkModule::on_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP);
        }
    }
    if !StatusModule::is_changing(boma) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
        }
    }
    if stick_play >= stick_x*lr {
        if stick_play >= stick_x*-lr {
            WorkModule::on_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY) {
        if situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_z = PostureModule::pos_z(boma);
    let shadow_x_pos = WorkModule::get_float(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_X_POS);
    let shadow_y_pos = WorkModule::get_float(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_Y_POS);
    let warp_count = WorkModule::get_int(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let work_warp_frame = WorkModule::get_int(boma, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    let chance = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("chance"));
    let warp_frame = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("warp_frame"));
    WorkModule::inc_int(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    if chance > warp_count {
        return 0.into();
    }
    if work_warp_frame == 0 {
        VisibilityModule::set_whole(boma, false);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
        AreaModule::set_whole(boma, false);
    }
    WorkModule::inc_int(boma, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    if warp_frame <= work_warp_frame {
        if WorkModule::is_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_GIMMICK) {
            GroundModule::set_shape_safe_pos(boma, &Vector2f{x: shadow_x_pos, y: shadow_y_pos});
        }
        else {
            GroundModule::set_shape_safe_pos(boma, &Vector2f{x: pos_x, y: pos_y+0.1});
        }
        PostureModule::set_pos(boma, &Vector3f{x: shadow_x_pos, y: shadow_y_pos, z: pos_z});
        if situation_kind != *SITUATION_KIND_GROUND {
            if !WorkModule::is_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_AIR) {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
        else {
            if WorkModule::is_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_AIR) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
        fighter.change_status(FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
    }
    1.into()
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, gekkouga_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, gekkouga_special_s_exec_status)
    .install()
    ;
}