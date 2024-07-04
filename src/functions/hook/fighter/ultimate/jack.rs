//Certain hooks are accredited to HDR and WuBor Patch
use super::*;

const JACK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb2f960; //Joker only
const JACK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb2fd70; //Joker only
const JACK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb303a0; //Joker only
const JACK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xb31350; //Joker only
const JACK_VTABLE_ON_ATTACK_OFFSET: usize = 0xb33d30; //Joker only
const JACK_CUSTOMIZER_OFFSET: usize = 0xb2f820; //Joker only
const JACK_KILL_DEAD_EVENT_LISTENERS_OFFSET: usize = 0x37addc0; //Joker only
const JACK_FIGHTERSPECIALIZER_CHECK_DOYLE_SUMMON_DISPATCH_OFFSET: usize = 0xb30954; //Joker only

#[skyline::from_offset(JACK_CUSTOMIZER_OFFSET)]
extern "C" fn jack_customizer(boma: *mut BattleObjectModuleAccessor, customize_to: u32);

//Joker Startup Initialization
#[skyline::hook(offset = JACK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_float(boma, 0.0, FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_CURRENT_REBEL_GAUGE);
    original!()(vtable, fighter)
}

//Joker Reset Initialization
#[skyline::hook(offset = JACK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
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
    WorkModule::set_float(boma, 0.0, FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_CURRENT_REBEL_GAUGE);
    original!()(vtable, fighter)
}

//Joker Death Initialization
#[skyline::hook(offset = JACK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
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
    original!()(vtable, fighter, param_3)
}

//Joker Once Per Fighter Frame
#[skyline::hook(offset = JACK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn jack_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let rebel_gauge = WorkModule::get_float(boma, 0x4D);
    if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        WorkModule::set_float(boma, rebel_gauge, FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_CURRENT_REBEL_GAUGE);
    }
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) && rebel_gauge <= 0.0 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
    }
    original!()(vtable, fighter)
}

//Joker On Attack
#[skyline::hook(offset = JACK_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn jack_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    if !WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
            if motion_kind == hash40("attack_11") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 3.0);
            }
            if motion_kind == hash40("attack_12") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
            }
            if motion_kind == hash40("attack_13") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 4.0);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if frame < 10.0 {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
            }
            else {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 3.0);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
            if (11.0..13.0).contains(&frame) {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
            }
            if (20.0..23.0).contains(&frame) {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 3.0);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 4.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            if frame < 15.0 {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
            }
            else {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 4.0);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 9.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 8.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 7.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if motion_kind == hash40("attack_air_n") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 3.0);
            }
            if motion_kind == hash40("attack_air_f") {
                if frame < 12.0 {
                    FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 1.0);
                }
                else {
                    FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 4.0);
                }
            }
            if motion_kind == hash40("attack_air_b") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 6.0);
            }
            if motion_kind == hash40("attack_air_hi") {
                if frame < 20.0 {
                    FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 1.0);
                }
                else {
                    FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
                }
            }
            if motion_kind == hash40("attack_air_lw") {
                FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 5.0);
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_ESCAPE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_START, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE].contains(&status_kind) {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 1.0);
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), 2.0);
        }
    }
    call_original!(vtable, fighter, log)
}

#[skyline::hook(offset = JACK_FIGHTERSPECIALIZER_CHECK_DOYLE_SUMMON_DISPATCH_OFFSET, inline)]
unsafe fn jack_fighterspecializer_check_doyle_summon_dispatch_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[21].x.as_ref() as *mut BattleObjectModuleAccessor;
    WorkModule::off_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST);
    jack_customizer(boma, 0);
}

pub fn install() {
    skyline::install_hooks!(
        jack_start_initialization,
        jack_reset_initialization,
        jack_death_initialization,
        jack_opff,
        jack_on_attack,
        jack_fighterspecializer_check_doyle_summon_dispatch_hook
    );
    //Disables automatically summoning Arsene
    skyline::patching::Patch::in_text(0xb3153c).data(0x14000035u32);
    skyline::patching::Patch::in_text(0xb30dd4).data(0x14000031u32);
    //Nops the location where Jokers meter is set while Arsene is active
    skyline::patching::Patch::in_text(0xb31674).nop();
}