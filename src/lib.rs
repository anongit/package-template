#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsBuffer, JsObject, Result};

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[js_function]
fn sync_fn(ctx: CallContext) -> Result<JsBuffer> {
  let output = vec![4, 2];
  ctx.env.create_buffer_with_data(output).map(|v| v.into_raw())
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("sync", sync_fn)?;
  Ok(())
}
