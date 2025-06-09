use super::*;

unsafe extern "C" fn common_cliff_robbed_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.05);
    //sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.08);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, common_cliff_robbed_init_status)
    .install()
    ;
}