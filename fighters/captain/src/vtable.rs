use super::*;

const CAPTAIN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x8b7ce0; //Captain Falcon only
const CAPTAIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x8b7610; //Captain Falcon only
const CAPTAIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x8b7cf0; //Captain Falcon only
const CAPTAIN_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x8b7d20; //Captain Falcon only
const CAPTAIN_VTABLE_ON_ATTACK_OFFSET: usize = 0x8b8b90; //Captain Falcon only

unsafe extern "C" fn captain_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
    WorkModule::set_int(boma, 0, *FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
}

//Captain Falcon Startup Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Captain Falcon Reset Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    original!()(vtable, fighter)
}

//Captain Falcon Death Initialization
#[skyline::hook(offset = CAPTAIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn captain_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    captain_var(&mut *boma);
    original!()(vtable, fighter)
}

//Captain Falcon Once Per Fighter Frame
#[skyline::hook(offset = CAPTAIN_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn captain_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let cmd_cat1 = agent.global_table[CMD_CAT1].get_i32();
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    let parry_timer = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    let parried = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    let falcon_punch_turn_count = WorkModule::get_int(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
    let get_touch_flag = GroundModule::get_touch_flag(boma);
    //Parry Voice
    if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind) {
        if (0.0..5.0).contains(&frame) {
            WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
            WorkModule::set_int(boma, 60, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
        }
    }
    if parry_timer > 0 {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    }
    if parry_timer == 30 {
        SoundModule::play_se(boma, Hash40::new("vc_captain_appeal02"), true, false, false, false, enSEType(0));
    }
    if parry_timer <= 0
    && parried == 1 {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::on_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
        };
        if WorkModule::is_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && (54.0..57.0).contains(&frame) {
            SoundModule::play_se(boma, Hash40::new("vc_captain_cheer"), true, false, false, false, enSEType(0));
        }
    };
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
    && WorkModule::is_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
    && frame > 70.0 {
        CancelModule::enable_cancel(boma);
    }
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN
    && WorkModule::is_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
    && frame > 104.0 {
        CancelModule::enable_cancel(boma);
    }
    if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN 
    && (25.0..40.0).contains(&frame)
    && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.5
    && falcon_punch_turn_count <= 15 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
    };
    if status_kind != *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN {
        WorkModule::set_int(boma, 0, *FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
    };
    if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
        WorkModule::off_flag(boma, *FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
    }
    //Down Special
    if motion_kind == hash40("special_air_lw") {
        if (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT != 0 && get_touch_flag == *GROUND_TOUCH_FLAG_LEFT as u64) || (cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT != 0 && get_touch_flag == *GROUND_TOUCH_FLAG_RIGHT as u64) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
    original!()(vtable, fighter)
}

//Captain Falcon On Attack
#[skyline::hook(offset = CAPTAIN_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn captain_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    call_original!(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        captain_start_initialization,
        captain_reset_initialization,
        captain_death_initialization,
        captain_opff,
        captain_on_attack
    );
}