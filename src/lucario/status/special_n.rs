use {
    smash::{
        lua2cpp::{L2CFighterCommon},
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    crate::table_const::*,
    super::helper::*
};

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucario_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    lucario_special_set_kinetic(fighter);
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
    let charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME);
    if charge < max_charge_frame as i32 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
        if !StopModule::is_stop(fighter.module_accessor) {
            lucario_special_n_substatus(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucario_special_n_substatus as *const () as _));
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    }
    lucario_special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if 0.0 < MotionModule::frame(fighter.module_accessor) {
            if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    return 0.into();
                }
            }
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
        }
    }
    0.into()
}

unsafe extern "C" fn lucario_special_n_joint_translate(fighter: &mut L2CFighterCommon) {
    let havel = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let haver = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("havel"),
        havel,
        true
    );
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("haver"),
        haver,
        true
    );
    let new_pos = Vector3f{x: havel.x + haver.x, y: havel.y + haver.y, z: havel.z + haver.z};
    let new_pos = Vector3f{x: new_pos.x * 0.5, y: new_pos.y * 0.5, z: new_pos.z * 0.5};
    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("throw"),
        &new_pos,
        true,
        false
    );
}

unsafe extern "C" fn lucario_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if !is_end {
        if !StatusModule::is_changing(fighter.module_accessor)
        && StatusModule::is_situation_changed(fighter.module_accessor) {
            lucario_special_set_kinetic(fighter);
            return 0.into();
        }
    }
    else {
        ControlModule::clear_command(fighter.module_accessor, true);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD) {
                fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
            }
            else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT) {
                fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
            }
        }
        else {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn lucario_special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    lucario_special_n_save_charge_status(fighter);
    0.into()
}

unsafe extern "C" fn lucario_special_n_save_charge_status(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    if status != statuses[0]
    || status_global == statuses[1]
    || status_global == statuses[3] {
        if status == statuses[1] {
            if status_global != statuses[2] {
                if status_global != statuses[3] {
                    lucario_special_n_save_charge_status_eff(fighter);
                    return;
                }
            }
        }
        else if status == statuses[2] {
            if status_global != statuses[3] {
                lucario_special_n_save_charge_status_eff(fighter);
                return;
            }
        }
        else if status != statuses[3] {
            lucario_special_n_save_charge_status_shoot(fighter);
            return;
        }
        else {
            if !ArticleModule::is_exist(fighter.module_accessor, statuses[5]) {
                lucario_special_n_save_charge_status_shoot(fighter);
                return;
            }
        }
    }
    lucario_special_n_save_charge_status_eff(fighter);
}

unsafe extern "C" fn lucario_special_n_save_charge_status_eff(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    let save_charge = if status_global != statuses[4] {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        false
    }
    else {
        if status != statuses[1] {
            if status != statuses[2] {
                EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
                false
            }
            else {
                EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
                true
            }
        }
        else {
            true
        }
    };
    if !save_charge {
        FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
    }
    else {
        let article = ArticleModule::get_article(fighter.module_accessor, 0);
        if article != 0 {
            let auraball = (*(article as *mut smash::app::Article)).battle_object.module_accessor;
            let charge = WorkModule::get_int(auraball, *WEAPON_LUCARIO_AURABALL_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
            FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, true, charge);
        }
    }
    ArticleModule::remove_exist(fighter.module_accessor, statuses[5], ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    lucario_special_n_save_charge_status_shoot(fighter);
}

unsafe extern "C" fn lucario_special_n_save_charge_status_shoot(fighter: &mut L2CFighterCommon) {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let status = StatusModule::status_kind(fighter.module_accessor);
    let status_global = fighter.global_table[STATUS_KIND].get_i32();
    let statuses = if kind != *FIGHTER_KIND_KIRBY {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
            *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    }
    else {
        [
            *FIGHTER_STATUS_KIND_SPECIAL_N,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_HOLD,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_MAX,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_SHOOT,
            *FIGHTER_KIRBY_STATUS_KIND_LUCARIO_SPECIAL_N_CANCEL,
            *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL
        ]
    };
    if status == statuses[0] {
        if status_global == statuses[3] {
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        }
    }
    if status == statuses[3] {
        FighterSpecializer_Lucario::save_aura_ball_status(fighter.module_accessor, false, 0);
    }
}

pub fn install() {
    install_status_scripts!(
        lucario_special_n_pre, lucario_special_n_main, lucario_special_n_end
    );
}