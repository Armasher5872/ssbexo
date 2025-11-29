#![allow(unused_assignments)] //Addresses warning: value assigned to `event` is never read
use super::*;

//Ink Bullet Hit Main Status
unsafe extern "C" fn inkling_inkbullet_hit_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let get_touch_pos = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let owner_boma = get_owner_boma(weapon);
    let owner_agent = get_fighter_common_from_accessor(&mut *owner_boma);
    let owner_battle_object = sv_system::battle_object(owner_agent.lua_state_agent);
    let owner_kind = owner_battle_object.kind;
    let owner_fighter: *mut Fighter = std::mem::transmute(owner_battle_object);
    let owner_lr = PostureModule::lr(owner_boma);
    let ink_const = FighterSpecializer_Inkling::get_ink_work_id(owner_kind as i32);
    let ink = WorkModule::get_float(owner_boma, ink_const);
    let raycheck_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let fighterinkling: &mut FighterInkling = std::mem::transmute(owner_fighter);
    let paint_life = WorkModule::get_param_float(owner_boma, hash40("param_private"), hash40("paint_life"));
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        if 0.0 < ink {
            for x in 0..9 {
                let ink_param = &mut fighterinkling.ink_params[x];
                let first = x == 0 && !WorkModule::is_flag(owner_boma, 0x21000012);
                if !ink_param.enabled || first {
                    if first {
                        WorkModule::on_flag(owner_boma, 0x21000012);
                        raycheck_pos.x = raycheck_pos.x+owner_lr+owner_lr;
                    }
                    ink_param.enabled = true;
                    ink_param.raycheck_pos = Vector2f{x: get_touch_pos.x, y: get_touch_pos.y};
                    ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                    ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                    ink_param.unk = 0;
                    ink_param.paint_life = paint_life;
                    ArticleModule::generate_article(owner_boma, 0x4, false, -1);
                    if ArticleModule::is_exist(owner_boma, 0x4) {
                        let active_rollerink_count = ArticleModule::get_active_num(owner_boma, 0x4);
                        for idx in 0..active_rollerink_count {
                            let rollerink_article = get_article_from_no(owner_boma, 0x4, idx as i32);
                            let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
                            let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
                            WorkModule::set_float(rollerink_boma, get_touch_pos.x, 0x3);
                            WorkModule::set_float(rollerink_boma, get_touch_pos.y, 0x4);
                        }
                    }
                    break;
                }
            }
        }
    }
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("clash"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(inkling_inkbullet_hit_main_loop as *const () as _))
}

