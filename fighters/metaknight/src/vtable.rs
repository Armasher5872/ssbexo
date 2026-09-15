use super::*;

//Set Move Customizer is accredited to WuBor Patch
unsafe extern "C" fn metaknight_waza_customize(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let waza_customize_to = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if waza_customize_to == *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1 {
        fighter.sv_set_status_func(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), LUA_SCRIPT_STATUS_FUNC_STATUS_PRE.into(), std::mem::transmute(metaknight_special_lw_pre_status as *const ()));
        0.into()
    }
    else if let Some(original) = get_original_customizer(fighter) {
        original(fighter)
    } 
    else {
        0.into()
    }
}

//Meta Knight Startup Initialization
unsafe extern "C" fn metaknight_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let shield_data = ShieldDataResource::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 10.0, Hash40::new("hip"), *COLLISION_SHAPE_TYPE_CAPSULE as u8, *SHIELD_TYPE_UNDEFINED as u8);
    let shield_datas = &mut (ShieldDatas::new().add(shield_data, 0));
    let resource = &mut ShieldGroupResource::new(shield_datas, 1, 0, false, false, false);
    common_initialization_variable_reset(&mut *boma);
    add_shield_group(boma, resource, *FIGHTER_METAKNIGHT_SHIELD_GROUP_KIND_SPECIAL_LW_GUARD);
    metaknight_var(&mut *boma);
    set_move_customizer(agent, metaknight_waza_customize);
    metaknight_waza_customize(agent);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Meta Knight Reset Initialization
unsafe extern "C" fn metaknight_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    metaknight_var(&mut *boma);
}

//Meta Knight Death Initialization
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 7, false, false))]
unsafe extern "C" fn metaknight_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    metaknight_var(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD);
    WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_LOOP);
}

//Meta Knight OPFF
#[skyline::hook(offset = get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 13, false, false))]
unsafe extern "C" fn metaknight_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER) {
        let handle = WorkModule::get_int(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
        if !EffectModule::is_exist_effect(boma, handle as u32) {
            let effect = EffectModule::req_follow(boma, Hash40::new("metaknight_sword"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
            WorkModule::set_int(boma, effect as i32, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
        }
    }
    if (!WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) || WorkModule::is_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD)) || WorkModule::is_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_LOOP) {
        if WorkModule::is_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD) || MotionModule::is_end_partial(boma, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) {
            WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD);
            WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_LOOP);
            MotionModule::add_motion_partial(boma, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new_raw(0x494f9390), 0.0, 1.0, true, false, 0.0, false, true, false);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD);
            WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_LOOP);
            MotionModule::remove_motion_partial(boma, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
        }
    }
    else {
        WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_WING_FOLD);
        MotionModule::add_motion_partial(boma, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new_raw(0x1705f44a), 0.0, 1.0, false, false, 0.0, false, true, false);
    }
}

//Meta Knight Shield Attack Detection Event
unsafe extern "C" fn metaknight_shield_attack_detection_event(_vtable: u64, fighter: &mut Fighter, event: *mut ShieldAttackCollisionEvent) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let pos = *PostureModule::pos(boma);
    let shield_group_index = (*event).group_index;
    let collision_log = (*event).collision_log;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if shield_group_index > 0 {
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_vtable: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(**(opponent_battle_object as *const *const u64));
            if !opponent_battle_object_vtable(opponent_battle_object) && 3 < *(opponent_battle_object as *const u8).add(0x34) {
                if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    let opponent_boma = (*opponent_battle_object).module_accessor;
                    let attack_data = *AttackModule::attack_data(opponent_boma, (*collision_log).collider_id as i32, (*collision_log).x35);
                    let opponent_pos = *PostureModule::pos(opponent_boma);
                    let new_lr = if pos.x <= opponent_pos.x {1.0} else {-1.0};
                    PostureModule::set_lr(boma, new_lr);
                    PostureModule::update_rot_y_lr(boma);
                    WorkModule::on_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT);
                    WorkModule::set_float(boma, attack_data.power, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
                } 
            }
        }
    }
}

//Meta Knight Shield Attack Transition Event
unsafe extern "C" fn metaknight_shield_attack_transition_event(_vtable: u64, fighter: &mut Fighter, _log: u64) {
    let boma = fighter.battle_object.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, false);
        WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_SHIELD_HIT);
    }
}

//Meta Knight On Damage Event
unsafe extern "C" fn metaknight_on_damage_event(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let damage = *((*(log as *const u64).add(0x10/0x8)) as *const f32).add(0x4/0x4);
    let subbed_damage = damage*0.6;
    let special_lw_damage = WorkModule::get_float(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
    if special_lw_damage-subbed_damage > 0.0 {
        WorkModule::set_float(boma, special_lw_damage-subbed_damage, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
    }
    else {
        WorkModule::set_float(boma, 0.0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLOAT_SPECIAL_LW_DAMAGE);
        EffectModule::kill(boma, WorkModule::get_int(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE) as u32, true, true);
        EffectModule::kill_kind(boma, Hash40::new("metaknight_beam"), false, false);
        WorkModule::set_int(boma, 0, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_LW_EFFECT_HANDLE);
        WorkModule::off_flag(boma, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_POWER);
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 0, false, true)).data(metaknight_start_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 4, false, true)).data(metaknight_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 50, false, true)).data(metaknight_shield_attack_detection_event as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 51, false, true)).data(metaknight_shield_attack_transition_event as *const () as u64);
    let _ = skyline::patching::Patch::in_text(get_agent_virtual_function(*FIGHTER_KIND_METAKNIGHT, 68, false, true)).data(metaknight_on_damage_event as *const () as u64);
    skyline::install_hooks!(
        metaknight_death_initialization,
        metaknight_opff
    );
}