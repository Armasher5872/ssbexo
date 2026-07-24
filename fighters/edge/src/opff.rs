use super::*;

unsafe extern "C" fn edge_flaredummy_on_start(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    WorkModule::off_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED);
}

unsafe extern "C" fn edge_flaredummy_frame(weapon: &mut L2CFighterBase) {
    let boma = weapon.module_accessor;
    if StatusModule::status_kind(boma) == *WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_FLY {
        if WorkModule::is_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED) {
            weapon.change_status(WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_TRY.into(), false.into());
        }
    }
}

unsafe extern "C" fn edge_flash_on_start(weapon: &mut L2CWeaponCommon) {
    let boma = weapon.module_accessor;
    WorkModule::set_float(boma, 0.0, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_FLOAT_OPPONENT_POS_X);
    WorkModule::set_float(boma, 0.0, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_FLOAT_OPPONENT_POS_Y);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_INT_OPPONENT_OBJECT_ID);
}

pub fn install() {
    Agent::new("edge_flaredummy")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(edge_flaredummy_on_start)
    .on_line(Main, edge_flaredummy_frame)
    .install()
    ;
    Agent::new("edge_flash")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .on_start(edge_flash_on_start)
    .install()
    ;
}