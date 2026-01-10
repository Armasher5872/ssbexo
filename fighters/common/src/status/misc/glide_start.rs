//Credit to: Liam Estares (LKE Studios), Ben Hall (arthur!), and WuBoytH
use super::*;

unsafe extern "C" fn status_init_glidestart(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get();
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    lua_bind::KineticEnergy::reset_energy(gravity, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: -params.gravity_start}, &Vector3f{x: 0.0, y: -params.gravity_start, z: 0.0}, fighter.module_accessor);
    lua_bind::KineticEnergy::reset_energy(motion, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: params.speed_mul_start * lr, y: 0.0}, &Vector3f{x: params.speed_mul_start * lr, y: 0.0, z: 0.0}, fighter.module_accessor);
    lua_bind::KineticEnergy::reset_energy(stop, *ENERGY_STOP_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: params.v_glide_start}, &Vector3f{x: 0.0, y: params.v_glide_start, z: 0.0}, fighter.module_accessor);
    lua_bind::KineticEnergy::enable(stop);
    0.into()
}

pub fn install() {
    Agent::new("fighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_GLIDE_START, status_init_glidestart)
    .install()
    ;
}