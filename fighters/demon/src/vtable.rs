use super::*;

const DEMON_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x930d60; //Kazuya only
const DEMON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x930ff0; //Kazuya only
const DEMON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x931680; //Kazuya only
const DEMON_VTABLE_ON_ATTACK_OFFSET: usize = 0x932f50; //Kazuya only
const DEMON_VTABLE_LINK_EVENT_OFFSET: usize = 0x933800; //Kazuya only
const DEMON_VTABLE_ON_GRAB_OFFSET: usize = 0x934310; //Kazuya only

unsafe extern "C" fn demon_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ONE_TWO_PUNCH);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEMON_SLAYER);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_STATURE_SMASH);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_TWIN_FANG_DOUBLE_KICK);
}

//Kazuya Startup Initialization
#[skyline::hook(offset = DEMON_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn demon_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Kazuya Reset Initialization
#[skyline::hook(offset = DEMON_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn demon_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kazuya Death Initialization
#[skyline::hook(offset = DEMON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn demon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kazuya On Attack
#[skyline::hook(offset = DEMON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn demon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if motion_kind == hash40("attack_stand_23") && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_REBOUND, false);
    }
    if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_IS_VALID_RAGE_DRIVE);
    }
    call_original!(vtable, fighter, log)
}

//Kazuya Link Event
#[skyline::hook(offset = DEMON_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn demon_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
        WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA);
        WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_THROW_MOTION_CAMERA);
        (1 as u64).into()
    } 
    else {
        original!()(vtable, fighter, event)
    }
}

//Kazuya On Grab
#[skyline::hook(offset = DEMON_VTABLE_ON_GRAB_OFFSET)]
unsafe extern "C" fn demon_on_grab(_vtable: u64, _fighter: &mut Fighter, catch_status: i32) -> i32 {
    return catch_status
}

pub fn install() {
    //Removes the call_script_single that creates the EWGF Unblockable Windbox
    let _ = skyline::patching::Patch::in_text(0x933454).nop();
    skyline::install_hooks!(
        demon_start_initialization,
        demon_reset_initialization,
        demon_death_initialization,
        demon_on_attack,
        demon_link_event,
        demon_on_grab
    );
}