use super::*;

unsafe extern "C" fn rockman_ladder_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LadderAttack_common();
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion_kind == hash40("attack_air_n") {
        fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_ROCKBUSTER_SHOOT_AIR.into(), false.into());
    }
    else {
        if [hash40("attack_air_hi"), hash40("attack_air_lw")].contains(&motion_kind) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92c83), 1, *FIGHTER_LOG_DATA_INT_SHOOT_NUM);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(rockman_ladder_attack_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn rockman_ladder_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LadderAttack_Main()
}

unsafe extern "C" fn rockman_ladder_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    fighter.status_end_LadderAttack();
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion_kind == hash40("attack_air_n") {
        if !rockman_rockbuster_pre_helper(status_kind.into()).get_bool() {
            rockman_rockbuster_end_var_reset(fighter);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCK_BUSTER_VISUAL);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("rockman")
    .status(Main, *FIGHTER_STATUS_KIND_LADDER_ATTACK, rockman_ladder_attack_main_status)
    .status(End, *FIGHTER_STATUS_KIND_LADDER_ATTACK, rockman_ladder_attack_end_status)
    .install()
    ;
}