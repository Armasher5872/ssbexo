//Credit to WuBoyTH for the following functions
use super::*;

//Deals with Inkling's Squid
pub unsafe extern "C" fn inkling_generate_squid_helper(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(fighter) {
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, frame);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
}

//Roller Ink Param, a custom event struct used to handle stage ink
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RollerInkParam {
    pub enabled: bool,
    pub padding: [u8; 0xF],
    pub raycheck_pos: Vector2f,
    pub some_vec: Vector2f,
    pub paint_size: Vector2f,
    pub unk: u64,
    pub paint_life: f32,
    pub padding2: [u8; 0xc]
}

//Fighter Inkling, used to properly assign the stage ink values
#[repr(C)]
pub struct FighterInkling {
    pub padding: [u8; 0x70],
    pub ink_params: [RollerInkParam; 10]
}