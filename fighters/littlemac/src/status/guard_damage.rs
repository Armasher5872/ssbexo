use super::*;

unsafe extern "C" fn littlemac_guard_damage_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        //Adds a third of the meter if Little Mac parries
        WorkModule::add_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus()
}

pub fn install() {
    Agent::new("littlemac")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, littlemac_guard_damage_init_status)
    .install()
    ;
}