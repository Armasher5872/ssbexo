use super::*;

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
    Agent::new("inkling_splashbomb")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *WEAPON_INKLING_SPLASHBOMB_STATUS_KIND_EXPLODE, inkling_splashbomb_explode_main_status)
    .install()
    ;
}