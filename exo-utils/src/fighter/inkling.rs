//Credit to WuBoyTH for the following functions
use super::*;

pub unsafe extern "C" fn inkling_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
    WorkModule::off_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_ON_ROLLER_INK);
    WorkModule::set_int(boma, 0, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
}

//Deals with Inkling's Squid
pub unsafe extern "C" fn inkling_generate_squid_helper(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    if !WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if is_excute(fighter) {
            ArticleModule::generate_article(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let rate = MotionModule::rate(boma);
    ArticleModule::change_motion(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if is_excute(fighter) {
        ArticleModule::set_frame(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, frame);
        ArticleModule::set_rate(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(boma);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
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

pub unsafe extern "C" fn inkling_spawn_stage_ink(fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let current_frame = agent.global_table[CURRENT_FRAME].get_f32();
    let prev_status_kind = agent.global_table[PREV_STATUS_KIND].get_i32();
    let battle_object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
    let ink_const = FighterSpecializer_Inkling::get_ink_work_id(fighter.battle_object.kind as i32);
    let roller_pos = ArticleModule::get_joint_pos(boma, *FIGHTER_INKLING_GENERATE_ARTICLE_ROLLER, Hash40::new("roll"), ArticleOperationTarget(0));
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let scale = PostureModule::scale(boma);
    let lr = PostureModule::lr(boma);
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let ink = WorkModule::get_float(boma, ink_const);
    let spawned_ink_count = WorkModule::get_int(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
    let paint_life = WorkModule::get_param_float(boma, hash40("param_private"), hash40("paint_life"));
    let fighterinkling: &mut FighterInkling = std::mem::transmute(fighter);
    let mut_fighter: *mut Fighter = std::mem::transmute(battle_object);
    let raycheck_height = (scale*-10.0).min(-10.0);
    let raycheck_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let roller_line = GroundModule::ray_check_hit_pos(boma, &Vector2f{x: roller_pos.x, y: roller_pos.y + 3.0}, &Vector2f{x: 0.0, y: raycheck_height}, raycheck_pos, true);
    if 0.0 < ink && situation_kind == *SITUATION_KIND_GROUND {
        if [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN].contains(&status_kind) {
            if roller_line {
                for x in 0..9 {
                    let ink_param = &mut fighterinkling.ink_params[x];
                    let first = x == 0 && !WorkModule::is_flag(boma, 0x21000012);
                    if !ink_param.enabled || first {
                        if first {
                            WorkModule::on_flag(boma, 0x21000012);
                            raycheck_pos.x = raycheck_pos.x+lr+lr;
                        }
                        ink_param.enabled = true;
                        ink_param.raycheck_pos = *raycheck_pos;
                        ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                        ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                        ink_param.unk = 0;
                        ink_param.paint_life = paint_life;
                        fighterinkling.ink_params[x] = *ink_param;
                        break;
                    }
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if current_frame == 10.0 {
                WorkModule::inc_int(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
            }
            if current_frame == 11.0 {
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
            }
            if current_frame == 19.0 {
                WorkModule::inc_int(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
            }
            if current_frame == 20.0 {
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
            }
            if current_frame > 20.0 {
                WorkModule::off_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
                WorkModule::set_int(boma, 0, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if current_frame == 7.0 {
                WorkModule::inc_int(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_INT_SPAWNED_INK_COUNT);
            }
            if current_frame == 8.0 {
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL].contains(&prev_status_kind) {
            if current_frame == 1.0 {
                WorkModule::on_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
            }
        }
        if WorkModule::is_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK) && situation_kind == *SITUATION_KIND_GROUND {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                for x in 0..999 {
                    let ink_param = &mut fighterinkling.ink_params[x];
                    let first = x == 0 && !WorkModule::is_flag(boma, 0x21000012);
                    if !ink_param.enabled || first {
                        if first {
                            WorkModule::on_flag(boma, 0x21000012);
                        }
                        ink_param.enabled = true;
                        if spawned_ink_count == 1 {
                            ink_param.raycheck_pos = Vector2f{x: pos_x+(12.0*lr), y: pos_y};
                        }
                        if spawned_ink_count == 2 {
                            ink_param.raycheck_pos = Vector2f{x: pos_x-(12.0*lr), y: pos_y};
                        }
                        ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                        ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                        ink_param.unk = 0;
                        ink_param.paint_life = paint_life;
                        FighterSpecializer_Inkling::generate_rollerink(mut_fighter);
                        if ArticleModule::is_exist(boma, 0x4) {
                            let active_rollerink_count = ArticleModule::get_active_num(boma, 0x4);
                            for idx in 0..active_rollerink_count {
                                let rollerink_article = get_article_from_no(boma, 0x4, idx as i32);
                                let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
                                let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
                                if spawned_ink_count == 1 {
                                    WorkModule::set_float(rollerink_boma, pos_x+(12.0*lr), 0x3);
                                }
                                if spawned_ink_count == 2 {
                                    WorkModule::set_float(rollerink_boma, pos_x-(12.0*lr), 0x3);
                                }
                                WorkModule::set_float(rollerink_boma, pos_y, 0x4);
                            }
                        }
                        break;
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                for y in 0..=2 {
                    for x in 0..999 {
                        let ink_param = &mut fighterinkling.ink_params[x];
                        let first = x == 0 && !WorkModule::is_flag(boma, 0x21000012);
                        if !ink_param.enabled || first {
                            if first {
                                WorkModule::on_flag(boma, 0x21000012);
                            }
                            ink_param.enabled = true;
                            if y == 0 {
                                ink_param.raycheck_pos = Vector2f{x: pos_x+(8.0*lr), y: pos_y};
                            }
                            if y == 1 {
                                ink_param.raycheck_pos = Vector2f{x: pos_x-(8.0*lr), y: pos_y};
                            }
                            if y == 2 {
                                ink_param.raycheck_pos = Vector2f{x: pos_x, y: pos_y};
                            }
                            ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                            ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                            ink_param.unk = 0;
                            ink_param.paint_life = paint_life;
                            FighterSpecializer_Inkling::generate_rollerink(mut_fighter);
                            if ArticleModule::is_exist(boma, 0x4) {
                                let active_rollerink_count = ArticleModule::get_active_num(boma, 0x4);
                                for idx in 0..active_rollerink_count {
                                    let rollerink_article = get_article_from_no(boma, 0x4, idx as i32);
                                    let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
                                    let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
                                    if y == 0 {
                                        WorkModule::set_float(rollerink_boma, pos_x+(8.0*lr), 0x3);
                                    }
                                    if y == 1 {
                                        WorkModule::set_float(rollerink_boma, pos_x-(8.0*lr), 0x3);
                                    }
                                    if y == 2 {
                                        WorkModule::set_float(rollerink_boma, pos_x, 0x3);
                                    }
                                    WorkModule::set_float(rollerink_boma, pos_y, 0x4);
                                }
                            }
                            break;
                        }
                    }
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && [*FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_ROT, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL].contains(&prev_status_kind) {
                for y in 0..=2 {
                    for x in 0..999 {
                        let ink_param = &mut fighterinkling.ink_params[x];
                        let first = x == 0 && !WorkModule::is_flag(boma, 0x21000012);
                        if !ink_param.enabled || first {
                            if first {
                                WorkModule::on_flag(boma, 0x21000012);
                            }
                            ink_param.enabled = true;
                            match y {
                                1 => ink_param.raycheck_pos = Vector2f{x: pos_x+(8.0*lr), y: pos_y},
                                2 => ink_param.raycheck_pos = Vector2f{x: pos_x-(8.0*lr), y: pos_y},
                                _ => ink_param.raycheck_pos = Vector2f{x: pos_x, y: pos_y}
                            };
                            ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                            ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                            ink_param.unk = 0;
                            ink_param.paint_life = paint_life;
                            FighterSpecializer_Inkling::generate_rollerink(mut_fighter);
                            if ArticleModule::is_exist(boma, 0x4) {
                                let active_rollerink_count = ArticleModule::get_active_num(boma, 0x4);
                                for idx in 0..active_rollerink_count {
                                    let rollerink_article = get_article_from_no(boma, 0x4, idx as i32);
                                    let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
                                    let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
                                    if y == 0 {
                                        WorkModule::set_float(rollerink_boma, pos_x+(8.0*lr), 0x3);
                                    }
                                    if y == 1 {
                                        WorkModule::set_float(rollerink_boma, pos_x-(8.0*lr), 0x3);
                                    }
                                    if y == 2 {
                                        WorkModule::set_float(rollerink_boma, pos_x, 0x3);
                                    }
                                    WorkModule::set_float(rollerink_boma, pos_y, 0x4);
                                }
                            }
                            break;
                        }
                    }
                }
            }
            WorkModule::off_flag(boma, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_SPAWN_INK);
        }
    }
}