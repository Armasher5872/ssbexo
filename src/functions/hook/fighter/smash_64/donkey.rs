use super::*;

const DONKEY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x993750; //Donkey Kong only
const DONKEY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x993ae0; //Donkey Kong only
const DONKEY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x993b40; //Donkey Kong only
const DONKEY_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x68d670; //Shared
const DONKEY_VTABLE_LINK_EVENT_OFFSET: usize = 0x993ee0; //Donkey Kong only

unsafe extern "C" fn donkey_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
    WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
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
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Donkey Kong Once Per Fighter Frame
#[skyline::hook(offset = DONKEY_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn donkey_opff(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let timer = WorkModule::get_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    if fighter.battle_object.kind == *FIGHTER_KIND_DONKEY as u32 {
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
        //Up B
        if ArticleModule::is_exist(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
            let barrel_boma = get_article_boma(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
            let barrel_frame = MotionModule::frame(barrel_boma);
            if barrel_frame > 40.0 {
                ArticleModule::remove_exist(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            }
        }
        //Down Special
        if timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
        }
        if timer <= 0 && WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) {
            WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
            fighter.battle_object.gimmick_flash();
        }
    }
    original!()(vtable, fighter)
}

//Link Event for Donkey Kong
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
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
	skyline::install_hooks!(
        donkey_start_initialization,
        donkey_reset_initialization,
        donkey_death_initialization,
        donkey_opff,
        donkey_link_event
    );
}