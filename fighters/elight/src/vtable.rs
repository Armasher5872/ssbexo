use super::*;

const ELIGHT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xa285e0; //Mythra only
const ELIGHT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa28640; //Mythra only
const ELIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa28a80; //Mythra only
const ELIGHT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa28dd0; //Mythra only
const ELIGHT_VTABLE_ON_ATTACK_OFFSET: usize = 0xa29ab0; //Mythra only

//Mythra Startup Initialization
#[skyline::hook(offset = ELIGHT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn elight_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)   
}

//Mythra Reset Initialization
#[skyline::hook(offset = ELIGHT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn elight_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    original!()(vtable, fighter)
}

//Mythra Death Initialization
#[skyline::hook(offset = ELIGHT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn elight_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    original!()(vtable, fighter)
}

//Mythra Once Per Fighter Frame
#[skyline::hook(offset = ELIGHT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn elight_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    if WorkModule::is_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
    && agent.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
        WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
    original!()(vtable, fighter)
}

//Mythra On Attack
#[skyline::hook(offset = ELIGHT_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn elight_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    if (status_kind == *FIGHTER_STATUS_KIND_ATTACK && motion_kind == hash40("attack_13"))
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && [hash40("attack_air_f"), hash40("attack_air_b")].contains(&motion_kind)) {
        WorkModule::on_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    }
    call_original!(vtable, fighter, log)
}

pub fn install() {
    //Both of these patches are related to disabling the transition from the normal escape animation into Foresight. The offsets are located in Myhtra's Once Per Fighter Frame
    let _ = skyline::patching::Patch::in_text(0xa28e78).nop();
    let _ = skyline::patching::Patch::in_text(0xa28e84).data(0x140000ACu32);
	skyline::install_hooks!(
        elight_start_initialization,
        elight_reset_initialization,
        elight_death_initialization,
        elight_opff,
        elight_on_attack
    );
}