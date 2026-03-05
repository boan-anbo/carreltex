#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const steps = {
  'assert-request-list': 'assert_request_list.cjs',
  'write-combined-request-list': 'write_combined_request_list.cjs',
  'assert-first-run': 'assert_first_run.cjs',
  'assert-hint-store': 'assert_hint_store.cjs',
  'assert-baseline-generator': 'assert_baseline_generator.cjs',
  'assert-second-run': 'assert_second_run.cjs',
  'assert-ondemand-integration': 'assert_ondemand_integration.cjs',
};

const [, , stepName, ...stepArgs] = process.argv;

if (!stepName || !(stepName in steps)) {
  const names = Object.keys(steps).join(', ');
  console.error(`FAIL: expected proof step name (${names})`);
  process.exit(1);
}

const stepPath = path.join(__dirname, 'proof_steps', steps[stepName]);
const result = spawnSync(process.execPath, [stepPath, ...stepArgs], {
  stdio: 'inherit',
});

if (typeof result.status === 'number') {
  process.exit(result.status);
}

console.error('FAIL: proof step exited without status');
process.exit(1);
