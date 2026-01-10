//Credit to: Liam Estares (LKE Studios), Ben Hall (arthur!), and WuBoytH
use super::*;

unsafe extern "C" fn status_glide_end_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);
    lua_bind::KineticEnergy::reset_energy(stop, *ENERGY_STOP_RESET_TYPE_GLIDE_START, &Vector2f::zero(), &Vector3f::zero(), fighter.module_accessor);
    lua_bind::KineticEnergy::enable(stop);
    lua_bind::KineticEnergy::reset_energy(motion, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: 0.0*lr, y: 0.0}, &Vector3f::zero(), fighter.module_accessor);
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_GLIDE_END, status_glide_end_init)
    .install()
    ;
}