use super::*;

const ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaa6510; //Armstrong only
const ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaa6520; //Armstrong only
const ARMSTRONG_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d680; //Shared
const ARMSTRONG_VTABLE_ON_ATTACK_OFFSET: usize = 0xaa6540; //Armstrong only
const ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET: usize = 0x68d9e0; //Shared
const ARMSTRONG_VTABLE_LINK_EVENT_OFFSET: usize = 0xaa6990; //Armstrong only

unsafe extern "C" fn armstrong_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn armstrong_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::set_int(boma, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
    WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
    WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
}

//Armstrong Startup Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    armstrong_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(armstrong_end_control as *const () as _));
}

//Armstrong Reset Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        armstrong_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Armstrong Death Initialization
#[skyline::hook(offset = ARMSTRONG_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn armstrong_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    armstrong_var(&mut *boma);
}

//Armstrong Once Per Fighter Frame
#[skyline::hook(offset = ARMSTRONG_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn armstrong_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        //Charge Mechanics
        if [
            hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_hi3"), hash40("attack_lw3"), hash40("attack_air_f"), hash40("attack_air_b"), hash40("attack_air_hi"), 
            hash40("attack_air_lw"), hash40("special_s_start"), hash40("special_air_s_start"), hash40("special_hi_catch"), hash40("special_hi_throw"), hash40("special_lw"), hash40("special_air_lw")
        ].contains(&motion_kind) {
            let armor_multiplier = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
            let damage_multiplier = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
            let charge_frames = WorkModule::get_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            let charging = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK);
            let charging_special = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
            let charge_start = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
            let charge_end = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
            let max_charge: f32 = 20.0;
            match motion_kind {
                _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_lw3")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 3.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 9.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if motion_kind == hash40("attack_hi3") => {
                    WorkModule::set_float(boma, 12.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 20.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_f")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 4.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 12.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 5.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 14.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_b"), hash40("attack_air_hi")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 5.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ if [hash40("attack_air_lw"), hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) => {
                    WorkModule::set_float(boma, 3.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
                    WorkModule::set_float(boma, 11.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
                }
                _ => {}
            }
            if (charge_start..charge_end).contains(&frame) && (charge_frames < (max_charge as i32)) && (charging || charging_special) {
                let mut motion_rate: f32 = 0.0;
                if charging {
                    match motion_kind {
                        _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_lw3")].contains(&motion_kind) => {
                            motion_rate = 0.03*(charge_frames as f32);
                        }
                        _ if [hash40("attack_hi3"), hash40("attack_air_f"), hash40("attack_air_lw")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ if [hash40("attack_air_b"), hash40("attack_air_hi")].contains(&motion_kind) => {
                            motion_rate = 0.025*(charge_frames as f32);
                        }
                        _ => {}
                    }
                    MotionModule::set_rate(boma, motion_rate);
                    WorkModule::set_float(boma, 1.0+((1.0/14.0)*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
                    WorkModule::set_float(boma, 1.0+(0.02*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
                    if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw"), hash40("attack_hi3"), hash40("attack_lw3")].contains(&motion_kind) {
                        let mut armor: f32 = 0.0;
                        match motion_kind {
                            _ if [hash40("attack_s3_s"), hash40("attack_s3_hi"), hash40("attack_s3_lw")].contains(&motion_kind) => {
                                armor = 6.0;
                            }
                            _ if motion_kind == hash40("attack_hi3") => {
                                armor = 7.0;
                            }
                            _ if motion_kind == hash40("attack_lw3") => {
                                armor = 4.5;
                            }
                            _ => {}
                        }
                        DamageModule::set_reaction_mul(boma, 0.85/armor_multiplier);
                        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, armor*armor_multiplier, -1.0, -1);
                    }
                    WorkModule::inc_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
                if charging_special {
                    match motion_kind {
                        _ if [hash40("special_s_start"), hash40("special_air_s_start")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ if motion_kind == hash40("special_hi_throw") => {
                            motion_rate = 1.0;
                        }
                        _ if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind) => {
                            motion_rate = 0.045*(charge_frames as f32);
                        }
                        _ => {}
                    }
                    MotionModule::set_rate(boma, motion_rate);
                    WorkModule::set_float(boma, 1.0+((1.0/14.0)*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
                    WorkModule::set_float(boma, 1.0+(0.02*(charge_frames as f32)), *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
                    if [hash40("special_s_start"), hash40("special_lw")].contains(&motion_kind) {
                        DamageModule::set_reaction_mul(boma, 0.85/armor_multiplier);
                        DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, 9.0*armor_multiplier, -1.0, -1);
                    }
                    WorkModule::inc_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
                }
            }
            else {
                if charge_frames > 0 {
                    AttackModule::set_power_up(boma, damage_multiplier);
                }
                MotionModule::set_rate(boma, 1.0);
            }
        }
        else {
            WorkModule::set_int(boma, 0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
            WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
            WorkModule::set_float(boma, 0.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
            WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
            WorkModule::set_float(boma, 1.0, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
        }
    }
    original!()(vtable, fighter)
}

//Armstrong On Attack
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn armstrong_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && situation_kind != *SITUATION_KIND_AIR && frame < 94.0 {
        call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        call_special_zoom(boma, log, *FIGHTER_KIND_GANON, hash40("param_special_n"), 1, 0, 0, 0, 0);
    }
    original!()(vtable, fighter, log)
}

//Armstrong On Damage
#[skyline::hook(offset = ARMSTRONG_VTABLE_ON_DAMAGE_OFFSET)]
unsafe extern "C" fn armstrong_on_damage(vtable: u64, fighter: &mut Fighter, on_damage: u64) {
    if fighter.battle_object.kind == *FIGHTER_KIND_GANON as u32 {
        let boma = fighter.battle_object.module_accessor;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && (1.0..=79.0).contains(&frame) {
            HitModule::set_check_catch(boma, true, 0);
            DamageModule::set_no_reaction_mode_status(boma, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
            MotionModule::change_motion(boma, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    original!()(vtable, fighter, on_damage)
}

//Armstrong Link Event
#[skyline::hook(offset = ARMSTRONG_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn armstrong_link_event(_vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status = StatusModule::status_kind(boma);
    let event: &mut LinkEvent = std::mem::transmute(log);
    let event_kind = event.link_event_kind.0;
    if event_kind == hash40("capture") {
        let capture_event: &mut LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if status == *FIGHTER_STATUS_KIND_SPECIAL_HI && capture_event.status == *FIGHTER_STATUS_KIND_CLUNG_GANON {
            if LinkModule::is_link(boma, 0) {
                capture_event.result = false;
                return 0;
            }
            StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, false);
            capture_event.result = true;
            capture_event.constraint = false;
            let object_id = capture_event.sender_id;
            CatchModule::set_catch(boma, object_id);
            if !LinkModule::is_link(boma, 0) {
                return 0;
            }
            let ptr = get_module_vtable_func(boma, 0x130, 0x80);
            let func: extern "C" fn(catch_module: *mut u64) = std::mem::transmute(ptr);
            let catch_module = (boma as *mut u64).add(0x130/0x8);
            func(catch_module);
            let mut offset = (0.0, 0.0);
            if object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                let object = get_battle_object_from_id(object_id);
                let vtable = *(object as *const u64) as *const u64;
                let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
                if !func(object) {
                    if (*object).battle_object_id >> 0x1c == 0 {
                        offset.0 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_x"));
                        offset.1 = WorkModule::get_param_float((*object).module_accessor, hash40("param_motion"), hash40("ganon_special_hi_offset_y"));
                    }
                }
            }
            LinkModule::set_model_constraint_flag(boma, 0x2003);
            LinkModule::set_constraint_translate_offset(boma, &Vector3f{x: offset.0, y: offset.1, z: 0.0});
            return 0;
        }
        if status == *FIGHTER_STATUS_KIND_SPECIAL_S {
            if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_GANON {
                StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = smash2::phx::Hash40::new("throw");
                return 0;
            }
            if capture_event.status == *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON {
                StatusModule::change_status_request(boma, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, false);
                capture_event.result = true;
                capture_event.node = smash2::phx::Hash40::new("throw");
                return 0;
            }
        }
        if status == *FIGHTER_STATUS_KIND_FINAL && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
    }
    else if event_kind == 0xa84e26287 {
        if status == *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING {
            CatchModule::set_send_cut_event(boma, false);
            CatchModule::cling_cut(boma, false);
            if *(log as *const u8).offset(0x29) != 0 {
                return 0;
            }
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
            return 0;
        }
    }
    else if event_kind == 0xdac7c579e {
        if status == *FIGHTER_GANON_STATUS_KIND_FINAL_ATTACK {
            let object_id = event.sender_id;
            let object = get_battle_object_from_id(object_id);
            let vtable = *(object as *const u64) as *const u64;
            let func : fn(*mut BattleObject) -> bool = std::mem::transmute(*vtable);
            if !func(object)
            && (*object).battle_object_id >> 0x1c == 1
            && WorkModule::get_int(boma, *FIGHTER_GANON_STATUS_WORK_ID_INT_BEAST_BEAST_TASK_ID) as u32 == (*object).battle_object_id {
                WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_BEAST_END);
                return 1;
            }
        }
    }
    1
}

pub fn install() {
    //Nops the original location where Neutral Special inflicts critical zoom, as I only want the fully charged final hit to inflict critical zoom
    let _ = skyline::patching::Patch::in_text(0xaa6618).nop();
	skyline::install_hooks!(
        armstrong_start_initialization,
        armstrong_reset_initialization,
        armstrong_death_initialization,
        armstrong_opff,
        armstrong_on_attack,
        armstrong_on_damage,
        armstrong_link_event
    );
}