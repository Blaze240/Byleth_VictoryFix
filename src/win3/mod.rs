use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
    },
    smash_script::*,
    smashline::*
};

unsafe extern "C" fn master_sound_win3(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 52.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_01"));
        }
    frame(agent.lua_state_agent, 59.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win03"));
    }
    frame(agent.lua_state_agent, 160.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_02"));
    }
}
    else {
        frame(agent.lua_state_agent, 10.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win03_02"));
        }
        frame(agent.lua_state_agent, 57.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win01_03"));
        }
        frame(agent.lua_state_agent, 59.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_master_win03"));
        }
        frame(agent.lua_state_agent, 98.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("se_master_win02_03"));
        }
                     }
                    }
pub fn install() {
    Agent::new("master")
     .sound_acmd("sound_win3",master_sound_win3)
     .install();
}
