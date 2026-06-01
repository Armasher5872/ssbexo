use super::*;

//Shoulder Start Pre Status
unsafe extern "C" fn donkey_shoulder_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

//Shoulder Start Init Status
unsafe extern "C" fn donkey_shoulder_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let throw_invincible_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("throw_invincible_frame"));
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    LinkModule::send_event_nodes(boma, *LINK_NO_CAPTURE, Hash40::new("shoulder"), 0);
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_SET_IGNORE_CATCHING, true);
    sv_module_access::capture(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    HitModule::set_invincible_frame_global(fighter.module_accessor, throw_invincible_frame, false, 0);
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, donkey_shoulder_start_pre_status)
    .status(Init, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, donkey_shoulder_start_init_status)
    .install()
    ;
}