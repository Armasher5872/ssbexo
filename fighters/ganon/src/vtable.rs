//Credited to WuBoyTH
use super::*;

const GANON_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800;

//Ganondorf Status Transition
#[skyline::hook(offset = GANON_VTABLE_STATUS_TRANSITION_OFFSET)]
unsafe extern "C" fn ganon_status_transition(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(boma, 0) as u64;
    let status = StatusModule::status_kind(boma) as u64;
    if prev_status < 0x37 /*catch*/ {
        if 1 << (prev_status & 0x3f /*catch_jump*/) & 0xe00000000000u64 != 0 && status & 0xfffffffe != 0x2e /*attack_s4_hold*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if 1 << (prev_status & 0x3f /*catch_jump*/) & 0x7000000000000u64 != 0 && 1 < status - 0x31 /*attack_lw4_hold*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if 1 << (prev_status & 0x3f /*catch_jump*/) & 0x38000000000000u64 != 0 && status & 0xfffffffe != 0x34 /*attack_hi4_hold*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if prev_status == 0x27 /*attack*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        if prev_status == 0x36 /*attack_air*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    if prev_status == 0x24 /*rebound_stop*/ {
        if status != 0x25 /*rebound*/ {
            ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    else if prev_status == 0x25 /*rebound*/ {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    else if prev_status == 0x18 /*landing_attack_air*/ {
        ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    else if [0x27 /*attack*/, 0x2f /*attack_s4*/, 0x32 /*attack_lw4*/, 0x35 /*attack_hi4*/].contains(&prev_status) {
        if status == 0x24 /*rebound_stop*/ {
            ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
        }
    }
}

pub fn install() {
	skyline::install_hooks!(ganon_status_transition);
}