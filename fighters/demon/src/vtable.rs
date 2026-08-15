use super::*;

const DEMON_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x930ff0; //Kazuya only
const DEMON_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x931680; //Kazuya only
const DEMON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x932a00; //Kazuya only
const DEMON_VTABLE_ON_ATTACK_OFFSET: usize = 0x932f50; //Kazuya only
//const DEMON_VTABLE_LINK_EVENT_OFFSET: usize = 0x933800; //Kazuya only
const DEMON_VTABLE_ON_GRAB_OFFSET: usize = 0x934310; //Kazuya only

//Kazuya Reset Initialization
#[skyline::hook(offset = DEMON_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn demon_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kazuya Death Initialization
#[skyline::hook(offset = DEMON_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn demon_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    demon_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kazuya Once Per Fighter Frame
#[skyline::hook(offset = DEMON_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn demon_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    if *battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 2 {
        //Added
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_FORM_ACTIVE) {
            WorkModule::set_flag(boma, WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL), *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL_PREV);
            WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DEVIL);
            if MotionModule::motion_kind_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING) != hash40("invalid") {
                MotionModule::remove_motion_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING, false);
            }
            if MotionModule::motion_kind_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_DEVIL) == hash40("invalid") || MotionModule::is_end_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_DEVIL) {
                MotionModule::add_motion_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_DEVIL, Hash40::new("devil_form"), 0.0, 0.0, false, false, 0.0, true, true, false);
            }
        }
        else {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_CLOSE_WING) {
                MotionModule::add_motion_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING, Hash40::new_raw(0xacc7ac1bf), 6.0, 0.0, false, false, 0.0, true, true, false);
                WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_CLOSE_WING);
            }
            if MotionModule::motion_kind_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING) != hash40("invalid") && MotionModule::is_end_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING) {
                MotionModule::remove_motion_partial(boma, *FIGHTER_DEMON_MOTION_PART_SET_KIND_WING, false);
            }
        }
    }
}

//Kazuya On Attack
#[skyline::hook(offset = DEMON_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn demon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if [1, 2].contains(&collision_kind) {
        if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            let opponent_object = get_battle_object_from_id(opponent_object_id);
            let opponent_battle_object_id = (*opponent_object).battle_object_id;
            let opponent_boma = (*opponent_object).module_accessor;
            if opponent_battle_object_id >> 0x1C == 0 {
                let opponent_situation_kind = StatusModule::situation_kind(opponent_boma);
                let opponent_status_kind = StatusModule::status_kind(opponent_boma);
                let opponent_pos = *PostureModule::pos(opponent_boma);
                let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
                let slip_check = [*FIGHTER_STATUS_KIND_SLIP, *FIGHTER_STATUS_KIND_SLIP_WAIT, *FIGHTER_STATUS_KIND_SLIP_DAMAGE, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, *FIGHTER_STATUS_KIND_FIST_DOWN, *FIGHTER_STATUS_KIND_FIST_DOWN2, *FIGHTER_STATUS_KIND_FIST_DOWN3].contains(&opponent_status_kind);
                println!("Last Attack Hitbox ID: {}", LAST_ATTACK_HITBOX_ID);
                println!("Opponent Status Kind: {}", opponent_status_kind);
                if status_kind == *FIGHTER_DEMON_STATUS_KIND_ESCAPE_ATTACK {
                    if LAST_ATTACK_HITBOX_ID == 0 {
                        if collision_kind == 1 {
                            if slip_check {
                                MotionAnimcmdModule::call_script_single(boma, 0, Hash40::new("game_escapeattacktrip"), -1);
                            }
                        }
                    }
                    if LAST_ATTACK_HITBOX_ID == 1 {
                        if collision_kind == 1 {
                            let mut pos = Vector3f{x: opponent_pos.x, y: opponent_pos.y, z: opponent_pos.z};
                            let distance_to_floor = GroundModule::get_distance_to_floor(opponent_boma, &pos, 8.0, true);
                            let mut is_near_floor = false;
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
                                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DOWN, false);
                            }
                        }
                    }
                }
                if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_DASH_3 {
                    if opponent_situation_kind == *SITUATION_KIND_GROUND {
                        MotionAnimcmdModule::call_script_single(boma, 0, Hash40::new("game_attackdash3hit"), -1);
                    }
                }
                if [hash40("attack_stand_22"), hash40("attack_stand_23")].contains(&motion_kind) {
                    if collision_kind == 2 {
                        WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                    }
                }
                if motion_kind == hash40("attack_stand_31") {
                    if slip_check {
                        if collision_kind == 1 {
                            MotionAnimcmdModule::call_script_single(boma, 0, Hash40::new("game_attackstand31saving"), -1);
                        }
                    }
                }
                if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S {
                    if collision_kind == 2 {
                        if frame < 33.0 {
                            WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                        }
                    }
                }
                if status_kind == *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2H {
                    if collision_kind == 2 {
                        WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                    }
                }
            }
        }
    }
    call_original!(vtable, fighter, log)
}

