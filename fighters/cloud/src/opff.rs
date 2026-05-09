use super::*;

unsafe extern "C" fn cloud_check_ground_guard_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) && fighter.sub_check_command_guard().get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_GUARD_ON.into(), true.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_ON.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}

unsafe extern "C" fn cloud_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

unsafe extern "C" fn cloud_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let shield_data = ShieldDataResource::new(0.0, 9.5, 3.0, 0.0, 9.5, 3.0, 13.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    add_shield_group(boma, resource, *FIGHTER_CLOUD_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    common_initialization_variable_reset(&mut *boma);
    cloud_var(&mut *boma);
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(cloud_end_control as *const () as _));
    fighter.global_table[CHECK_GROUND_GUARD_UNIQ].assign(&L2CValue::Ptr(cloud_check_ground_guard_uniq as *const () as _));
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(cloud_on_start)
    .install()
    ;
}