use super::*;

const LUIGI_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xca0ce0; //Luigi only
const LUIGI_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const LUIGI_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xca0cf0; //Luigi only
const LUIGI_VTABLE_ON_ATTACK_OFFSET: usize = 0xca1380; //Luigi only
const LUIGI_VTABLE_LINK_EVENT_OFFSET: usize = 0xca0e70; //Luigi only
const LUIGI_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0xca2960; //Luigi only
const LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET: usize = 0xca1510; //Luigi only

unsafe extern "C" fn luigi_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_BATABATA);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ATTACK_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_THROW);
    WorkModule::off_flag(boma, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_PLUNGER_THROW);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_N_HELD_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
}

unsafe extern "C" fn luigi_check_jump_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let is_aerial = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let allow_float = ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) && !is_aerial && stick_y <= squat_stick_y;
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) 
    || WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_BATABATA) {
            if allow_float {
                fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_BATABATA.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn luigi_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_BATABATA);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

//Luigi Startup Initialization
#[skyline::hook(offset = LUIGI_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    luigi_var(&mut *boma);
    agent.global_table[CHECK_AIR_JUMP_UNIQ].assign(&L2CValue::Ptr(luigi_check_jump_uniq as *const () as _));
    agent.global_table[CHECK_AIR_JUMP_AERIAL_UNIQ].assign(&L2CValue::Ptr(luigi_check_jump_uniq as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(luigi_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Luigi Reset Initialization
#[skyline::hook(offset = LUIGI_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUIGI as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        luigi_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Luigi Death Initialization
#[skyline::hook(offset = LUIGI_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    luigi_var(&mut *boma);
    original!()(vtable, fighter)
}

//Luigi On Attack
#[skyline::hook(offset = LUIGI_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn luigi_on_attack(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && WorkModule::is_flag(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT) {
        call_special_zoom(boma, log, *FIGHTER_KIND_LUIGI, hash40("param_special_hi"), 1, 0, 0, 0, 0);
    }
}

//Luigi Link Event
#[skyline::hook(offset = LUIGI_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
    let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        if capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            if [*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER].contains(&status_kind) {
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                capture_event.motion_offset = offset;
                capture_event.motion_offset_lw = offset_lw;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, false);
            }
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Luigi On Search
#[skyline::hook(offset = LUIGI_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn luigi_on_search(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let opponent_battle_object_id = (*collision_log).opponent_battle_object_id;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER {
        if opponent_battle_object_id >> 0x1C == 0 {
            WorkModule::on_flag(boma, 0x200000e6 /*FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_CATCH_SEARCH*/);
        }
    }
}

//Fixes issues regarding Grab
#[skyline::hook(offset = LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET)]
unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {}

pub fn install() {
	skyline::install_hooks!(
        luigi_start_initialization,
        luigi_reset_initialization,
        luigi_death_initialization,
        luigi_on_attack,
        luigi_link_event,
        luigi_on_search,
        luigi_change_motion_callback
    );
}