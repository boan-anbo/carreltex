export function createMemHelpers(ctx) {
  function allocBytes(value, field) {
    const ptr = ctx.alloc(value.byteLength);
    if (!Number.isInteger(ptr) || ptr <= 0) {
      throw new Error(`alloc failed (${field}), ptr=${ptr}`);
    }
    new Uint8Array(ctx.memory.buffer, ptr, value.byteLength).set(value);
    return ptr;
  }

  function callWithBytes(value, field, callback) {
    const ptr = allocBytes(value, field);
    try {
      return callback(ptr, value.byteLength);
    } finally {
      ctx.dealloc(ptr, value.byteLength);
    }
  }

  return {
    allocBytes,
    callWithBytes,
  };
}
