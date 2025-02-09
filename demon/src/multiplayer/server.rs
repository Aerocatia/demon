use c_mine::c_mine;

#[repr(C)]
pub struct Server {
    pub _unknown_0x000: u32,

    /// if 1, reset server game time?
    pub _unknown_0x004: u16,
    pub _unknown_0x006: u16,

    pub _unknown_0x008: [u8; 0x3EC],
    pub server_game_time: u32
}

const _: () = assert!(size_of::<Server>() == 0x3F8);

#[c_mine]
pub extern "C" fn update_server_game_time(server: Option<&mut Server>, game_time: u32) {
    let server = server.expect("update_server_game_time with null server");
    if server._unknown_0x004 == 1 {
        server.server_game_time = game_time;
    }
    assert!(server.server_game_time <= game_time, "Server game time ({}) fell behind game time ({game_time})!", server.server_game_time);
}
