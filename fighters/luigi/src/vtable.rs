use super::*;

const LUIGI_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xca0cf0; //Luigi only
const LUIGI_VTABLE_ON_ATTACK_OFFSET: usize = 0xca13a0; //Luigi only
const LUIGI_VTABLE_LINK_EVENT_OFFSET: usize = 0xca0e70; //Luigi only
const LUIGI_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0xca2960; //Luigi only
const LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET: usize = 0xca1510; //Luigi only

//Luigi Reset Initialization
unsafe extern "C" fn luigi_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    luigi_var(&mut *boma);
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
        if capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED && status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER {
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request(boma, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, false);
        }
        if capture_event.status == *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START && status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP {
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.constraint = true;
            StatusModule::change_status_request(boma, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_CATCH_PULL, false);
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
    let collision_log = collision_log as *mut CollisionLogScuffed;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let status_kind = StatusModule::status_kind(boma);
    if status_kind == *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_PLUNGER {
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
            if opponent_battle_object_id >> 0x1C == 0 {
                WorkModule::on_flag(boma, 0x200000e6 /*FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_CATCH_SEARCH*/);
            }
        }
    }
}

//Fixes issues regarding Grab
#[skyline::hook(offset = LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET)]
unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {}

//Luigi Fireball On Attack Offset
unsafe extern "C" fn luigi_fireball_on_attack(vtable: u64, weapon: *mut smash::app::Weapon, collision_bitmask: u32) -> u64 {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if owner_kind == *FIGHTER_KIND_LUIGI {
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_GANON {
        *(weapon as *mut bool).add(0x90) = false;
    }
    if owner_kind == *FIGHTER_KIND_WARIO {
        *(weapon as *mut bool).add(0x90) = true;
    }
    normal_weapon_hit_handler(vtable, weapon, collision_bitmask)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fe1158).data(luigi_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x51e1898).data(luigi_fireball_on_attack as *const () as u64);
	skyline::install_hooks!(
        luigi_death_initialization,
        luigi_on_attack,
        luigi_link_event,
        luigi_on_search,
        luigi_change_motion_callback
    );
}