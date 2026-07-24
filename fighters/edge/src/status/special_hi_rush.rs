use super::*;

unsafe extern "C" fn edge_special_hi_rush_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    original_status(Exec, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)(fighter)
}

unsafe extern "C" fn edge_special_hi_rush_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL);
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
                CancelModule::enable_cancel(boma);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn edge_special_hi_rush_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
        KineticModule::mul_speed(boma, &Vector3f{x: 0.1, y: 0.1, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_BLADE_DASH_EARLY_CANCEL);
    }
    original_status(End, fighter, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH)(fighter)
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Exec, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, edge_special_hi_rush_exec_status)
    .status(CheckAttack, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, edge_special_hi_rush_check_attack_status)
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, edge_special_hi_rush_end_status)
    .install()
    ;
}