use super::*;

unsafe extern "C" fn edge_on_start(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

unsafe extern "C" fn edge_flaredummy_frame(weapon: &mut L2CFighterBase) {
    let owner_boma = get_owner_boma(weapon);
    let owner_status_kind = StatusModule::status_kind(owner_boma);
    let owner_prev_status_kind = StatusModule::prev_status_kind(owner_boma, 0);
    if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_FLY {
        if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            if owner_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL
            && owner_prev_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE {
                weapon.change_status(WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_TRY.into(), false.into());
            }
        }
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(edge_on_start)
    .install()
    ;
    Agent::new("edge_flaredummy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_line(Main, edge_flaredummy_frame)
    .install()
    ;
}