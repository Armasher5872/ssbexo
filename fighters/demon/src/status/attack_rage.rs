use super::*;

//Rage Drive Pre Status
unsafe extern "C" fn demon_attack_rage_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let correct_kind = if situation_kind != *SITUATION_KIND_GROUND {*GROUND_CORRECT_KIND_AIR} else {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK};
    let status_attr = if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE) {*FIGHTER_STATUS_ATTR_START_TURN} else {0}; 
    WorkModule::set_flag(boma, WorkModule::get_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_RAGE_SYSTEM_FRAME) > 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(correct_kind), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, 0, status_attr as u32, (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW | *FIGHTER_POWER_UP_ATTACK_BIT_THROW) as u32, 0);
    0.into()
}

//Rage Drive End Status
unsafe extern "C" fn demon_attack_rage_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
    fun_7100028ee0(fighter);
    if status_kind != *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE_CATCH {
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ATTACK_RAGE_CAPTURE);
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_ATTACK_RAGE_DRIVE_DISABLE_EFFECT) {
            if 0 < WorkModule::get_int(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_RAGE_SYSTEM_FRAME) {
                MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_EFFECT, Hash40::new_raw(0x15115d148e), -1);
            }
        }
        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        sub_rage_system(global_fighter, true);
    }
    0.into()
}

unsafe extern "C" fn fun_7100028ee0(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_DEMON_STATUS_KIND_SPECIAL_LW_CATCH, *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE_CATCH].contains(&status_kind) {
        fun_710001f3f0(fighter);
    }
}

unsafe extern "C" fn fun_710001f3f0(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_STATUS_KIND_DEAD {
        if !WorkModule::is_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_CAPTURE_CUT) {
            CatchModule::set_send_cut_event(boma, true);
        }
        else {
            CatchModule::set_send_cut_event(boma, false);
        }
        CatchModule::catch_cut(boma, false, false);
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_STATUS_SPECIAL_LW_FLAG_DEAD);
        LinkModule::send_event_nodes(boma, *LINK_NO_CAPTURE, Hash40::new_raw(0xa5fbe21f9), 0);
    }
    CameraModule::zoom_out(boma, 0);
    if lua_bind::FighterCutInManager::is_play(singletons::FighterCutInManager()) {
        lua_bind::FighterCutInManager::request_end(singletons::FighterCutInManager());
    }
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE, demon_attack_rage_pre_status)
    .status(End, *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE, demon_attack_rage_end_status)
    .install()
    ;
}