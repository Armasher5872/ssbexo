use super::*;

unsafe extern "C" fn rockman_attack_air_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn rockman_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    let mut motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let mini_jump_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_ROCKBUSTER_COUNT_MINI_JUMP_ATTACK);
    if 0 < mini_jump_attack {
        motion_kind = hash40("attack_air_n");
    }
    if motion_kind == hash40("attack_air_n") {
        fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR.into(), false.into());
    }
    else {
        if [hash40("attack_air_hi"), hash40("attack_air_lw")].contains(&motion_kind) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92c83), 1, *FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(rockman_attack_air_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn rockman_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackAir_Main()
}

unsafe extern "C" fn rockman_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    fighter.status_end_AttackAir();
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion_kind == hash40("attack_air_n") {
        if !rockman_rockbuster_pre_helper(status_kind.into()).get_bool() {
            rockman_rockbuster_end_var_reset(fighter);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, rockman_attack_air_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, rockman_attack_air_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, rockman_attack_air_end_status)
    .install()
    ;
}