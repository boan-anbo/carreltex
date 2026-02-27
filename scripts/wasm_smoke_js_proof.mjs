import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { createCtx } from './wasm_smoke_js/ctx.mjs';
import { createMemHelpers } from './wasm_smoke_js/mem.mjs';
import { createAssertHelpers } from './wasm_smoke_js/assert.mjs';
import { runCasesV0 } from './wasm_smoke_js/cases_v0.mjs';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const ctx = await createCtx(rootDir);
const mem = createMemHelpers(ctx);
const helpers = createAssertHelpers(ctx, mem);

runCasesV0(ctx, mem, helpers);
