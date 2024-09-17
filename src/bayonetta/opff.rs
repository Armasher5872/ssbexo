use super::*;

unsafe extern "C" fn bayonetta_frame(fighter: &mut L2CFighterCommon) {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_SMASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_U_SMASH, FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_D_SMASH].contains(&status_kind)
    && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    }
}

pub fn install() {
    Agent::new("bayonetta")
    .on_line(Main, bayonetta_frame)
    .install()
    ;
}