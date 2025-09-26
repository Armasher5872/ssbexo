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

unsafe extern "C" fn jack_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        WorkModule::on_flag(fighter.module_accessor, 0x200000E4);
        FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_DISPATCH);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, 0x200000E3);
        FighterSpecializer_Jack::check_doyle_summon_dispatch(fighter.module_accessor, true, false);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_JACK_STATUS_KIND_SUMMON);
    }
    1.into()
}

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
    let rebel_gauge = WorkModule::get_float(boma, 0x4D);
    if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) && rebel_gauge <= 0.0 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
    }
    original!()(vtable, fighter)
}

//Joker On Attack
#[skyline::hook(offset = JACK_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn jack_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let opponent_object = get_battle_object_from_id(opponent_object_id);
    let opponent_battle_object_id = (*opponent_object).battle_object_id;
    if [1, 2].contains(&collision_kind) {
        let attack_data = AttackModule::attack_data(boma, (*collision_log).collider_id as i32, (*collision_log).x35);
        let power = (*attack_data).power;
        if opponent_battle_object_id >> 0x1C == 0
        && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
            FighterSpecializer_Jack::add_rebel_gauge(boma, FighterEntryID(entry_id), power);
        }
    }
    call_original!(vtable, fighter, log)
}

#[skyline::hook(offset = JACK_FIGHTERSPECIALIZER_CHECK_DOYLE_SUMMON_DISPATCH_OFFSET, inline)]
unsafe fn jack_fighterspecializer_check_doyle_summon_dispatch_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
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