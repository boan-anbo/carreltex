import path from 'node:path';
import { pathToFileURL } from 'node:url';

export {
  sha256HexV0,
  readArtifactBytesV0,
  readLogBytesV0,
  entrypointSetOkV0,
  loadGalleryManifestV0,
  loadFixtureCasesV0,
  loadDeltaPolicyV1,
  classifyBaselineCmpV1,
  buildBaselineMetricsV1,
  computeBaselineMatchV0,
  buildConfigHashV0,
  buildTypedArtifactsPlaceholderV0,
  mergeStoreRequestsV0,
  normalizeStoreRequestFromEntryV0,
  loadStoreRequestsV0,
  runCaseV0,
  runWasmFixtureGalleryV1,
} from './runner_v0.mjs';

import { runWasmFixtureGalleryV1 } from './runner_v0.mjs';

const isDirectRun = process.argv[1]
  ? import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href
  : false;

if (isDirectRun) {
  runWasmFixtureGalleryV1().catch((error) => {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`FAIL: wasm fixture gallery v0: ${message}`);
    process.exit(1);
  });
}
