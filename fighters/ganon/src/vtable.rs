//Credited to WuBoyTH
use super::*;

const GANON_ARMSTRONG_SPRINGTRAP_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaa6520; //Ganondorf only
const GANON_ARMSTRONG_SPRINGTRAP_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540; //Ganondorf only
const GANON_ARMSTRONG_SPRINGTRAP_VTABLE_STATUS_TRANSITION_OFFSET: usize = 0xaa6800; //Ganondorf only
const GANON_ARMSTRONG_SPRINGTRAP_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Armstrong only

//Ganondorf & Armstrong & Springtrap Reset Initialization
unsafe extern "C" fn ganon_armstrong_springtrap_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    if is_armstrong_slots(boma) {
        armstrong_var(&mut *boma);
    }
    else if is_springtrap_slots(boma) {
        springtrap_var(boma, false);
    }
    else {
        ganon_var(&mut *boma);
    }
    common_reset_variable_reset(&mut *boma);
}

//Ganondorf & Armstrong & Springtrap Death Initialization
#[skyline::hook(offset = GANON_ARMSTRONG_SPRINGTRAP_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ganon_armstrong_springtrap_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    if is_armstrong_slots(boma) {
        armstrong_var(&mut *boma);
    }
    else if is_springtrap_slots(boma) {
        springtrap_var(boma, true);
    }
    else {
        ganon_var(&mut *boma);
    }
    common_death_variable_reset(&mut *boma);
}

//Ganondorf & Armstrong & Springtrap Once Per Fighter Frame
unsafe extern "C" fn ganon_armstrong_springtrap_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    if !is_armstrong_slots(boma) && !is_springtrap_slots(boma) {
        if !ArticleModule::is_exist(boma, FIGHTER_GANON_GENERATE_ARTICLE_VOLLEY) {
            WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_HAS_ACTIVE_VOLLEY);
        }
    }
}

