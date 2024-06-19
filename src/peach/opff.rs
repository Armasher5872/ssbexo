use super::*;

unsafe extern "C" fn peach_check_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn peach_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&false.into());
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(peach_check_special_lw_uniq as *const () as _));
}

pub fn install() {
    Agent::new("peach")
    .on_start(peach_init)
    .install()
    ;
}