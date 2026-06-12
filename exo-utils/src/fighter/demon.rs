use super::*;

pub unsafe extern "C" fn demon_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_HIGH_POUNCE_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_MIST_STEP_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_SOBAT_TURN);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ELECTRIC_DRAGON_UPPERCUT);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_RAGE_DRIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TO_HEAVENS_DOOR);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_THROW);
    WorkModule::set_int(boma, 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
}

pub unsafe extern "C" fn fun_710002aed0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.sub_GetLightItemImm(L2CValue::Void());
    (StatusModule::status_kind_que_from_script(boma) as i32 != *STATUS_KIND_NONE).into()
}