//Ganondorf & Armstrong & Springtrap On Attack
#[skyline::hook(offset = GANON_ARMSTRONG_SPRINGTRAP_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn ganon_armstrong_springtrap_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_object).battle_object_id;
        let opponent_boma = (*opponent_object).module_accessor;
        let attack_data = *AttackModule::attack_data(boma, (*collision_log).collider_id as i32, (*collision_log).x35);
        let lr = PostureModule::lr(boma);
        let status_kind = StatusModule::status_kind(boma);
        let sound_attr = attack_data.sound_attr as i32;
        let sound_level = attack_data.sound_level as i32;
        if is_armstrong_slots(boma) {
            let charge = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_NEUTRAL_SPECIAL_CHARGE);
            if status_kind == *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_N_ATTACK && charge > 0.75 {
                call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
            }
            if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
            }
            if collision_kind == 1 {
                if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status(opponent_boma, (*collision_log).receiver_id as i32, 0) == 0 {
                    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                        let opponent_situation_kind = StatusModule::situation_kind(opponent_boma);
                        if opponent_situation_kind == *SITUATION_KIND_GROUND {
                            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
                        }
                    }
                }
            }
        }
        else if is_springtrap_slots(boma) {
            if opponent_battle_object_id >> 0x1C == 0 {
                let opponent_lr = PostureModule::lr(opponent_boma);
                let opponent_pos = *PostureModule::pos(opponent_boma);
                if opponent_lr == lr {
                    EffectModule::req(opponent_boma, Hash40::new("springtrap_soul_burst"), &Vector3f{x: opponent_pos.x, y: opponent_pos.y+12.0, z: opponent_pos.z}, &Vector3f{x: 90.0, y: 90.0, z: 0.0}, 1.0, 0, -1, false, 0);
                }
                if status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_S_ATTACK {
                    if collision_kind != *COLLISION_KIND_SHIELD as u8 && attack_data.attr == hash40("collision_attr_saving") && WorkModule::get_float(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CHARGE) >= 1.0 {
                        WorkModule::on_flag(boma, *FIGHTER_SPRINGTRAP_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CRIT);
                    }
                }
            }
            if sound_attr == *COLLISION_SOUND_ATTR_SPRINGTRAP_KNIFE {
                let volume = match sound_level {
                    0 => {0.3},
                    1 => {0.5},
                    2 => {0.7},
                    3 => {1.0},
                    _ => {1.0}
                };
                let crit = SoundModule::play_se(boma, Hash40::new("se_ganon_attackhard_h03"), true, false, false, false, enSEType(0));
                SoundModule::set_se_vol(boma, crit as i32, volume, 0);
            }
        }
        else {
            if status_kind == *FIGHTER_GANON_STATUS_KIND_APPEAL_ATTACK {
                call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Ganondorf & Armstrong & Springtrap Status Transition
#[skyline::hook(offset = GANON_ARMSTRONG_SPRINGTRAP_VTABLE_STATUS_TRANSITION_OFFSET)]
unsafe extern "C" fn ganon_armstrong_springtrap_status_transition(_vtable: u64, fighter: &mut Fighter) {
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

//Ganondorf & Armstrong & Springtrap Link Event
#[skyline::hook(offset = GANON_ARMSTRONG_SPRINGTRAP_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn ganon_armstrong_springtrap_link_event(_vtable: u64, fighter: &mut Fighter, log: *mut u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(boma);
    let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
    let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
    let event: &mut LinkEvent = std::mem::transmute(log);
    let event_kind = event.link_event_kind.0;
    if is_armstrong_slots(boma) {
        if event_kind == hash40("capture") {
            let capture_event: &mut LinkEventCapture = std::mem::transmute(event);
            let captured_status = capture_event.status;
            if status == *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN {
                if captured_status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
                if captured_status == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
            }
            if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                capture_event.motion_offset = offset;
                capture_event.motion_offset_lw = offset_lw;
                StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            }
            if status == *FIGHTER_STATUS_KIND_FINAL && captured_status == *FIGHTER_STATUS_KIND_THROWN {
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                capture_event.motion_offset = offset;
                capture_event.motion_offset_lw = offset_lw;
                StatusModule::change_status_request(boma, *FIGHTER_ARMSTRONG_STATUS_KIND_FINAL_THROW, false);
                return 0;
            }
        }
    }
    else {
        if event_kind == hash40("capture") {
            let capture_event: &mut LinkEventCapture = std::mem::transmute(event);
            let object_id = capture_event.sender_id;
            if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
                if LinkModule::is_link(boma, 0) {
                    capture_event.result = false;
                    return 0;
                }
                StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
                capture_event.result = true;
                capture_event.constraint = false;
                CatchModule::set_catch(boma, object_id);
                if !LinkModule::is_link(boma, 0) {
                    return 0;
                }
                let ptr = get_module_vtable_func(boma, 0x130, 0x80);
                let func: extern "C" fn(catch_module: *mut u64) = std::mem::transmute(ptr);
                let catch_module = (boma as *mut u64).add(0x130/0x8);
                func(catch_module);
                let mut offset = (0.0, 0.0);
                if object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                    let object = get_battle_object_from_id(object_id);
                    let vtable = *(object as *const u64) as *const u64;
                    let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
                    if !func(object) {
                        if (*object).battle_object_id >> 0x1c == 0 {
                            offset.0 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                            offset.1 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                        }
                    }
                }
                LinkModule::set_model_constraint_flag(boma, 0x2003);
                LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: offset.0, y: offset.1, z: 0.0});
                return 0;
            }
            if status == *FIGHTER_STATUS_KIND_SPECIAL_S {
                if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
                if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                    StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                    capture_event.result = true;
                    capture_event.node = smash2::phx::Hash40::new("throw");
                    return 0;
                }
            }
        }
        else if event_kind == 0xa84e26287 {
            if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
                CatchModule::set_send_cut_event(boma, false);
                CatchModule::cling_cut(boma, false);
                if *(log as *const u8).offset(0x29) != 0 {
                    return 0;
                }
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
                return 0;
            }
        }
        else if event_kind == 0xdac7c579e {
            if status == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK {
                let object_id = event.sender_id;
                let object = get_battle_object_from_id(object_id);
                let vtable = *(object as *const u64) as *const u64;
                let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
                if !func(object)
                && (*object).battle_object_id >> 0x1c == 1
                && WorkModule::get_int(boma, *FIGHTER_GANON_STATUS_WORK_ID_INT_BEAST_BEAST_TASK_ID) as u32 == (*object).battle_object_id {
                    WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_BEAST_END);
                    return 1;
                }
            }
        }
    }
    1
}

