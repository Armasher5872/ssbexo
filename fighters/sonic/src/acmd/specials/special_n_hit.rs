use super::*;

//Homing Attack Hit ACMD
unsafe extern "C" fn ssbexo_sonic_homing_attack_hit_acmd(agent: &mut L2CAgentBase) {
    let vector = Vector3f{x: -0.1, y: 1.5, z: 0.0};
    if is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &vector);
    }
    FT_MOTION_RATE(agent, 0.6);
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnhit", ssbexo_sonic_homing_attack_hit_acmd, Low)
    .install()
    ;
}