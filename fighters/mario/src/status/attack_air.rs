use super::*;

//Aerial Check Attack Status
unsafe extern "C" fn mario_attack_air_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW
            || motion_kind == hash40("attack_air_lw") {
                if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
                    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                        MotionModule::change_motion(boma, Hash40::new("attack_air_lw_bounce"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.6);
                    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, mario_attack_air_check_attack_status)
    .install()
    ;
}