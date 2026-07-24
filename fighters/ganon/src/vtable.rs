//Credited to WuBoyTH
use super::*;

const GANON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d680; //Shared
const GANON_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800;

//Ganondorf Once Per Fighter Frame
#[skyline::hook(offset = GANON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ganon_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        if !is_armstrong_slots(boma) && !is_springtrap_slots(boma) {
            if !ArticleModule::is_exist(boma, FIGHTER_GANON_GENERATE_ARTICLE_VOLLEY) {
                WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_HAS_ACTIVE_VOLLEY);
            }
        }
    }
    original!()(vtable, fighter)
}

//Ganondorf Status Transition
#[skyline::hook(offset = GANON_VTABLE_STATUS_TRANSITION_OFFSET)]
unsafe extern "C" fn ganon_status_transition(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let prev_status = StatusModule::prev_status_kind(boma, 0) as u64;
    let status = StatusModule::status_kind(boma) as u64;
    if !is_springtrap_slots(boma) {
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
    else {
        let active_axe = WorkModule::is_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_ACTIVE_AXE);
        let status_kind = StatusModule::status_kind(boma);
        let active_axe_statuses = [
            *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_CHARGE_LOOP, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_LOW_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_HIGH_FIRE, 
            *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, *FIGHTER_STATUS_KIND_WIN
        ];
        let unactive_axe_statuses = [
            *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_CHARGE_LOOP, 
            *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_LOW_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_HIGH_FIRE, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, *FIGHTER_STATUS_KIND_WIN
        ];
        if active_axe {
            if active_axe_statuses.contains(&status_kind) {
                ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
            }
            else {
                ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        else {
            if unactive_axe_statuses.contains(&status_kind) {
                ArticleModule::generate_article_enable(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, -1);
            }
            else {
                ArticleModule::remove_exist(boma, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
    }
}

pub fn install() {
	skyline::install_hooks!(
        ganon_opff,
        ganon_status_transition
    );
}