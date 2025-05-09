//Certain hooks are accredited to HDR and WuBor Patch
use super::*;

const JACK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb2f960; //Joker only
const JACK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb2fd70; //Joker only
const JACK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb303a0; //Joker only
const JACK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xb31350; //Joker only
const JACK_VTABLE_ON_ATTACK_OFFSET: usize = 0xb33d30; //Joker only
const JACK_CUSTOMIZER_OFFSET: usize = 0xb2f820; //Joker only
const JACK_FIGHTERSPECIALIZER_CHECK_DOYLE_SUMMON_DISPATCH_OFFSET: usize = 0xb30954; //Joker only

#[skyline::from_offset(JACK_CUSTOMIZER_OFFSET)]
extern "C" fn jack_customizer(boma: *mut BattleObjectModuleAccessor, customize_to: u32);

//Set Move Customizer is accredited to WuBor Patch
unsafe extern "C" fn jack_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let waza_customize_to = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if [*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1, *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2].contains(&waza_customize_to) {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(jack_special_lw_pre_status as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

//Joker Startup Initialization
#[skyline::hook(offset = JACK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    set_move_customizer(agent, jack_waza_customize);
    jack_waza_customize(agent);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Joker Reset Initialization
#[skyline::hook(offset = JACK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Joker Death Initialization
#[skyline::hook(offset = JACK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn jack_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: i32) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter, param_3)
}

//Joker Once Per Fighter Frame
#[skyline::hook(offset = JACK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn jack_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let rebel_gauge = WorkModule::get_float(boma, 0x4D);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) && rebel_gauge <= 0.0 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
    }
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_jack_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
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
    let _ = skyline::patching::Patch::in_text(0xb3153c).data(0x14000035u32); //Disables automatically summoning Arsene
    let _ = skyline::patching::Patch::in_text(0xb30dd4).data(0x14000031u32); //Disables automatically summoning Arsene
    let _ = skyline::patching::Patch::in_text(0xb31674).nop(); //Nops the location where Jokers meter is set while Arsene is active
    skyline::install_hooks!(
        jack_start_initialization,
        jack_reset_initialization,
        jack_death_initialization,
        jack_opff,
        jack_on_attack,
        jack_fighterspecializer_check_doyle_summon_dispatch_hook
    );
}