use core::ffi::CStr;
use core::sync::atomic::Ordering;
use spin::Lazy;
use crate::tag::TagID;
use c_mine::{c_mine, pointer_from_hook};
use crate::init::{get_exe_type, ExeType, get_command_line_argument_value};
use crate::map::find_maps_with_prefix;
use crate::multiplayer::{GameConnectionState, GAME_CONNECTION_STATE};
use crate::multiplayer::map_list::{add_mp_map, all_mp_maps};
use crate::util::{utf16_to_slice, CStrPtr, PointerProvider, StaticStringBytes};

#[c_mine]
pub unsafe extern "C" fn choose_item_collection_item(item_collection: TagID) -> TagID {
    super::item_collection::choose_item_collection_item(item_collection)
}

#[c_mine]
pub unsafe extern "C" fn keystone_put(message: *const u16) {
    if message.is_null() {
        return
    }

    let string = utf16_to_slice(message);
    let string = StaticStringBytes::<256>::from_utf16(string);
    hud!("{string}")
}

#[c_mine]
pub extern "C" fn set_game_connection_state(state: u16) {
    let state = GameConnectionState::try_from(state).expect("set_game_connection_state with invalid state");
    GAME_CONNECTION_STATE.store(state as u16, Ordering::Relaxed);
}

#[c_mine]
pub extern "C" fn get_game_connection_state() -> GameConnectionState {
    GAME_CONNECTION_STATE.load(Ordering::Relaxed).try_into().expect("get_game_connection_state with invalid state")
}

const DECODE_PACKET_INNER: PointerProvider<unsafe extern "C" fn(*mut u8, *mut u8) -> u32> = pointer_from_hook!("decode_packet_inner");

#[c_mine]
pub unsafe extern "C" fn decode_packet(destination_data: *mut u8, header: *mut u8) -> u32 {
    let r = DECODE_PACKET_INNER.get()(destination_data, header);
    if *((*(header as *mut *mut u8)).add(4)) == 0xF {
        // FIXME: There seems to be a bug with these packets in this build; the channel is off by
        //        256?
        *(destination_data as *mut u32) += 256;
    }
    r
}

const ADD_MAP_TO_MP_MAP_LIST: PointerProvider<unsafe extern "C" fn(*const u8, usize)> = pointer_from_hook!("add_map_to_mp_map_list");

#[c_mine]
pub unsafe extern "C" fn create_multiplayer_map_list() {
    const ALL_MP_STOCK_MAPS: [&CStr; 19] = [
        c"levels\\test\\beavercreek\\beavercreek",
        c"levels\\test\\sidewinder\\sidewinder",
        c"levels\\test\\damnation\\damnation",
        c"levels\\test\\ratrace\\ratrace",
        c"levels\\test\\prisoner\\prisoner",
        c"levels\\test\\hangemhigh\\hangemhigh",
        c"levels\\test\\chillout\\chillout",
        c"levels\\test\\carousel\\carousel",
        c"levels\\test\\boardingaction\\boardingaction",
        c"levels\\test\\bloodgulch\\bloodgulch",
        c"levels\\test\\wizard\\wizard",
        c"levels\\test\\putput\\putput",
        c"levels\\test\\longest\\longest",
        c"levels\\test\\icefields\\icefields",
        c"levels\\test\\deathisland\\deathisland",
        c"levels\\test\\dangercanyon\\dangercanyon",
        c"levels\\test\\infinity\\infinity",
        c"levels\\test\\timberland\\timberland",
        c"levels\\test\\gephyrophobia\\gephyrophobia",
    ];

    for (index, string) in ALL_MP_STOCK_MAPS.iter().enumerate() {
        let string = string.to_str().unwrap();
        match get_exe_type() {
            ExeType::Cache => {
                let base_name = &string[string.rfind("\\").unwrap() + 1..string.len()];
                add_mp_map(base_name, Some(index as u32));
            },
            ExeType::Tag => {
                add_mp_map(string, Some(index as u32));
            }
        }
    }

    for i in find_maps_with_prefix("") {
        add_mp_map(i.as_str(), None);
    }

    if all_mp_maps().is_empty() {
        panic!("No multiplayer maps found!");
    }
}

#[c_mine]
pub unsafe extern "C" fn handle_connect_cli_arg() {
    const MAIN_CONNECT: PointerProvider<unsafe extern "C" fn(CStrPtr, CStrPtr)> = pointer_from_hook!("main_connect");

    let Some(ip_arg) = get_command_line_argument_value("-connect") else { return };
    let password_arg = get_command_line_argument_value("-password").unwrap_or(CStrPtr::from_cstr(c""));

    MAIN_CONNECT.get()(ip_arg, password_arg);
}

static NETWORK_LOGGING: Lazy<bool> = Lazy::new(|| ini_bool!("log", "network_logging").unwrap_or(false));

#[c_mine]
pub unsafe extern "C" fn dump_to_network_log_a(a: u32) {
    const FN: PointerProvider<unsafe extern "C" fn(u32)> = pointer_from_hook!("dump_to_network_log_a_inner");
    if *NETWORK_LOGGING {
        FN.get()(a)
    }
}

#[c_mine]
pub unsafe extern "C" fn dump_to_network_log_b(a: u32, b: u32) {
    const FN: PointerProvider<unsafe extern "C" fn(u32, u32)> = pointer_from_hook!("dump_to_network_log_b_inner");
    if *NETWORK_LOGGING {
        FN.get()(a,b)
    }
}
