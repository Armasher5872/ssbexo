#![allow(improper_ctypes_definitions)]
use super::*;

const LITTLEMAC_UI_UPDATE_INTERNAL_OFFSET: usize = 0x68cda0; //Little Mac only
const LITTLEMAC_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc44790; //Little Mac only
const LITTLEMAC_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc44830; //Little Mac only
const LITTLEMAC_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc448c0; //Little Mac only
const LITTLEMAC_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0xc44b80; //Little Mac only
const LITTLEMAC_VTABLE_ON_ATTACK_OFFSET: usize = 0xc456a0; //Little Mac only
const LITTLEMAC_VTABLE_ON_SEARCH_OFFSET: usize = 0xc463d0; //Little Mac only
const LITTLEMAC_VTABLE_ON_DAMAGE_OFFSET: usize = 0xc45d70; //Little Mac only

#[skyline::from_offset(LITTLEMAC_UI_UPDATE_INTERNAL_OFFSET)]
fn update_littlemac_ui_internal(manager_offset: *mut u32, total_gauge: i32);

//Updates Battle UI, credit to HDR
unsafe extern "C" fn update_littlemac_ui(entry_id: i32, total_gauge: f32) {
    let manager = singletons::FighterManager() as *mut u64;
    let fighter_entry = (*manager + (entry_id as u64 * 8) + 0x20) as *mut u64;
    update_littlemac_ui_internal((*fighter_entry + 0x41e4) as *mut u32, total_gauge as i32);
}

unsafe extern "C" fn littlemac_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_USED_AIR_SPECIAL_N);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn littlemac_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_HAS_STAR);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_USED_AIR_SPECIAL_N);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FLAG_IS_START_AIR);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_STAR_DAMAGE);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
}

//Little Mac Startup Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(littlemac_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Little Mac Reset Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE_MAX_VALUE);
}

//Little Mac Death Initialization
#[skyline::hook(offset = LITTLEMAC_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn littlemac_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    littlemac_var(&mut *boma);
    original!()(vtable, fighter)
}

//Little Mac Once Per Fighter Frame
#[skyline::hook(offset = LITTLEMAC_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn littlemac_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    //Resets the meter to 0 if the values are invalid
    if ko_gauge < 0.0 || ko_gauge.is_nan() {
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    //Sets the meter to specific values if they're not the exact bounds
    if (1.0..=33.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if (35.0..=67.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 34.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if (69.0..=99.0).contains(&ko_gauge) {
        WorkModule::set_float(boma, 68.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    if ko_gauge > 100.0 {
        WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    }
    //Used to check if Little Mac has a star
    if ko_gauge > 0.0 {
        WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_HAS_STAR);
    }
    //Updates his UI accordingly
    update_littlemac_ui(entry_id, ko_gauge);
    //Used for training mode purposes
    littlemac_training_mode_features(boma);
    original!()(vtable, fighter)
}

//Little Mac On Attack
#[skyline::hook(offset = LITTLEMAC_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn littlemac_on_attack(meter: f32, vtable: u64, battle_object: *mut BattleObject, log: u64) -> u64 {
    let boma = &mut (*(*battle_object).module_accessor);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind as i32;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let opponent_object = get_battle_object_from_id(opponent_object_id);
    let opponent_battle_object_id = (*opponent_object).battle_object_id;
    let opponent_boma = &mut *(sv_battle_object::module_accessor(opponent_battle_object_id));
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let status_kind = StatusModule::status_kind(boma);
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let star_punch_strength = WorkModule::get_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    if status_kind == *FIGHTER_STATUS_KIND_THROW && motion_kind == hash40("throw_lw") && frame >= 16.0 {
        StatusModule::change_status_request(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && ko_gauge == 100.0 && star_punch_strength == 3 && collision_kind == *COLLISION_KIND_HIT {
        call_special_zoom(boma, log, *FIGHTER_KIND_LITTLEMAC, hash40("param_special_n"), 1, 0, 0, 0, 0);
    }
    original!()(meter, vtable, battle_object, log)
}

//Little Mac On Search
#[skyline::hook(offset = LITTLEMAC_VTABLE_ON_SEARCH_OFFSET)]
unsafe extern "C" fn littlemac_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_boma = &mut *(sv_battle_object::module_accessor((*collision_log).opponent_object_id));
    let opponent_category = sv_battle_object::category((*collision_log).opponent_object_id);
    let slow_frame = SlowModule::frame(opponent_boma, 0);
    let status_kind = StatusModule::status_kind(boma);
    if [1, 2].contains(&collision_kind) {
        let attack_data = AttackModule::attack_data(opponent_boma, (*collision_log).collider_id as i32, (*collision_log).x35);
        let power = (*attack_data).power;
        if power > 0.0 {
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                //Slows the opponent down
                if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    if slow_frame < 20 {
                        SlowModule::set(opponent_boma, 0, 8, 20, false, *BATTLE_OBJECT_ID_INVALID as u32);
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Little Mac On Damage
#[skyline::hook(offset = LITTLEMAC_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn littlemac_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let log = *(on_damage as *const u64).add(0x10/0x8);
    let damage = *((log as *const u64).add(0x4) as *const f32);
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_HAS_STAR) {
        WorkModule::add_float(boma, damage, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_STAR_DAMAGE);
    }
    //Removes a star for each 40% Little Mac takes
    if WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_STAR_DAMAGE) >= 40.0 && WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_HAS_STAR) {
        WorkModule::sub_float(boma, 1.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_STAR_DAMAGE);
    }
    //Allows Little Mac to do Side Special multiple times if he's hit
    if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
        WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
    call_original!(vtable, fighter, on_damage)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0xc45938).nop(); //Removes the vanilla special zoom call on Neutral Special
	skyline::install_hooks!(
        littlemac_start_initialization,
        littlemac_reset_initialization,
        littlemac_death_initialization,
        littlemac_opff,
        littlemac_on_attack,
        littlemac_on_search,
        littlemac_on_damage
    );
}