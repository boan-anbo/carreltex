import { runWasmFixtureGalleryV1 } from './wasm_fixture_gallery_v1/main.mjs';

runWasmFixtureGalleryV1().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`FAIL: wasm fixture gallery v0: ${message}`);
  process.exit(1);
});
