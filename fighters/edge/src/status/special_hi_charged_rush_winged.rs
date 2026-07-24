use super::*;

//Octaslash Prime Pre Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

//Octaslash Prime Init Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Octaslash Prime Main Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    JostleModule::set_weight(boma, 0.0);
    MotionModule::change_motion(boma, Hash40::new("special_hi2_end_wing"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(edge_special_hi_charged_rush_winged_main_loop as *const () as _))
}

unsafe extern "C" fn edge_special_hi_charged_rush_winged_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let scale = PostureModule::scale(boma);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if frame == 1 {
        if !smash2::app::FighterCutInManager::is_vr_mode() {
            if smash2::app::FighterCutInManager::is_one_on_one_including_thrown(&*(boma as *const smash2::app::BattleObjectModuleAccessor)) {
                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
                    if !smash2::app::stage::is_flat_stage() {
                        CHECK_VALID_START_CAMERA(fighter, 0, 0, 0, 0, 0, 0, false);
                        REQ_MOTION_CAMERA(fighter, Hash40::new("d02specialhiendwing.nuanmb"), false);
                    }
                }
                CAM_ZOOM_IN_arg5(fighter, 7.0, 0.0, scale*1.5, 0.0, 0.0);
                SLOW_OPPONENT(fighter, 30.0, 36.0);
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

//Octaslash Prime Exec Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Octaslash Prime Check Attack Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let opponent_id = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
            let opponent_battle_object = get_battle_object_from_id(opponent_id as u32);
            let opponent_boma = (*opponent_battle_object).module_accessor;
            if frame < 100 {
                StopModule::set_hit_stop_frame_fix(opponent_boma, 8);
                WorkModule::set_int(opponent_boma, 12, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                WorkModule::set_int(opponent_boma, 12, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
            }
        }
    }
    0.into()
}

//Octaslash Prime End Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    JostleModule::set_weight(boma, 1.0);
    0.into()
}

//Octaslash Prime Exit Status
unsafe extern "C" fn edge_special_hi_charged_rush_winged_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    JostleModule::set_weight(boma, 1.0);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_pre_status)
    .status(Init, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_init_status)
    .status(Main, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_main_status)
    .status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_exec_status)
    .status(CheckAttack, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_check_attack_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_end_status)
    .status(Exit, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH_WINGED, edge_special_hi_charged_rush_winged_exit_status)
    .install()
    ;
}