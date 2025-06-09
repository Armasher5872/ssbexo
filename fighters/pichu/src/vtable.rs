use super::*;

const PICHU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf2a520; //Shared
const PICHU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared
const PICHU_VTABLE_ON_ATTACK_OFFSET: usize = 0xf2ae00; //Shared
const PICHU_VTABLE_LINK_EVENT_OFFSET: usize = 0xf2a7c0; //Shared

unsafe extern "C" fn pichu_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn pichu_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_IS_VALID_NUZZLE);
    WorkModule::off_flag(boma, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_AGILITY_CAN_CANCEL);
}

//Pichu Startup Initialization
#[skyline::hook(offset = PICHU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        pichu_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(pichu_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Pichu Reset Initialization
#[skyline::hook(offset = PICHU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        pichu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pichu Death Initialization
#[skyline::hook(offset = PICHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pichu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        pichu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pichu On Attack
#[skyline::hook(offset = PICHU_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn pichu_on_attack(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::on_flag(boma, *FIGHTER_PICHU_INSTANCE_WORK_ID_FLAG_IS_VALID_NUZZLE);
        }
    }
    call_original!(vtable, fighter)
}

//Pichu Link Event
#[skyline::hook(offset = PICHU_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn pichu_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PICHU as u32 {
        if event.link_event_kind.0 == hash40("capture") {
            let boma = fighter.battle_object.module_accessor;
            let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
            let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
            let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                capture_event.motion_offset = offset;
                capture_event.motion_offset_lw = offset_lw;
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_THROW, false);
                return 0;
            }
            return 1;
        }
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
	skyline::install_hooks!(
        pichu_start_initialization,
        pichu_reset_initialization,
        pichu_death_initialization,
        pichu_on_attack,
        pichu_link_event
    );
}