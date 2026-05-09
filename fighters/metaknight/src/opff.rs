use super::*;

unsafe extern "C" fn metaknight_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let shield_data = ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, Hash40::new("hip"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    common_initialization_variable_reset(&mut *boma);
    add_shield_group(boma, resource, *FIGHTER_METAKNIGHT_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    metaknight_var(&mut *boma);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(metaknight_on_start)
    .install()
    ;
}