/*
//Kazuya Link Event
#[skyline::hook(offset = DEMON_VTABLE_LINK_EVENT_OFFSET)]
pub unsafe extern "C" fn demon_link_event(vtable: u64, fighter: &mut Fighter, log: *mut u64) -> u64 {
    let ret = original!()(vtable, fighter, log);
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let event: &mut LinkEvent = std::mem::transmute(log);
    let event_kind = event.link_event_kind.0;
    if event_kind == hash40("capture") {
        println!("Is Capture");
        let capture_event: &mut LinkEventCapture = std::mem::transmute(event);
        let object_id = capture_event.sender_id;
        if object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
            println!("Object ID Isn't Invalid");
            let object = get_battle_object_from_id(object_id);
            if (*object).battle_object_id >> 0x1c == 0 {
                println!("Is a fighter");
                let link_boma = (*object).module_accessor;
                let lr = PostureModule::lr(boma);
                let link_lr = PostureModule::lr(link_boma);
                if link_lr == lr {
                    println!("Lr and opponent lr are equal");
                    if [*FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status_kind) && capture_event.status == *FIGHTER_STATUS_KIND_THROWN {
                        println!("Was in grab statues and grab kind is thrown");
                        //WorkModule::on_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_THROW_MOTION_CAMERA);
                        capture_event.node = smash2::phx::Hash40::new("throw");
                        capture_event.result = true;
                        capture_event.constraint = true;
                        StatusModule::change_status_request(boma, *FIGHTER_DEMON_STATUS_KIND_CATCH_BACK, false);
                    }
                }
            }
        }
    }
    ret
}
*/

unsafe extern "C" fn demon_on_search(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let collision_log = *(log as *const u64).add(0x10/0x8);
    let collision_log = collision_log as *mut CollisionLogScuffed;
    let opponent_object_id = (*collision_log).opponent_object_id;
    if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let opponent_category = sv_battle_object::category(opponent_object_id);
        let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
        let opponent_boma = (*opponent_battle_object).module_accessor;
        if opponent_category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            let opponent_status_kind = StatusModule::status_kind(opponent_boma);
            if [*FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&opponent_status_kind) {
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_EDGE, false);
                }
            }
        }
    }
}

//Kazuya On Grab
#[skyline::hook(offset = DEMON_VTABLE_ON_GRAB_OFFSET)]
unsafe extern "C" fn demon_on_grab(_vtable: u64, _fighter: &mut Fighter, catch_status: i32) -> i32 {
    return catch_status
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x933454).nop(); //Removes the call_script_single that creates the EWGF Unblockable Windbox
    let _ = skyline::patching::Patch::in_text(0x4fa2220).data(demon_on_search as *const () as u64);
    skyline::install_hooks!(
        demon_reset_initialization,
        demon_death_initialization,
        demon_opff,
        demon_on_attack,
        //demon_link_event,
        demon_on_grab
    );
}