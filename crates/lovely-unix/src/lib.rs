use lovely_core::log::*;
use lovely_core::sys::{LuaState, LUA_LIB};
use std::{
    env,
    ffi::c_void,
    mem, panic,
    sync::{LazyLock, OnceLock},
};

use lovely_core::Lovely;

mod lualib;

static RUNTIME: OnceLock<Lovely> = OnceLock::new();

static RECALL: LazyLock<
    unsafe extern "C" fn(*mut LuaState, *const u8, isize, *const u8, *const u8) -> u32,
> = LazyLock::new(|| unsafe {
    let lua_loadbufferx: unsafe extern "C" fn(
        *mut LuaState,
        *const u8,
        isize,
        *const u8,
        *const u8,
    ) -> u32 = *LUA_LIB.get(b"luaL_loadbufferx").unwrap();
    let orig = dobby_rs::hook(
        lua_loadbufferx as *mut c_void,
        lua_loadbufferx_detour as *mut c_void,
    )
    .unwrap();
    mem::transmute(orig)
});

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C" fn luaL_loadbuffer(
    state: *mut LuaState,
    buf_ptr: *const u8,
    size: isize,
    name_ptr: *const u8,
) -> u32 {
    let rt = RUNTIME.get().unwrap_unchecked();
    rt.apply_buffer_patches(state, buf_ptr, size, name_ptr, std::ptr::null())
}

unsafe extern "C" fn lua_loadbufferx_detour(
    state: *mut LuaState,
    buf_ptr: *const u8,
    size: isize,
    name_ptr: *const u8,
    mode_ptr: *const u8,
) -> u32 {
    let rt = RUNTIME.get().unwrap_unchecked();
    rt.apply_buffer_patches(state, buf_ptr, size, name_ptr, mode_ptr)
}

#[ctor::ctor]
unsafe fn construct() {
    panic::set_hook(Box::new(|x| {
        let message = format!("lovely-injector has crashed: \n{x}");
        error!("{message}");
    }));
    let args: Vec<_> = env::args().collect();
    let dump_all = args.contains(&"--dump-all".to_string());

    let rt = Lovely::init(
        &|a, b, c, d, e| RECALL(a, b, c, d, e),
        lualib::get_lualib(),
        dump_all,
    );
    RUNTIME
        .set(rt)
        .unwrap_or_else(|_| panic!("Failed to instantiate runtime."));
}
