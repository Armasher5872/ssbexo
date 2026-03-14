//Credited to WuBoyTH
use super::*;

const GANON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaa6510; //Ganon only
const GANON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d680; //Shared
const GANON_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800;

unsafe extern "C" fn ganon_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_USED_SPECIAL_N_AIR);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

//Ganon Startup Initialization
#[skyline::hook(offset = GANON_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ganon_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    ganon_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(ganon_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Ganondorf Once Per Fighter Frame
#[skyline::hook(offset = GANON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ganon_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let frame = MotionModule::frame(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        if !is_armstrong_slots(boma) {
            if !ArticleModule::is_exist(boma, FIGHTER_GANON_GENERATE_ARTICLE_VOLLEY) {
                WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_HAS_ACTIVE_VOLLEY);
            }
            if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
                if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind) && (17.0..=85.0).contains(&frame) {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_GANON_STATUS_KIND_APPEAL_ATTACK, false);
                    }
                }
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
	skyline::install_hooks!(
        ganon_start_initialization,
        ganon_opff,
        ganon_status_transition
    );
}