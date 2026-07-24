use super::*;

const EDGE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x9d9e10; //Sephiroth only
const EDGE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x9da390; //Sephiroth only
const EDGE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x9da970; //Sephiroth only
const EDGE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x9db070; //Sephiroth only
const EDGE_VTABLE_ON_ATTACK_OFFSET: usize = 0x9df7b0; //Sephiroth only
const EDGE_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET: usize = 0x9e02f0; //Sephiroth only
const EDGE_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET: usize = 0x9e05b0; //Sephiroth only

unsafe extern "C" fn edge_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(boma) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    }
    0.into()
}

//Sephiroth Start Initialization
#[skyline::hook(offset = EDGE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let shield_data = ShieldDataResource::new(0.0, 9.5, 3.0, 0.0, 9.5, 3.0, 13.0, Hash40::new("top"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    common_initialization_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    add_shield_group(boma, resource, *FIGHTER_EDGE_SHIELD_GROUP_KIND_SPECIAL_LW_FLASH);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(edge_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Sephiroth Reset Initialization
#[skyline::hook(offset = EDGE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sephiroth Death Initialization
#[skyline::hook(offset = EDGE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let ret = original!()(vtable, fighter);
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    ret
}

//Sephiroth Once Per Fighter Frame
#[skyline::hook(offset = EDGE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn edge_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let ret = original!()(vtable, fighter);
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let retaliation_charge = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    let retaliation_charge_timer = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
    let activate_point = WorkModule::get_float(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
    let threshold_activate_point = WorkModule::get_float(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_THRESHOLD_ACTIVATE_POINT);
    let is_winged = WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
    if retaliation_charge > 0 {
        WorkModule::dec_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
        if retaliation_charge_timer <= 0 {
            WorkModule::dec_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
            WorkModule::set_int(boma, 1800, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
        }
    }
    UiManager::set_edge_materia_enable(entry_id, true);
    UiManager::set_edge_materia_info(entry_id, activate_point, threshold_activate_point, is_winged);
    ret
}

#[skyline::hook(offset = 0x9dd3f8, inline)]
unsafe fn edge_opff_winged_form_check(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = ctx.registers[21].x() as *mut BattleObjectModuleAccessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let fighter = &mut *(agent.global_table[FIGHTER].get_ptr() as *mut Fighter);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_CHANGE) {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_CHANGE);
        WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED);
        WorkModule::set_float(boma, 0.0, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ACTIVATE_POINT);
        WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX), *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_JUMP_COUNT_MAX);
        WorkModule::add_int(boma, WorkModule::get_param_int(boma, hash40("param_one_winged"), hash40("jump_add_count")), *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        let attack_power_mul = *((*((singletons::FighterParamAccessor2() as *const u8).add((0x59 as usize)*0x38+0x70) as *const u64)) as *const f32).add(0x160/0x4);
        WorkModule::set_float(boma, attack_power_mul, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLOAT_ONE_WINGED_ATTACK_POWER_MUL);
        set_one_winged_light_weight_data(fighter, false);
        WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_MOTION_BLEND_STATE);
        WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_WING_STATE);
        fun_71009de2b0(fighter);
        WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_HAIR_OFF_CHANGED);
        fun_71009de890(fighter);
        SoundModule::play_se(boma, Hash40::new("se_edge_winged_on"), true, false, false, false, enSEType(0));
        /*
        if *(singletons::FighterParamAccessor2() as *const u8).add(0xBC) < 1 {
            MotionAnimcmdModule::call_script_single(boma, 2, Hash40::new("sound_wingstart"), -1);
        }
        */
        MotionAnimcmdModule::call_script_single(boma, 2, Hash40::new("sound_wingstart"), -1);
        MotionAnimcmdModule::call_script_single(boma, 3, Hash40::new("effect_wingstart"), -1);
        MotionAnimcmdModule::call_script_single(boma, 1, Hash40::new("expression_wingstart"), -1);
    }
}

