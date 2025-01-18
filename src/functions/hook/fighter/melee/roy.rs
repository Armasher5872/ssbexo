use super::*;

const ROY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10bb480; //Shared
const ROY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ROY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10bb700; //Shared
const ROY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10bbaa0; //Shared

unsafe extern "C" fn roy_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
    }
    0.into()
}

unsafe extern "C" fn roy_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SOL_ACTIVE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_OBJECT_ID);
    WorkModule::set_int(boma, 0, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_TIMER);
}

//Roy Startup Initialization
#[skyline::hook(offset = ROY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        roy_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(roy_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Roy Reset Initialization
#[skyline::hook(offset = ROY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        roy_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Roy Death Initialization
#[skyline::hook(offset = ROY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn roy_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Roy Once Per Fighter Frame
#[skyline::hook(offset = ROY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn roy_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_ROY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let sol_object_id = WorkModule::get_int(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_OBJECT_ID);
        let sol_timer = WorkModule::get_int(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_TIMER);
        if WorkModule::is_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SOL_ACTIVE) && sol_object_id != *BATTLE_OBJECT_ID_INVALID {
            let opponent_boma = sv_battle_object::module_accessor(sol_object_id as u32);
            let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
            let opponent_status_kind = StatusModule::status_kind(opponent_boma);
            if sol_timer > 0 || ![*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_STANDBY, *FIGHTER_STATUS_KIND_DEMO, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE].contains(&opponent_status_kind) {
                FighterUtil::set_face_motion_by_priority(opponent_boma, FighterFacial(*FIGHTER_FACIAL_POISON), Hash40::new("fill_blank_damage"));
                if [300, 270, 240, 210, 180, 150, 120, 90, 60, 30].contains(&sol_timer) {
                    macros::EFFECT_OFF_KIND(opponent_agent, Hash40::new("roy_fire"), false, false);
                    macros::EFFECT_OFF_KIND(opponent_agent, Hash40::new("roy_attack_fire"), false, false);
                    macros::EFFECT_FOLLOW(opponent_agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 7, 0, 0, 0, 0, 0.9, true);
                    opponent_agent.clear_lua_stack();
                    lua_args!(opponent_agent, Hash40::new("roy_attack_fire"), Hash40::new("top"), 0.0, 7.0, 0.0, 0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, true);
                    smash::app::sv_animcmd::EFFECT_FOLLOW_RND(opponent_agent.lua_state_agent);
                    opponent_agent.pop_lua_stack(1);
                }
                if [300, 240, 180, 120, 60].contains(&sol_timer) {
                    DamageModule::add_damage(opponent_boma, 1.0, 0);
                }
                WorkModule::dec_int(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_TIMER);
            }
            else {
                FighterUtil::cancel_face_motion_by_priority(opponent_boma, FighterFacial(*FIGHTER_FACIAL_POISON));
                WorkModule::off_flag(boma, *FIGHTER_ROY_INSTANCE_WORK_ID_FLAG_SOL_ACTIVE);
                macros::EFFECT_OFF_KIND(opponent_agent, Hash40::new("roy_fire"), false, false);
                macros::EFFECT_OFF_KIND(opponent_agent, Hash40::new("roy_attack_fire"), false, false);
                WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_ROY_INSTANCE_WORK_ID_INT_SOL_OBJECT_ID);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        roy_start_initialization,
        roy_reset_initialization,
        roy_death_initialization,
        roy_opff
    );
}