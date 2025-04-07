use super::*;

const CHROM_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x10bb480; //Shared
const CHROM_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const CHROM_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10bb700; //Shared
const CHROM_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10bbaa0; //Shared

unsafe extern "C" fn chrom_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn chrom_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_CHROM_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK);
    WorkModule::set_int(boma, 0, *FIGHTER_CHROM_INSTANCE_WORK_ID_INT_SPECIAL_LW_HIT_COUNT);
}

//Chrom Startup Initialization
#[skyline::hook(offset = CHROM_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        chrom_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(chrom_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Chrom Reset Initialization
#[skyline::hook(offset = CHROM_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        chrom_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Chrom Death Initialization
#[skyline::hook(offset = CHROM_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn chrom_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        chrom_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Chrom Once Per Fighter Frame
#[skyline::hook(offset = CHROM_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn chrom_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_CHROM as u32 {
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
                    EffectModule::remove_screen(boma, Hash40::new("bg_chrom_final"), -1);
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
    let _ = skyline::patching::Patch::in_text(0x10bbf04).nop(); //Nops the original location where Neutral Special inflicts critical zoom, as I only want the fully charged final hit to inflict critical zoom
	skyline::install_hooks!(
        chrom_start_initialization,
        chrom_reset_initialization,
        chrom_death_initialization,
        chrom_opff
    );
}