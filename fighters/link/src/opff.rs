use super::*;

unsafe extern "C" fn link_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_AIR || is_damaged(boma) {
        WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_GAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::set_int(boma, 300, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    }
    0.into()
}

unsafe extern "C" fn link_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    link_var(&mut *boma);
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(link_end_control as *const () as _));
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(link_on_start)
    .install()
    ;
}