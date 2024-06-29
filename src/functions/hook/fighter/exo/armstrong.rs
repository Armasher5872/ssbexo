use super::*;

const ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaa6510; //Armstrong only
const ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Armstrong only
const ARMSTRONG_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Armstrong only

//Armstrong Startup Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
    WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
    WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
    WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
}

//Armstrong Reset Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
        COUNTERHIT_CHECK[entry_id] = false;
        COUNTERHIT_SUCCESS[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        FIGHTER_BOOL_1[entry_id] = false;
        FIGHTER_BOOL_2[entry_id] = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
        WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        SPECIAL_WALL_JUMP = false;
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
        WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
        WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
        LAST_DAMAGE[entry_id] = 0.0;
        SIZE0[entry_id] = 0.0;
        SIZE1[entry_id] = 0.0;
        SIZE2[entry_id] = 0.0;
        SIZE3[entry_id] = 0.0;
        FULL_HOP_ENABLE_DELAY[entry_id] = 0;
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
        WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
        WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
        WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
        WorkModule::set_float(boma, 1.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    }
}

//Armstrong On Damage
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn armstrong_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let status_kind = fighter.battle_object.status();
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
    && (1.0..=79.0).contains(&frame) {
        HitModule::set_check_catch(boma, true, 0);
        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
    }
}

//Armstrong Link Event
#[skyline::hook(offset = ARMSTRONG_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn armstrong_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let ret = call_original!(vtable, fighter, event);
    let boma = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(boma);
    /*
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
            if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
                capture_event.result = false;
                return 0;
            }
            fighter.battle_object.change_status_req(*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            capture_event.result = true;
            capture_event.constraint = false;
            CatchModule::set_catch(boma, capture_event.sender_id);
            if !LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
                return 0;
            }
            /*
            let catch_module_u64 = *(boma as *const u64).offset(0x120/8) as *const u64;
            let func: fn(*mut smash2::app::Module) = std::mem::transmute(catch_module_u64.offset(0x80/8));
            let catch_module = *(boma as *const u64).offset(0x120 / 8) as *mut smash2::app::Module;
            func(catch_module);
            */
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            if capture_event.sender_id != 0x50000000 {
                let object = crate::functions::hook::misc::get_battle_object_from_id(capture_event.sender_id);
                if !object.is_null() {
                    if (*object).battle_object_id >> 0x1c == 0 {
                        offset_x = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                        offset_y = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                    }
                }
            }
            LinkModule::set_model_constraint_flag(boma, 0x2003);
            LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: offset_x, y: offset_y, z: 0.0});
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
        if status == *FIGHTER_STATUS_KIND_FINAL && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
        return 1;
    }
    else if event.link_event_kind.0 == 0xa84e26287 {
        if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
            CatchModule::set_send_cut_event(boma, false);
            CatchModule::cling_cut(boma, false);
            if *(event.link_event_kind.0 as *const u8).offset(0x29) != 0 {
                return 0;
            }
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
        }
    }
    1
    */
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if status == *FIGHTER_STATUS_KIND_FINAL && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
        return 1;
    }
    ret
}

pub fn install() {
	skyline::install_hooks!(
        armstrong_start_initialization,
        armstrong_reset_initialization,
        armstrong_on_damage,
        armstrong_link_event
    );
}