//Ganondorf & Armstrong & Springtrap On Search Event
unsafe extern "C" fn ganon_armstrong_springtrap_on_search(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8) as *const CollisionLogScuffed;
    let status_kind = StatusModule::status_kind(boma);
    if is_springtrap_slots(boma) {
        if status_kind == *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_LOOP {
            let opponent_id = (*collision_log).opponent_object_id;
            let opponent_battle_object = get_battle_object_from_id(opponent_id);
            let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
            let opponent_boma = (*opponent_battle_object).module_accessor;
            let opponent_kind = utility::get_kind(&mut *opponent_boma);
            if opponent_battle_object_id >> 0x1C == 1 {
                if opponent_kind == *WEAPON_KIND_KROOL_IRONBALL {
                    let owner_id = WorkModule::get_int(opponent_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
                    if owner_id == fighter.battle_object.battle_object_id {
                        WorkModule::set_int(opponent_boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_SPRINGTRAP_STATUS_KIND_SPECIAL_N_RECALL_END, false);
                    }
                }
            }
        }
    }
}

//Ganon & Armstrong & Springtrap On Damage
unsafe extern "C" fn ganon_armstrong_springtrap_on_damage(_vtable: u64, fighter: &mut Fighter, _on_damage: u64) {
    let boma = fighter.battle_object.module_accessor;
    if is_armstrong_slots(boma) {
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let status_kind = StatusModule::status_kind(boma);
        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let situation_kind = StatusModule::situation_kind(boma);
        let statuses = [
            *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ARMSTRONG_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_STATUS_KIND_SPECIAL_LW
        ];
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && (1.0..=79.0).contains(&frame) {
            HitModule::set_check_catch(boma, true, 0);
            DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
            MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
        }
        if statuses.contains(&status_kind) || statuses.contains(&prev_status_kind) {
            PLAY_SE(agent, Hash40::new("se_common_metal_step_l"));
            DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW && situation_kind == *SITUATION_KIND_GROUND && WorkModule::is_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_COUNTER_ACTIVE) {
                WorkModule::on_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
            }
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fbb308).data(ganon_armstrong_springtrap_reset_initialization as *const () as *const u64);
    let _ = skyline::patching::Patch::in_text(0x4fbb350).data(ganon_armstrong_springtrap_opff as *const () as *const u64);
    let _ = skyline::patching::Patch::in_text(0xaa6618).nop(); //Nops the original location where Neutral Special inflicts critical zoom, as I want both Ganon and Armstrong to have different places where they inflict critical zoom
    let _ = skyline::patching::Patch::in_text(0x4fbb468).data(ganon_armstrong_springtrap_on_search as *const () as *const u64);
    let _ = skyline::patching::Patch::in_text(0x4fbb508).data(ganon_armstrong_springtrap_on_damage as *const () as *const u64);
	skyline::install_hooks!(
        ganon_armstrong_springtrap_death_initialization,
        ganon_armstrong_springtrap_on_attack,
        ganon_armstrong_springtrap_status_transition,
        ganon_armstrong_springtrap_link_event
    );
}