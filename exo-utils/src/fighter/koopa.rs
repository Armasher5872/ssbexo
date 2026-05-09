use super::*;

pub unsafe extern "C" fn koopa_var(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
}