unsafe extern "C" fn inkling_inkbullet_hit_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let get_speed_x = {
        weapon.clear_lua_stack();
        lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        sv_kinetic_energy::get_speed_x(weapon.lua_state_agent)
    };
    let get_speed_y = {
        weapon.clear_lua_stack();
        lua_args!(weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        sv_kinetic_energy::get_speed_y(weapon.lua_state_agent)
    };
    let r = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_INKBULLET_INSTANCE_WORK_ID_FLOAT_R);
    let g = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_INKBULLET_INSTANCE_WORK_ID_FLOAT_G);
    let b = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_INKBULLET_INSTANCE_WORK_ID_FLOAT_B);
    let pos = PostureModule::pos(weapon.module_accessor);
    let scale = PostureModule::scale(weapon.module_accessor);
    let get_touch_pos_up = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32);
    let get_touch_normal_up = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32);
    let get_touch_pos_down = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let get_touch_normal_down = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let get_touch_pos_left = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    let get_touch_normal_left = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
    let get_touch_pos_right = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    let get_touch_normal_right = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
    let pos_x = (*pos).x;
    let pos_y = (*pos).y;
    let pos_z = (*pos).z;
    let mut x_pos = pos_x;
    let mut y_pos = pos_y;
    let mut x_touch = 0.0;
    let mut y_touch = 0.0;
    let get_touch_pos_up_x = get_touch_pos_up.x;
    let get_touch_pos_up_y = get_touch_pos_up.y;
    let get_touch_normal_up_x = get_touch_normal_up.x;
    let get_touch_normal_up_y = get_touch_normal_up.y;
    let get_touch_pos_down_x = get_touch_pos_down.x;
    let get_touch_pos_down_y = get_touch_pos_down.y;
    let get_touch_normal_down_x = get_touch_normal_down.x;
    let get_touch_normal_down_y = get_touch_normal_down.y;
    let get_touch_pos_left_x = get_touch_pos_left.x;
    let get_touch_pos_left_y = get_touch_pos_left.y;
    let get_touch_normal_left_x = get_touch_normal_left.x;
    let get_touch_normal_left_y = get_touch_normal_left.y;
    let get_touch_pos_right_x = get_touch_pos_right.x;
    let get_touch_pos_right_y = get_touch_pos_right.y;
    let get_touch_normal_right_x = get_touch_normal_right.x;
    let get_touch_normal_right_y = get_touch_normal_right.y;
    EffectModule::detach_all(weapon.module_accessor, 5);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INKLING_INKBULLET_INSTANCE_WORK_ID_FLAG_HIT) {
        EffectModule::req(weapon.module_accessor, Hash40::new_raw(0x17f8dcd76c), &Vector3f{x: pos_x, y: pos_y, z: pos_z}, &Vector3f::zero(), 1.0, 0u32, -1, false, 0);
        EffectModule::set_rgb_partial_last(weapon.module_accessor, r, g, b);
    }
    else {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
            x_pos = get_touch_pos_up_x;
            y_pos = get_touch_pos_up_y;
            x_touch = get_touch_normal_up_x;
            y_touch = get_touch_normal_up_y;
        }
        else {
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                x_pos = get_touch_pos_down_x;
                y_pos = get_touch_pos_down_y;
                x_touch = get_touch_normal_down_x;
                y_touch = get_touch_normal_down_y;
            }
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) {
                x_pos = get_touch_pos_left_x;
                y_pos = get_touch_pos_left_y;
                x_touch = get_touch_normal_left_x;
                y_touch = get_touch_normal_left_y;
            }
            if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
                x_pos = get_touch_pos_right_x;
                y_pos = get_touch_pos_right_y;
                x_touch = get_touch_normal_right_x;
                y_touch = get_touch_normal_right_y;
            }
            x_touch -= get_speed_x;
            y_touch -= get_speed_y;
        }
        EffectModule::req(weapon.module_accessor, Hash40::new_raw(0x1a72de490f), &Vector3f{x: x_pos, y: y_pos, z: pos_z}, &Vector3f{x: (x_touch).atan(), y: (y_touch).atan(), z: pos_z}, 1.0, 0u32, -1, false, 0);
        EffectModule::set_rgb_partial_last(weapon.module_accessor, r, g, b);
        SoundModule::play_se(weapon.module_accessor, Hash40::new_raw(0x16fe772137), true, false, false, false, enSEType(0));
        fun_71000369c0(weapon, x_pos.into(), y_pos.into(), pos_z.into(), scale.into(), scale.into());
    }
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    0.into()
}

unsafe extern "C" fn fun_71000369c0(weapon: &mut L2CWeaponCommon, x_pos: L2CValue, y_pos: L2CValue, z_pos: L2CValue, scale_1: L2CValue, scale_2: L2CValue) {
    let paint_pos_rand = WorkModule::get_param_float(weapon.module_accessor, hash40("param_inkbullet"),  hash40("paint_pos_rand"));
    let rand_degrees = sv_math::randf(hash40("weapon"), sv_math::randf(hash40("weapon"), paint_pos_rand*100.0)/100.0).to_degrees();
    let mut event = FighterInklingLinkEventPaint__new_l2c_table();
    let event_x = event["x"].get_f32();
    let event_y = event["y"].get_f32();
    event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new_raw(0x1badb0080f)));
    event["pos_"]["x"].assign(&L2CValue::F32(rand_degrees.sin()*event_x+x_pos.get_f32()));
    event["pos_"]["y"].assign(&L2CValue::F32(rand_degrees.cos()*event_y+y_pos.get_f32()));
    event["pos_"]["z"].assign(&L2CValue::F32(z_pos.get_f32()));
    event[0xc3da1bb73u64]["x"].assign(&L2CValue::F32(scale_1.get_f32()));
    event[0xc3da1bb73u64]["y"].assign(&L2CValue::F32(scale_2.get_f32()));
    let callable: extern "C" fn() -> *mut smash::app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_parents_struct(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, link_event);
    event = lua_bind::LinkEvent::store_l2c_table(link_event);
    let deleter: extern "C" fn(*mut smash::app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
}

//Side Special Pre Status
unsafe extern "C" fn inkling_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

