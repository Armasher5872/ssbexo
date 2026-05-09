use super::*;

pub unsafe extern "C" fn ken_var(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KEN_INSTANCE_WORK_ID_FLAG_CAN_KARA_CANCEL);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KEN_INSTANCE_WORK_ID_INT_ATTACK_COMMAND1_COUNTER);
}