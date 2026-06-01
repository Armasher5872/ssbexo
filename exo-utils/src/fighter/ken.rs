use super::*;

pub unsafe extern "C" fn ken_var(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_CAN_KARA_CANCEL);
    WorkModule::set_int(boma, 0, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
}