//Sephiroth On Attack
#[skyline::hook(offset = EDGE_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn edge_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    let mut is_near_floor = false;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_object = get_battle_object_from_id(opponent_object_id);
        let opponent_battle_object_id = (*opponent_object).battle_object_id;
        let opponent_boma = (*opponent_object).module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if collision_kind == 1 {
            if opponent_battle_object_id >> 0x1C == 0 && HitModule::get_status(opponent_boma, (*collision_log).receiver_id as i32, 0) == 0 {
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 && MotionModule::motion_kind(boma) == hash40("attack_s4_lw_wing") {
                    let opponent_boma = sv_battle_object::module_accessor(opponent_object_id);
                    let opponent_pos = *PostureModule::pos(opponent_boma);
                    let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
                    let mut pos = Vector3f{x: opponent_pos.x, y: opponent_pos.y, z: opponent_pos.z};
                    let distance_to_floor = GroundModule::get_distance_to_floor(opponent_boma, &pos, 4.0, true);
                    if distance_to_floor != -1.0 {
                        pos.y -= distance_to_floor;
                        is_near_floor = true;
                    }
                    PostureModule::set_pos(opponent_boma, &pos);
                    PostureModule::init_pos(opponent_boma, &pos, true, true);
                    if is_near_floor {
                        opponent_agent.set_situation(SITUATION_KIND_GROUND.into());
                        GroundModule::attach_ground(opponent_boma, true);
                        GroundModule::set_correct(opponent_boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                        KineticModule::mul_speed(opponent_boma, &Vector3f{x: 0.1, y: 0.1, z: 0.1}, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                        StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

//Sephiroth Shield Attack Detection Event
#[skyline::hook(offset = EDGE_VTABLE_SHIELD_ATTACK_DETECTION_EVENT_OFFSET)]
unsafe extern "C" fn edge_shield_attack_detection_event(_vtable: u64, fighter: &mut Fighter, event: *mut ShieldAttackCollisionEvent) {
    let boma = fighter.battle_object.module_accessor;
    let shield_group_index = (*event).group_index;
    let collision_log = (*event).collision_log;
    let opponent_battle_object = get_battle_object_from_id((*collision_log).opponent_object_id);
    let opponent_battle_object_vtable: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(**(opponent_battle_object as *const *const u64));
    if shield_group_index > 0 {
        if !opponent_battle_object_vtable(opponent_battle_object) && 3 < *(opponent_battle_object as *const u8).add(0x34) {
            if !WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD) {
                let opponent_boma = (*opponent_battle_object).module_accessor;
                let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
                let pos = *PostureModule::pos(boma);
                let opponent_pos = *PostureModule::pos(opponent_boma);
                let new_lr = if pos.x <= opponent_pos.x {1.0} else {-1.0};
                AttackModule::clear_all(boma);
                WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD);
                WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD_ONCE);
                if shield_group_index == 1 {
                    if WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE) < 2 {
                        WorkModule::set_int(boma, 1800, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE_TIMER);
                        WorkModule::inc_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
                    }
                    if (*opponent_battle_object).battle_object_id >> 0x1C == 0 {
                        StopModule::set_hit_stop_frame_fix(opponent_boma, 45);
                        WorkModule::set_int(opponent_boma, 45, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                        WorkModule::set_int(opponent_boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
                        EFFECT(opponent_agent, Hash40::new("edge_throwlw_gravity"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                        EFFECT(opponent_agent, Hash40::new("edge_shadowflare_hold"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, true);
                        StopModule::set_hit_stop_frame_fix(boma, 12);
                        WorkModule::set_int(boma, 12, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                        WorkModule::set_int(boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
                    }
                }
                if shield_group_index == 2 {
                    PostureModule::set_lr(boma, new_lr);
                    PostureModule::update_rot_y_lr(boma);
                    if (*opponent_battle_object).battle_object_id >> 0x1C == 0 {
                        StopModule::set_hit_stop_frame_fix(opponent_boma, 20);
                        WorkModule::set_int(opponent_boma, 20, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                        WorkModule::set_int(opponent_boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
                        StopModule::set_hit_stop_frame_fix(boma, 20);
                        WorkModule::set_int(boma, 20, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                        WorkModule::set_int(boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
                    }
                }
            }
        }
    }
}

//Sephiroth Shield Attack Transition Event
#[skyline::hook(offset = EDGE_VTABLE_SHIELD_ATTACK_TRANSITION_EVENT_OFFSET)]
unsafe extern "C" fn edge_shield_attack_transition_event(_vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD_ONCE) {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_LW_FLAG_IS_HIT_SHIELD_ONCE);
        if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_PARRY_FLASH {
            WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_IS_PARRY_FLASH);
        }
        StatusModule::change_status_request(boma, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, false);
    }
    1
}

unsafe extern "C" fn edge_flash_on_search_event(_vtable: u64, _weapon: &mut smash::app::Weapon, log: *mut CollisionLogScuffed) {
    let opponent_object_id = (*log).opponent_object_id;
    let opponent_category = (*log).opponent_object_category;
    let mut is_near_floor = false;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        if opponent_category == 0 {
            let opponent_boma = sv_battle_object::module_accessor(opponent_object_id);
            let opponent_pos = *PostureModule::pos(opponent_boma);
            let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
            let mut pos = Vector3f{x: opponent_pos.x, y: opponent_pos.y, z: opponent_pos.z};
            let distance_to_floor = GroundModule::get_distance_to_floor(opponent_boma, &pos, 8.0, true);
            if distance_to_floor != -1.0 {
                pos.y -= distance_to_floor;
                is_near_floor = true;
            }
            PostureModule::set_pos(opponent_boma, &pos);
            PostureModule::init_pos(opponent_boma, &pos, true, true);
            if is_near_floor {
                opponent_agent.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::attach_ground(opponent_boma, true);
                GroundModule::set_correct(opponent_boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::mul_speed(opponent_boma, &Vector3f{x: 0.1, y: 0.1, z: 0.1}, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
            }
        }
    }
}

pub fn install() {
    weapon_initialise_module(*WEAPON_KIND_EDGE_FLASH, ModuleInitModules::SearchModule);
    let _ = skyline::patching::Patch::in_text(0x51c0ff0).data(edge_flash_on_search_event as *const () as u64); //035
    //fuck it we ball (disables almost all code in the winged form on check)
    let _ = skyline::patching::Patch::in_text(0x9dd3f8).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd410).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd438).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd450).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd478).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd490).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd4b4).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd4c0).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd4d8).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd4f0).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd4f8).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd50c).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd514).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd544).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd560).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd584).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd5ac).nop();
    let _ = skyline::patching::Patch::in_text(0x9dd5d0).nop();
	skyline::install_hooks!(
        edge_start_initialization,
        edge_reset_initialization,
        edge_death_initialization,
        edge_opff,
        edge_opff_winged_form_check,
        edge_on_attack,
        edge_shield_attack_detection_event,
        edge_shield_attack_transition_event
    );
}