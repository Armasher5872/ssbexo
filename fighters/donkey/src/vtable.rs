use super::*;

const DONKEY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x993750; //Donkey Kong only
const DONKEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x993ae0; //Donkey Kong only
const DONKEY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x993b40; //Donkey Kong only
const DONKEY_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x68d670; //Shared
const DONKEY_VTABLE_LINK_EVENT_OFFSET: usize = 0x993ee0; //Donkey Kong only
const DONKEY_VTABLE_ON_SEARCH_EVENT_OFFSET: usize = 0x68d8a0; //Shared

unsafe extern "C" fn donkey_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
}

unsafe extern "C" fn donkey_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    }
    0.into()
}

//Donkey Kong Startup Initialization
#[skyline::hook(offset = DONKEY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn donkey_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    donkey_var(&mut *boma);
    agent.global_table[THROW_F_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW.into());
	agent.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START.into());
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(donkey_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Donkey Kong Reset Initialization
#[skyline::hook(offset = DONKEY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn donkey_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    donkey_var(&mut *boma);
    original!()(vtable, fighter)
}

//Donkey Kong Death Initialization
#[skyline::hook(offset = DONKEY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn donkey_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Donkey Kong Once Per Fighter Frame
#[skyline::hook(offset = DONKEY_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn donkey_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_DONKEY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let frame = MotionModule::frame(boma);
        let motion_kind = MotionModule::motion_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let lr = PostureModule::lr(boma);
        let is_cstick = Buttons::from_bits_retain(ControlModule::get_button(boma)).intersects(Buttons::CStickOverride);
        let stick_x = if is_cstick {ControlModule::get_sub_stick_x(boma)} else {ControlModule::get_stick_x(boma)};
        let stick_y = if is_cstick {ControlModule::get_sub_stick_y(boma)} else {ControlModule::get_stick_y(boma)};
        let stick_x = stick_x*lr;
        //DK Taunt Holding
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind)
            && frame >= 48.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    MotionModule::set_frame_sync_anim_cmd(boma, 32.0, true, true, false);
                }
            }
        }
        //Cargo
        if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START {
            if motion_kind == hash40("throw_f") {
                MotionModule::change_motion(boma, Hash40::new("throw_hi"), 0.0, 1.0, false, 0.0, false, false);
            }
            if prev_status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL {
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
                WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            }
        }
        //Barrel Toss
        if [
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WAIT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_WALK, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_TURN, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP_SQUAT_B, 
            *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_JUMP, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_FALL, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_LANDING, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_PASS
        ].contains(&status_kind) {
            if WorkModule::is_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED) {
                if stick_x >= 0.4 {
                    StatusModule::change_status_force(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_F, false);
                }
                if stick_x <= -0.4 {
                    StatusModule::change_status_force(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_B, false);
                }
                if stick_y >= 0.4 {
                    StatusModule::change_status_force(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_HI, false);
                }
                if stick_y <= -0.4 {
                    StatusModule::change_status_force(boma, *FIGHTER_DONKEY_STATUS_KIND_THROW_F_LW, false);
                }
                if ArticleModule::is_exist(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL) {
                    let barrel_boma = get_article_boma(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL);
                    let life = WorkModule::get_int(barrel_boma, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                    if life <= 40 {
                        StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_CATCH_CUT, false);
                    }
                }
            }
        }
    }
    original!()(vtable, fighter)
}

//Donkey Kong Link Event
#[skyline::hook(offset = DONKEY_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        let direction = PostureModule::lr(boma) as i32;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_int(boma, direction, *FIGHTER_STATUS_SHOULDERED_DONKEY_WORK_INT_START_LR);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Donkey Kong On Search
#[skyline::hook(offset = DONKEY_VTABLE_ON_SEARCH_EVENT_OFFSET)]
unsafe extern "C" fn donkey_on_search(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_DONKEY as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let throw_hi_status_kind = agent.global_table[THROW_HI_STATUS_KIND].get_i32();
        let collision_log = *(log as *const u64).add(0x10/0x8);
        let collision_log = collision_log as *const CollisionLog;
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
            let opponent_id = (*collision_log).opponent_battle_object_id;
            if opponent_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                if sv_battle_object::category(opponent_id) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let opponent_boma = smash::app::sv_battle_object::module_accessor(opponent_id);
                    let opponent_kind = utility::get_kind(&mut *opponent_boma);
                    if opponent_kind == *WEAPON_KIND_KOOPAJR_CANNONBALL {
                        if is_barrel(opponent_boma) {
                            WorkModule::on_flag(boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
                            WorkModule::off_flag(boma, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLAG_DID_ATTACK);
                            WorkModule::set_int(opponent_boma, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_BOUND_COUNT);
                            WorkModule::set_int(opponent_boma, 0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_INT_THROW_ANGLE);
                            WorkModule::set_float(opponent_boma, 0.0, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
                            StatusModule::change_status_request_from_script(opponent_boma, *WEAPON_DONKEY_BARREL_STATUS_KIND_HELD, false);
                            StatusModule::change_status_request_from_script(boma, throw_hi_status_kind, false);
                        }
                    }
                }
            }
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
	skyline::install_hooks!(
        donkey_start_initialization,
        donkey_reset_initialization,
        donkey_death_initialization,
        donkey_opff,
        donkey_link_event,
        donkey_on_search
    );
}