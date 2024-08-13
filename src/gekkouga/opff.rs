use super::*;

unsafe extern "C" fn gekkouga_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(gekkouga_end_control as *const () as _));
}

pub fn install() {
    Agent::new("gekkouga")
    .on_start(gekkouga_init)
    .install()
    ;
}