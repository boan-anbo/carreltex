import { readFile, writeFile } from 'node:fs/promises';

export async function readJsonFileV0(filePath) {
  const bytes = await readFile(filePath);
  return JSON.parse(bytes.toString('utf8'));
}

export async function writeJsonFileV0(filePath, payload) {
  await writeFile(filePath, `${JSON.stringify(payload, null, 2)}\n`);
}