//Splashbomb Explode Main Status
unsafe extern "C" fn inkling_splashbomb_explode_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let r = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_R);
    let g = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_G);
    let b = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_B);
    let pos = PostureModule::pos(weapon.module_accessor);
    let get_touch_pos = GroundModule::get_touch_pos(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let get_touch_normal = GroundModule::get_touch_normal(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    let owner_boma = get_owner_boma(weapon);
    let owner_agent = get_fighter_common_from_accessor(&mut *owner_boma);
    let owner_battle_object = sv_system::battle_object(owner_agent.lua_state_agent);
    let owner_kind = owner_battle_object.kind;
    let owner_fighter: *mut Fighter = std::mem::transmute(owner_battle_object);
    let owner_lr = PostureModule::lr(owner_boma);
    let ink_const = FighterSpecializer_Inkling::get_ink_work_id(owner_kind as i32);
    let ink = WorkModule::get_float(owner_boma, ink_const);
    let raycheck_pos = &mut Vector2f{x: 0.0, y: 0.0};
    let fighterinkling: &mut FighterInkling = std::mem::transmute(owner_fighter);
    let paint_life = WorkModule::get_param_float(owner_boma, hash40("param_private"), hash40("paint_life"));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, true, 0.0, false, false);
    VisibilityModule::set_whole(weapon.module_accessor, false);
    EffectModule::detach_all(weapon.module_accessor, 5);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new_raw(0x18c19c7ee8), &Vector3f{x: get_touch_pos.x, y: get_touch_pos.y, z: (*pos).z}, &Vector3f{x: (get_touch_normal.x).atan(), y: (get_touch_normal.y).atan().to_degrees(), z: (*pos).z}, 1.0, 0u32, -1, false, 0);
        EffectModule::set_rgb_partial_last(weapon.module_accessor, r, g, b);
        if 0.0 < ink {
            for y in 0..=2 {
                for x in 0..9 {
                    let ink_param = &mut fighterinkling.ink_params[x];
                    let first = x == 0 && !WorkModule::is_flag(owner_boma, 0x21000012);
                    if !ink_param.enabled || first {
                        if first {
                            WorkModule::on_flag(owner_boma, 0x21000012);
                            raycheck_pos.x = raycheck_pos.x+owner_lr+owner_lr;
                        }
                        ink_param.enabled = true;
                        match y {
                            1 => ink_param.raycheck_pos = Vector2f{x: get_touch_pos.x+8.0, y: get_touch_pos.y},
                            2 => ink_param.raycheck_pos = Vector2f{x: get_touch_pos.x-8.0, y: get_touch_pos.y},
                            _ => ink_param.raycheck_pos = Vector2f{x: get_touch_pos.x, y: get_touch_pos.y}
                        };
                        ink_param.some_vec = Vector2f{x: 0.0, y: 1.0};
                        ink_param.paint_size = Vector2f{x: 8.0, y: 1.0};
                        ink_param.unk = 0;
                        ink_param.paint_life = paint_life;
                        fighterinkling.ink_params[x] = *ink_param;
                        ArticleModule::generate_article(owner_boma, 0x4, false, -1);
                        if ArticleModule::is_exist(owner_boma, 0x4) {
                            let active_rollerink_count = ArticleModule::get_active_num(owner_boma, 0x4);
                            for idx in 0..active_rollerink_count {
                                let rollerink_article = get_article_from_no(owner_boma, 0x4, idx as i32);
                                let rollerink_battle_object_id = smash::app::lua_bind::Article::get_battle_object_id(rollerink_article) as u32;
                                let rollerink_boma = sv_battle_object::module_accessor(rollerink_battle_object_id);
                                match y {
                                    1 => {
                                        WorkModule::set_float(rollerink_boma, get_touch_pos.x+8.0, 0x3);
                                    },
                                    2 => {
                                        WorkModule::set_float(rollerink_boma, get_touch_pos.x-8.0, 0x3);
                                    },
                                    _ => {
                                        WorkModule::set_float(rollerink_boma, get_touch_pos.x, 0x3);
                                    }
                                };
                                WorkModule::set_float(rollerink_boma, get_touch_pos.y, 0x4);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    weapon.fastshift(L2CValue::Ptr(inkling_splashbomb_explode_main_loop as *const () as _))
}

unsafe extern "C" fn inkling_splashbomb_explode_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let current_frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let float_attack = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_ATTACK);
    let float_ink = WorkModule::get_float(weapon.module_accessor, *WEAPON_INKLING_SPLASHBOMB_INSTANCE_WORK_ID_FLOAT_INK);
    AttackModule::set_power(weapon.module_accessor, 0, float_attack, false);
    AttackModule::set_ink_value(weapon.module_accessor, 0, float_ink);
    if 1.0 < current_frame {
        if !AttackModule::is_attack(weapon.module_accessor, 0, false) {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
    }
    if current_frame >= 20.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("inkling")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, inkling_special_s_pre_status)
    .install()
    ;
    Agent::new("inkling_inkbullet")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_INKLING_INKBULLET_STATUS_KIND_HIT, inkling_inkbullet_hit_main_status)
    .install()
    ;
    Agent::new("inkling_splashbomb")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_INKLING_SPLASHBOMB_STATUS_KIND_EXPLODE, inkling_splashbomb_explode_main_status)
    .install()
    ;
}