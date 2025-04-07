use super::*;

const MURABITO_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdba810; //Shared
const MURABITO_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xdbab30; //Shared
const MURABITO_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xdbad80; //Shared
const MURABITO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xdbb940; //Shared

unsafe extern "C" fn murabito_var(boma: &mut BattleObjectModuleAccessor) {
    let team_no = TeamModule::team_no(boma) as i32;
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
    WorkModule::set_int(boma, team_no, *FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
}

//Villager Startup Initialization
#[skyline::hook(offset = MURABITO_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn murabito_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MURABITO as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
        murabito_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Villager Reset Initialization
#[skyline::hook(offset = MURABITO_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn murabito_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_MURABITO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        murabito_var(&mut *boma);
        WorkModule::set_int(boma, 0, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_NUM);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C8);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C9);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY_PREV);
    }
}

//Villager Death Initialization
#[skyline::hook(offset = MURABITO_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn murabito_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_MURABITO as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        murabito_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Villager Once Per Fighter Frame
#[skyline::hook(offset = MURABITO_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn murabito_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    ac_common(boma);
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_murabito_final"), -1);
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

pub fn install() {
	skyline::install_hooks!(
        murabito_start_initialization,
        murabito_reset_initialization,
        murabito_death_initialization,
        murabito_opff
    );
}