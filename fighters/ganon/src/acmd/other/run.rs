use super::*;

//Run Sound
unsafe extern "C" fn ssbexo_ganon_run_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("sound_run", ssbexo_ganon_run_sound, Low)
    .install()
    ;
}