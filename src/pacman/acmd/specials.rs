use super::*;

//Up Special Start ACMD
unsafe extern "C" fn ssbexo_pacman_up_special_start_acmd(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 0.65);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, -1);
        }
    }
}

pub fn install() {
    Agent::new("pacman")
    .game_acmd("game_specialhistart", ssbexo_pacman_up_special_start_acmd, Priority::Low)
    .game_acmd("game_specialairhistart", ssbexo_pacman_up_special_start_acmd, Priority::Low)
    .install()
    ;
}