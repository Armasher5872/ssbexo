use super::*;

//Aerial Check Attack Status
unsafe extern "C" fn sheik_attack_air_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let speed_x = get_sum_speed_x*lr;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW
            || motion_kind == hash40("attack_air_lw") {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
                    SET_SPEED_EX(fighter, speed_x, 2.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("sheik")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, sheik_attack_air_check_attack_status)
    .install()
    ;
}