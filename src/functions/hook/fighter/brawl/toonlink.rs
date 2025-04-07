use super::*;

const TOONLINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const TOONLINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const TOONLINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const TOONLINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

unsafe extern "C" fn toonlink_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_SHOOT_WIND);
    WorkModule::off_flag(boma, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_HURRICANE_SLASH_MOVE);
    WorkModule::off_flag(boma, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_DEKU_LEAF_ACTIVE);
}

unsafe extern "C" fn toonlink_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TOONLINK_INSTANCE_WORK_ID_FLAG_DEKU_LEAF_ACTIVE);
    }
    0.into()
}

//Toon Link Startup Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        toonlink_var(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(toonlink_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Toon Link Reset Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        toonlink_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Toon Link Death Initialization
#[skyline::hook(offset = TOONLINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn toonlink_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        toonlink_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Toon Link Once Per Fighter Frame
#[skyline::hook(offset = TOONLINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn toonlink_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_TOONLINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
        let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
        //Final Zoom Effect Clearing
        if counter > 0 {
            if counter == 20 {
                if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                    EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                    set_stage_visibility(boma, 1);
                    set_vis_hud(true);
                }
                else {
                    EffectModule::remove_screen(boma, Hash40::new("bg_toonlink_final"), -1);
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
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        toonlink_start_initialization,
        toonlink_reset_initialization,
        toonlink_death_initialization,
        toonlink_opff
    );
}