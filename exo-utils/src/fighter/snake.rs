use super::*;

pub unsafe extern "C" fn snake_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EARLY_END);
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
}

pub unsafe extern "C" fn fun_710001bc90(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    if !bool_check.get_bool() {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::set_float(boma, stick_x*lr, *FIGHTER_SNAKE_STATUS_SPECIAL_N_HOLD_WAIT_WORK_FLOAT_THROW_RATE);
            fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::I32(0));
            fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
            if WorkModule::is_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EARLY_END) {
                WorkModule::off_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EARLY_END);
                if situation_kind == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
            }
            else {
                fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), true.into());
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn fun_710001bb20(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        if ArticleModule::is_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE) {
            ArticleModule::remove_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            let grenade_count = lua_bind::ItemManager::get_num_of_ownered_item(singletons::ItemManager(), fighter.battle_object_id, ItemKind(*ITEM_KIND_SNAKEGRENADE));
            if grenade_count < 2 {
                if !ItemModule::is_have_item(boma, 0) {
                    ItemModule::have_item(boma, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0, 0, false, false);
                }
            }
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EARLY_END);
            return 1.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn fun_710001b2a0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_fighter = fighter.global_table[FIGHTER].get_ptr() as *mut Fighter;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if ![
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WAIT, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_F, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_F, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_B, 
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_WALK_BRAKE_B, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_F, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_DASH_B, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP, 
        *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_AERIAL, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_AIR, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_JUMP_SQUAT, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_HOLD_LANDING, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW
    ].contains(&status_kind) {
        if FighterSpecializer_Snake::is_constraint_article(global_fighter, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST)) & 1 != 0 {
            ArticleModule::shoot_exist(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        }
        WorkModule::off_flag(boma, *FIGHTER_SNAKE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_EARLY_END);
    }
    0.into()
}