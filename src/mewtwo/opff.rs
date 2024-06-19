use super::*;

unsafe extern "C" fn mewtwo_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn mewtwo_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(mewtwo_end_control as *const () as _));
}

pub fn install() {
    Agent::new("mewtwo")
    .on_start(mewtwo_init)
    .install()
    ;
}