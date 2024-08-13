use super::*;

unsafe extern "C" fn rosetta_special_guard_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn rosetta_special_guard_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn rosetta_special_guard_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(rosetta_special_guard_main_loop as *const () as _))
}

unsafe extern "C" fn rosetta_special_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let transit_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    let start_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_X);
    let start_y = WorkModule::get_float(fighter.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_Y);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
        let tico = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
        let tico_id = smash::app::lua_bind::Article::get_battle_object_id(tico) as u32;
        let tico_boma = sv_battle_object::module_accessor(tico_id);
        let tico_status_kind = StatusModule::status_kind(tico_boma);
        let tico_start_x = WorkModule::get_float(tico_boma, WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_X);
        let tico_start_y = WorkModule::get_float(tico_boma, WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_Y);
        let tico_status_check = ![*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, *WEAPON_ROSETTA_TICO_STATUS_KIND_DOWN, *WEAPON_ROSETTA_TICO_STATUS_KIND_STANDBY, *WEAPON_ROSETTA_TICO_STATUS_KIND_NONE, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_AIR, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FALL, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *WEAPON_ROSETTA_TICO_STATUS_KIND_DAMAGE_FLY_REFLECT_U].contains(&tico_status_kind);
        if transit_timer <= 0 && tico_status_check {
            if frame == 1.0 {
                //Starts teleport
                HitModule::set_whole(tico_boma, HitStatus(*HIT_STATUS_OFF), 0);
                VisibilityModule::set_whole(tico_boma, false);
                JostleModule::set_status(tico_boma, false);
                WorkModule::set_float(fighter.module_accessor, PostureModule::pos_x(fighter.module_accessor), FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_X);
                WorkModule::set_float(fighter.module_accessor, PostureModule::pos_y(fighter.module_accessor), FIGHTER_ROSETTA_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_Y);
                WorkModule::set_float(tico_boma, PostureModule::pos_x(tico_boma), WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_X);
                WorkModule::set_float(tico_boma, PostureModule::pos_y(tico_boma), WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLOAT_WORMHOLE_TRANSIT_START_Y);
                let handle = EffectModule::req_on_joint(tico_boma, Hash40::new("rosetta_escape"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 0.5, &NONE_VECTOR, &NONE_VECTOR, false, 0, 0, 0);
                EffectModule::set_alpha(tico_boma, handle as u32, 1.0);
            }
            if frame == 9.0 {
                //Ends teleport
                HitModule::set_whole(tico_boma, HitStatus(*HIT_STATUS_NORMAL), 0);
                VisibilityModule::set_whole(tico_boma, true);
                JostleModule::set_status(tico_boma, true);
                PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: tico_start_x, y: tico_start_y, z: PostureModule::pos_z(fighter.module_accessor)});
                PostureModule::set_pos(tico_boma, &Vector3f{x: start_x, y: start_y, z: PostureModule::pos_z(tico_boma)});
                PostureModule::init_pos(fighter.module_accessor, &Vector3f{x: tico_start_x, y: tico_start_y, z: PostureModule::pos_z(fighter.module_accessor)}, true, true);
                PostureModule::init_pos(tico_boma, &Vector3f{x: start_x, y: start_y, z: PostureModule::pos_z(tico_boma)}, true, true);
                WorkModule::set_int(fighter.module_accessor, 300, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
                let handle = EffectModule::req_on_joint(tico_boma, Hash40::new("rosetta_escape_end"), Hash40::new("top"), &NONE_VECTOR, &NONE_VECTOR, 1.0, &NONE_VECTOR, &NONE_VECTOR, false, 0, 0, 0);
                EffectModule::set_alpha(tico_boma, handle as u32, 1.0);
            }
        }
    }
    if transit_timer > 0 {
        if frame > 29.0 {
            CancelModule::enable_cancel(fighter.module_accessor);  
        }
    }
    else {
        if frame > 20.0 {
            CancelModule::enable_cancel(fighter.module_accessor);   
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn rosetta_special_guard_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn rosetta_special_guard_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let transit_timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if transit_timer <= 0 {
        WorkModule::set_int(fighter.module_accessor, 300, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    }
    0.into()
}

unsafe extern "C" fn rosetta_special_guard_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("rosetta")
    .status(Pre, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_pre_status)
    .status(Init, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_init_status)
    .status(Main, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_main_status)
    .status(Exec, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_exec_status)
    .status(End, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_end_status)
    .status(Exit, FIGHTER_STATUS_KIND_SPECIAL_GUARD, rosetta_special_guard_exit_status)
    .install()
    ;
}