use super::*;

unsafe extern "C" fn lucas_attack_s4_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                EFFECT(fighter, Hash40::new("starman_smash"), Hash40::new("top"), 0, 16, -10, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, true);
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_lucas_smash"), false, false, false, false, enSEType(0));
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("lucas")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S4, lucas_attack_s4_check_attack_status)
    .install()
    ;
}