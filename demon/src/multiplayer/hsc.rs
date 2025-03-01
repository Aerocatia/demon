use c_mine::pointer_from_hook;
use crate::script::HS_RETURN;
use crate::util::PointerProvider;

pub unsafe extern "C" fn sv_map_restart_eval(a: u32, b: u32, c: u32) {
    // TODO: Determine if `begin_end_game` advances the map cycle. If so, we need to make it not do
    //       this once sv_mapcycle_* is added.
    const BEGIN_END_GAME: PointerProvider<unsafe extern "C" fn()> = pointer_from_hook!("begin_end_game");
    BEGIN_END_GAME.get()();
    HS_RETURN.get()(b, 0);
}
