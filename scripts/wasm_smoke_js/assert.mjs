export function createAssertHelpers(ctx, mem) {
  function addMountedFile(pathValue, dataValue, label) {
    const encodedPath = new TextEncoder().encode(pathValue);
    if (encodedPath.byteLength === 0) {
      return mem.callWithBytes(dataValue, `${label}_data`, (dataPtr, dataLen) => {
        return ctx.mountAddFile(0, 0, dataPtr, dataLen);
      });
    }
    return mem.callWithBytes(encodedPath, `${label}_path`, (pathPtr, pathLen) => {
      return mem.callWithBytes(dataValue, `${label}_data`, (dataPtr, dataLen) => {
        return ctx.mountAddFile(pathPtr, pathLen, dataPtr, dataLen);
      });
    });
  }

  function pathBytes(pathValue) {
    return new TextEncoder().encode(pathValue);
  }

  function expectInvalid(value, label) {
    if (value !== 1) {
      throw new Error(`${label} expected invalid(1), got ${value}`);
    }
  }

  function expectNotImplemented(value, label) {
    if (value !== 2) {
      throw new Error(`${label} expected NOT_IMPLEMENTED(2), got ${value}`);
    }
  }

  function readCompileReportJson() {
    const jsonLen = ctx.reportLen();
    if (!Number.isInteger(jsonLen) || jsonLen <= 0 || jsonLen > 4096) {
      throw new Error(`report_len_v0 unexpected: ${jsonLen}`);
    }

    const outPtr = ctx.alloc(jsonLen);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`alloc failed for report, ptr=${outPtr}`);
    }

    try {
      const written = ctx.reportCopy(outPtr, jsonLen);
      if (written !== jsonLen) {
        throw new Error(`report_copy_v0 expected ${jsonLen}, got ${written}`);
      }
      const outBytes = new Uint8Array(ctx.memory.buffer, outPtr, jsonLen);
      const text = new TextDecoder().decode(outBytes);
      return JSON.parse(text);
    } finally {
      ctx.dealloc(outPtr, jsonLen);
    }
  }

  function readCompileLogBytes() {
    const bytesLen = ctx.logLen();
    if (!Number.isInteger(bytesLen) || bytesLen < 0 || bytesLen > 4096) {
      throw new Error(`compile_log_len_v0 unexpected: ${bytesLen}`);
    }
    if (bytesLen === 0) {
      return new Uint8Array();
    }

    const outPtr = ctx.alloc(bytesLen);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`alloc failed for compile log, ptr=${outPtr}`);
    }

    try {
      const written = ctx.logCopy(outPtr, bytesLen);
      if (written !== bytesLen) {
        throw new Error(`compile_log_copy_v0 expected ${bytesLen}, got ${written}`);
      }
      return new Uint8Array(ctx.memory.buffer, outPtr, bytesLen).slice();
    } finally {
      ctx.dealloc(outPtr, bytesLen);
    }
  }

  function readEventsBytes() {
    const bytesLen = ctx.eventsLen();
    if (!Number.isInteger(bytesLen) || bytesLen < 0 || bytesLen > 4096) {
      throw new Error(`events_len_v0 unexpected: ${bytesLen}`);
    }
    if (bytesLen === 0) {
      return new Uint8Array();
    }

    const outPtr = ctx.alloc(bytesLen);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`alloc failed for events, ptr=${outPtr}`);
    }

    try {
      const written = ctx.eventsCopy(outPtr, bytesLen);
      if (written !== bytesLen) {
        throw new Error(`events_copy_v0 expected ${bytesLen}, got ${written}`);
      }
      return new Uint8Array(ctx.memory.buffer, outPtr, bytesLen).slice();
    } finally {
      ctx.dealloc(outPtr, bytesLen);
    }
  }

  function readU32LE(bytes, offset) {
    return (
      bytes[offset]
      | (bytes[offset + 1] << 8)
      | (bytes[offset + 2] << 16)
      | (bytes[offset + 3] << 24)
    ) >>> 0;
  }

  function decodeEvents(bytes, label) {
    const events = [];
    let offset = 0;
    while (offset < bytes.length) {
      if (offset + 8 > bytes.length) {
        throw new Error(`${label}: truncated event header at offset=${offset}`);
      }
      const kind = readU32LE(bytes, offset);
      const payloadLen = readU32LE(bytes, offset + 4);
      offset += 8;
      if (offset + payloadLen > bytes.length) {
        throw new Error(`${label}: truncated event payload at offset=${offset}, len=${payloadLen}`);
      }
      events.push({
        kind,
        payload: bytes.subarray(offset, offset + payloadLen),
      });
      offset += payloadLen;
    }
    return events;
  }

  function assertEventsMatchLogAndStats(logBytes, expectedStatsExact, label) {
    const eventsBytes = readEventsBytes();
    const events = decodeEvents(eventsBytes, label);
    if (events.length !== 2) {
      throw new Error(`${label}: expected 2 events, got ${events.length}`);
    }
    const logEvent = events[0];
    if (logEvent.kind !== 1) {
      throw new Error(`${label}: event[0] kind expected 1, got ${logEvent.kind}`);
    }
    if (
      logEvent.payload.length !== logBytes.length
      || !logEvent.payload.every((byte, index) => byte === logBytes[index])
    ) {
      throw new Error(`${label}: event[0] payload bytes mismatch with compile log`);
    }

    const statsEvent = events[1];
    if (statsEvent.kind !== 2) {
      throw new Error(`${label}: event[1] kind expected 2, got ${statsEvent.kind}`);
    }
    let statsText;
    try {
      statsText = new TextDecoder('utf-8', { fatal: true }).decode(statsEvent.payload);
    } catch {
      throw new Error(`${label}: event[1] payload is not valid utf-8`);
    }
    if (/[ \t\r\n]/.test(statsText)) {
      throw new Error(`${label}: event[1] payload must not contain whitespace`);
    }
    if (statsText.includes('"unexpected_key"')) {
      throw new Error(`${label}: event[1] payload must not contain unexpected_key`);
    }

    let stats;
    try {
      stats = JSON.parse(statsText);
    } catch {
      throw new Error(`${label}: event[1] payload is not valid JSON`);
    }
    if (typeof stats !== 'object' || stats === null) {
      throw new Error(`${label}: event[1] JSON payload must be object`);
    }
    const statsKeys = Object.keys(stats);
    const expectedKeys = [
      'token_count',
      'control_seq_count',
      'char_count',
      'space_count',
      'begin_group_count',
      'end_group_count',
      'max_group_depth',
    ];
    if (statsKeys.length !== expectedKeys.length || !expectedKeys.every((key) => statsKeys.includes(key))) {
      throw new Error(`${label}: event[1] JSON keys mismatch`);
    }
    for (const [key, value] of Object.entries(expectedStatsExact)) {
      if (stats[key] !== value) {
        throw new Error(`${label}: event[1] ${key} expected ${value}, got ${stats[key]}`);
      }
    }
    if (!(typeof stats.token_count === 'number' && stats.token_count > 0)) {
      throw new Error(`${label}: event[1] token_count expected >0`);
    }
    if (!(typeof stats.char_count === 'number' && stats.char_count > 0)) {
      throw new Error(`${label}: event[1] char_count expected >0`);
    }
    return stats;
  }

  function readMountedFileBytes(pathValue, label) {
    const encodedPath = pathBytes(pathValue);
    return mem.callWithBytes(encodedPath, `${label}_path`, (pathPtr, pathLen) => {
      const len = ctx.mountReadFileLen(pathPtr, pathLen);
      if (!Number.isInteger(len) || len < 0 || len > 4 * 1024 * 1024) {
        throw new Error(`${label}: unexpected mounted file len=${len}`);
      }
      if (len === 0) {
        return new Uint8Array();
      }
      const outPtr = ctx.alloc(len);
      if (!Number.isInteger(outPtr) || outPtr <= 0) {
        throw new Error(`${label}: alloc failed for mounted file copy, ptr=${outPtr}`);
      }
      try {
        const written = ctx.mountReadFileCopy(pathPtr, pathLen, outPtr, len);
        if (written !== len) {
          throw new Error(`${label}: mounted file copy expected ${len}, got ${written}`);
        }
        return new Uint8Array(ctx.memory.buffer, outPtr, len).slice();
      } finally {
        ctx.dealloc(outPtr, len);
      }
    });
  }

  function assertReadbackZero(pathValue, label) {
    const encodedPath = pathBytes(pathValue);
    const len = mem.callWithBytes(encodedPath, `${label}_len_path`, (pathPtr, pathLen) => ctx.mountReadFileLen(pathPtr, pathLen));
    if (len !== 0) {
      throw new Error(`${label}: expected read_file_len=0, got ${len}`);
    }
    const copyNull = mem.callWithBytes(encodedPath, `${label}_copy_null_path`, (pathPtr, pathLen) =>
      ctx.mountReadFileCopy(pathPtr, pathLen, 0, 0),
    );
    if (copyNull !== 0) {
      throw new Error(`${label}: expected read_file_copy(null,0)=0`);
    }
    const outPtr = ctx.alloc(1);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`${label}: alloc(1) failed`);
    }
    try {
      const copyOne = mem.callWithBytes(encodedPath, `${label}_copy_one_path`, (pathPtr, pathLen) =>
        ctx.mountReadFileCopy(pathPtr, pathLen, outPtr, 1),
      );
      if (copyOne !== 0) {
        throw new Error(`${label}: expected read_file_copy(out,1)=0, got ${copyOne}`);
      }
    } finally {
      ctx.dealloc(outPtr, 1);
    }
  }

  function assertMainXdvArtifactEmpty(label) {
    const bytesLen = ctx.artifactMainXdvLen();
    if (bytesLen !== 0) {
      throw new Error(`${label}: expected main.xdv len=0, got ${bytesLen}`);
    }
    if (ctx.artifactMainXdvCopy(0, 0) !== 0) {
      throw new Error(`${label}: expected main.xdv copy(null,0)=0`);
    }

    const outPtr = ctx.alloc(1);
    if (!Number.isInteger(outPtr) || outPtr <= 0) {
      throw new Error(`${label}: alloc(1) failed for artifact copy check`);
    }
    try {
      if (ctx.artifactMainXdvCopy(outPtr, 1) !== 0) {
        throw new Error(`${label}: expected main.xdv copy(out,1)=0`);
      }
    } finally {
      ctx.dealloc(outPtr, 1);
    }

    const mainName = new TextEncoder().encode('main.xdv');
    const genericLen = mem.callWithBytes(mainName, `${label}_generic_main_len`, (namePtr, nameLen) =>
      ctx.artifactLenByName(namePtr, nameLen),
    );
    if (genericLen !== 0) {
      throw new Error(`${label}: expected generic artifact_len(main.xdv)=0, got ${genericLen}`);
    }
    const genericCopyNull = mem.callWithBytes(mainName, `${label}_generic_main_copy_null`, (namePtr, nameLen) =>
      ctx.artifactCopyByName(namePtr, nameLen, 0, 0),
    );
    if (genericCopyNull !== 0) {
      throw new Error(`${label}: expected generic artifact_copy(main.xdv,null,0)=0`);
    }
    const genericOutPtr = ctx.alloc(1);
    if (!Number.isInteger(genericOutPtr) || genericOutPtr <= 0) {
      throw new Error(`${label}: alloc(1) failed for generic artifact copy check`);
    }
    try {
      const genericCopyOne = mem.callWithBytes(mainName, `${label}_generic_main_copy_one`, (namePtr, nameLen) =>
        ctx.artifactCopyByName(namePtr, nameLen, genericOutPtr, 1),
      );
      if (genericCopyOne !== 0) {
        throw new Error(`${label}: expected generic artifact_copy(main.xdv,out,1)=0, got ${genericCopyOne}`);
      }
    } finally {
      ctx.dealloc(genericOutPtr, 1);
    }

    const unknownName = new TextEncoder().encode('unknown.bin');
    const unknownLen = mem.callWithBytes(unknownName, `${label}_generic_unknown_len`, (namePtr, nameLen) =>
      ctx.artifactLenByName(namePtr, nameLen),
    );
    if (unknownLen !== 0) {
      throw new Error(`${label}: expected generic artifact_len(unknown.bin)=0, got ${unknownLen}`);
    }
    const unknownCopy = mem.callWithBytes(unknownName, `${label}_generic_unknown_copy`, (namePtr, nameLen) =>
      ctx.artifactCopyByName(namePtr, nameLen, 0, 0),
    );
    if (unknownCopy !== 0) {
      throw new Error(`${label}: expected generic artifact_copy(unknown.bin,null,0)=0`);
    }
  }

  function assertNoEvents(label) {
    const bytes = readEventsBytes();
    if (bytes.length !== 0) {
      throw new Error(`${label}: expected zero events for INVALID_INPUT, got ${bytes.length} bytes`);
    }
  }

  return {
    addMountedFile,
    pathBytes,
    expectInvalid,
    expectNotImplemented,
    readCompileReportJson,
    readCompileLogBytes,
    readEventsBytes,
    decodeEvents,
    assertEventsMatchLogAndStats,
    readMountedFileBytes,
    assertReadbackZero,
    assertMainXdvArtifactEmpty,
    assertNoEvents,
  };
}
