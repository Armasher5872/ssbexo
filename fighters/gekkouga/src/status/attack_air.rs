use super::*;

unsafe extern "C" fn gekkouga_attack_air_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW || motion_kind == hash40("attack_air_lw") {
                let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
                let opponent_boma = sv_battle_object::module_accessor(object_id);
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);  
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, gekkouga_attack_air_check_attack_status)
    .install()
    ;
}