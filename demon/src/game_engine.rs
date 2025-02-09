use c_mine::pointer_from_hook;
use crate::util::PointerProvider;

pub const GAME_ENGINE_RUNNING: PointerProvider<extern "C" fn() -> bool> = pointer_from_hook!("game_engine_running");
