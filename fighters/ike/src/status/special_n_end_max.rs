use super::*;

unsafe extern "C" fn ike_special_lw_end_max_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
            let opponent_boma = sv_battle_object::module_accessor(object_id);
            if DamageModule::damage(opponent_boma, 0) >= 150.0 && fighter.global_table[CURRENT_FRAME].get_f32() < 11.0 {
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DEAD, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ike")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END_MAX, ike_special_lw_end_max_check_attack_status)
    .install()
    ;
}