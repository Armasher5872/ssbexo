use super::*;

unsafe extern "C" fn cloud_special_s3_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL) {
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_r"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_l"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_r"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_l"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_r"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_l"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_r"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_l"), 5);
    }
    else {
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_r_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke1_l_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_r_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke2_l_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_r_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke3_l_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_r_lb"), 5);
        EffectModule::end_kind(fighter.module_accessor, Hash40::new("cloud_kyogiri_stroke4_l_lb"), 5);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, cloud_special_s3_end_status)
    .install()
    ;
}