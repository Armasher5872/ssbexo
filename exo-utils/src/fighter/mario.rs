use super::*;

pub unsafe extern "C" fn mario_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT);
    WorkModule::off_flag(boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
    WorkModule::set_float(boma, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    WorkModule::set_float(boma, 0.0, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_DEGREE);
    WorkModule::set_int(boma, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_FRAME);
}