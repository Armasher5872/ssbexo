use super::*;

unsafe extern "C" fn edge_special_hi_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let charged_rush = WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let correct_kind = if charged_rush {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP} else {*GROUND_CORRECT_KIND_GROUND};
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, correct_kind as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn edge_special_hi_landing_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let charged_rush = WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let speed_mul = if !WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_USE_LANDING_SPEED_MUL) {
        edge_special_hi_param_float_helper(fighter, hash40("ground_speed_x_mul").into(), charged_rush.into()).get_f32()
    }
    else {
        edge_special_hi_param_float_helper(fighter, hash40("landing_speed_x_mul").into(), charged_rush.into()).get_f32()
    };
    let landing_brake_x = edge_special_hi_param_float_helper(fighter, hash40("landing_brake_x").into(), charged_rush.into()).get_f32();
    let get_stop_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let get_stop_speed = lua_bind::KineticEnergy::get_speed(get_stop_energy as *mut smash::app::KineticEnergy);
    lua_bind::KineticEnergyNormal::set_speed(get_stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: get_stop_speed*speed_mul, y: 0.0});
    lua_bind::KineticEnergyNormal::set_brake(get_stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: landing_brake_x, y: 0.0});
    if !charged_rush {
        MotionModule::change_motion(boma, Hash40::new("special_hi1_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("special_hi2_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_landing_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_hi_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let charged_rush = WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let end_frame = MotionModule::end_frame(boma);
    let frame = MotionModule::frame(boma);
    let hit_id = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    let mut is_near_floor = false;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if charged_rush && frame < 2.0 {
        if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                if hit_id != *BATTLE_OBJECT_ID_INVALID {
                    let opponent_battle_object = get_battle_object_from_id(hit_id as u32);
                    let opponent_boma = (*opponent_battle_object).module_accessor;
                    let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
                    let opponent_pos = *PostureModule::pos(opponent_boma);
                    let pos = *PostureModule::pos(boma);
                    let lr = PostureModule::lr(boma);
                    let mut pos_vec = Vector3f{x: pos.x, y: pos.y, z: pos.z};
                    let mut opponent_pos_vec = Vector3f{x: opponent_pos.x, y: opponent_pos.y, z: opponent_pos.z};
                    let dist_check = if lr < 0.0 {opponent_pos.x.abs() <= pos.x.abs()} else {opponent_pos.x.abs() >= pos.x.abs()};
                    if dist_check && GroundModule::get_distance_to_floor(opponent_boma, &opponent_pos_vec, 20.0, true) == -1.0 {
                        EFFECT(opponent_agent, Hash40::new("edge_throwb_teleport"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                        EFFECT(opponent_agent, Hash40::new("edge_throwb_bom2"), Hash40::new("rot"), 0, 0, -10.0*lr, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                        EFFECT(fighter, Hash40::new("edge_throwb_teleport"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                        EFFECT(fighter, Hash40::new("edge_throwb_bom2"), Hash40::new("rot"), 0, 0, -20.0*lr, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                        opponent_pos_vec.x = pos.x-(10.0*lr);
                        pos_vec.x -= 20.0*lr;
                    }
                    if GroundModule::get_distance_to_floor(opponent_boma, &opponent_pos_vec, 20.0, true) != -1.0 {
                        opponent_pos_vec.y -= GroundModule::get_distance_to_floor(opponent_boma, &opponent_pos_vec, 20.0, true);
                        is_near_floor = true;
                    }
                    PostureModule::set_pos(boma, &pos_vec);
                    PostureModule::init_pos(boma, &pos_vec, true, true);
                    PostureModule::set_pos(opponent_boma, &opponent_pos_vec);
                    PostureModule::init_pos(opponent_boma, &opponent_pos_vec, true, true);
                    if is_near_floor {
                        opponent_agent.set_situation(SITUATION_KIND_GROUND.into());
                        GroundModule::attach_ground(opponent_boma, true);
                        GroundModule::set_correct(opponent_boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                        KineticModule::clear_speed_energy_id(opponent_boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_FURAFURA, false);
                    }
                }
                WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME);
                fighter.change_status(FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED.into(), false.into());
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL) {
        if !charged_rush {
            if end_frame-frame <= 9.0 {
                CancelModule::enable_cancel(boma);
            } 
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(boma) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_landing_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let object_id = get_table_value(table, "object_id_").try_integer().unwrap();
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) 
            && WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH)
            && !WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME) {
                WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME);
                WorkModule::set_int(boma, object_id as i32, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("edge_octaslash_line"), true, true);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL);
    0.into()
}

unsafe extern "C" fn edge_special_hi_landing_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_OCTASLASH_PRIME);
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_pre_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_main_status)
    .status(CheckAttack, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_check_attack_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_end_status)
    .status(Exit, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_exit_status)
    .install()
    ;
}