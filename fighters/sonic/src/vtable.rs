use super::*;

const SONIC_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11d56f0; //Sonic only
const SONIC_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11d5820; //Sonic only
const SONIC_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x11d7b20; //Sonic only
const SONIC_VTABLE_ON_ATTACK_OFFSET: usize = 0x11d5a00; //Sonic only
const SONIC_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x11d63d0; //Sonic only
const SONIC_VTABLE_ON_DAMAGE_OFFSET: usize = 0x11d7910; //Sonic only

unsafe extern "C" fn sonic_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED);
    WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED);
    WorkModule::set_float(boma, 0.0, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_SEARCH_MISS_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    UiManager::set_sonic_meter_info(entry_id, 0.0);
}

unsafe extern "C" fn sonic_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn sonic_check_special_n_uniq(_fighter: &mut L2CFighterCommon) -> L2CValue {
    false.into()
}

unsafe extern "C" fn sonic_check_ground_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    0.into()
}

unsafe extern "C" fn sonic_check_air_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    0.into()
}

//Sonic Startup Initialization
#[skyline::hook(offset = SONIC_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    sonic_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(sonic_check_special_n_uniq as *const () as _));
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(sonic_check_ground_special_uniq as *const () as _));
    agent.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&L2CValue::Ptr(sonic_check_air_special_uniq as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sonic_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Sonic Reset Initialization
#[skyline::hook(offset = SONIC_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SONIC as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        sonic_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Sonic Death Initialization
#[skyline::hook(offset = SONIC_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sonic_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    sonic_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sonic OPFF
#[skyline::hook(offset = SONIC_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn sonic_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let cmd_cat1 = agent.global_table[CMD_CAT1].get_i32();
    let status_kind = StatusModule::status_kind(boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let timer = WorkModule::get_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
    let cooldown = WorkModule::get_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    let miss_timer = WorkModule::get_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_SEARCH_MISS_TIMER);
    let boost_value = WorkModule::get_float(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    let special_n_start_auto_attack_frame = WorkModule::get_param_int(boma, hash40("param_special_n"), hash40("special_n_start_auto_attack_frame"));
    let neutral_special_check = cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0;
    let neutral_special_statuses = [
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING, 
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT
    ].contains(&status_kind);
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) && !WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED) && neutral_special_check {
        if cooldown <= 0 {
            SEARCH(agent, 0, 0, Hash40::new("top"), 35.0, 0.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
            WorkModule::inc_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_SEARCH_MISS_TIMER);
            if miss_timer >= 1 {
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
                WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_SEARCH_MISS_TIMER);
            }
        }
        else {
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED) && !neutral_special_statuses && !is_damaged(boma) {
        if timer >= special_n_start_auto_attack_frame {
            ArticleModule::remove_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_HOMINGTARGET, ArticleOperationTarget(0));
            SoundModule::stop_se(boma, Hash40::new("se_sonic_tracking"), 0);
            search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
            DamageModule::set_damage_mul(boma, 1.0);
            DamageModule::set_reaction_mul(boma, 1.0);
            WorkModule::set_int(boma, 120, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
            WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
            WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED);
        }
        else {
            if timer >= 120 && neutral_special_check {
                ArticleModule::remove_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_HOMINGTARGET, ArticleOperationTarget(0));
                SoundModule::stop_se(boma, Hash40::new("se_sonic_tracking"), 0);
                search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
                StatusModule::change_status_force(boma, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, false);
                WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED);
                WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
            }
            DamageModule::set_damage_mul(boma, 1.1);
            DamageModule::set_reaction_mul(boma, 1.1);
            WorkModule::inc_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
        }
    }
    if neutral_special_statuses {
        WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_TIMER);
        SoundModule::stop_se(boma, Hash40::new("se_sonic_tracking"), 0);
        WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED);
    }
    if boost_value > 0.0 {
        WorkModule::sub_float(boma, 0.03, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    }
    if cooldown > 0 {
        WorkModule::dec_int(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    }
    if cooldown == 1 {
        gimmick_flash(agent);
    }
    UiManager::set_sonic_meter_info(entry_id, boost_value);
    UiManager::set_sonic_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

//Sonic On Attack
#[skyline::hook(offset = SONIC_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn sonic_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let get_attack_air_kind = ControlModule::get_attack_air_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let boost_value = WorkModule::get_float(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let opponent_object = get_battle_object_from_id(opponent_object_id);
    let opponent_battle_object_id = (*opponent_object).battle_object_id;
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if [1, 2].contains(&collision_kind) {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || motion_kind == hash40("attack_air_f") {
                MotionModule::change_motion(boma, Hash40::new("attack_air_f_hit"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    if [1, 2].contains(&collision_kind) {
        let attack_data = AttackModule::attack_data(boma, (*collision_log).collider_id as i32, (*collision_log).x35);
        let power = (*attack_data).power;
        if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status((*opponent_object).module_accessor, (*collision_log).receiver_id as i32, 0) == 0 {
            if boost_value+power < 100.0 {
                if collision_kind == 1 {
                    WorkModule::add_float(boma, power, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
                }
                else {
                    WorkModule::add_float(boma, power/2.0, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Sonic On Search
#[skyline::hook(offset = SONIC_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn sonic_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *const CollisionLog;
    let status_kind = StatusModule::status_kind(boma);
    let neutral_special_statuses = [
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING_START, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HOMING, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_CANCEL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_LANDING, 
        *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_REBOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT
    ].contains(&status_kind);
    if !neutral_special_statuses && !WorkModule::is_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED) {
        let opponent_id = (*collision_log).opponent_battle_object_id;
        if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                let tracking = SoundModule::play_se(boma, Hash40::new("se_sonic_tracking"), true, false, false, false, smash::app::enSEType(0));
                SoundModule::set_se_vol(boma, tracking as i32, 3.0, 0);
                ArticleModule::generate_article(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_HOMINGTARGET, false, -1);
                if ArticleModule::is_exist(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_HOMINGTARGET) {
                    let homingtarget_boma = get_article_boma(boma, *FIGHTER_SONIC_GENERATE_ARTICLE_HOMINGTARGET);
                    WorkModule::set_int(homingtarget_boma, 130, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                    WorkModule::set_int(homingtarget_boma, 130, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
                    WorkModule::set_int(homingtarget_boma, opponent_id as i32, 0x1000000A /*WEAPON_SONIC_HOMINGTARGET_INSTANCE_WORK_ID_INT_OBJECT_ID*/);
                }
                WorkModule::set_int(boma, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_SEARCH_MISS_TIMER);
                WorkModule::on_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_N_TARGET_DETECTED);
            }
        }
    }
    original!()(vtable, fighter, log)
}


//Sonic On Damage
#[skyline::hook(offset = SONIC_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn sonic_on_damage(_vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    let boma = fighter.battle_object.module_accessor;
    let boost_value = WorkModule::get_float(boma, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    let log = *(on_damage as *const u64).add(0x10/0x8);
    let damage = *(log as *const f32).add(0x4/0x4);
    if *(on_damage as *const u8).add(0x18) != 0 {
        WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_HI_FALL);
        WorkModule::off_flag(boma, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
        WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        if boost_value-damage > 0.0 {
            WorkModule::sub_float(boma, damage, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        sonic_start_initialization,
        sonic_reset_initialization,
        sonic_death_initialization,
        sonic_opff,
        sonic_on_attack,
        sonic_on_search,
        sonic_on_damage
    );
}