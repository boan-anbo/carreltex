import { readFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const STATUS_OK_V0 = 'OK';
const STATUS_NI_V0 = 'NI';
const STATUS_INVALID_V0 = 'INVALID';
const STATUS_FAIL_V0 = 'FAIL';
const EXPECTED_STATUS_VALUES_V0 = new Set([STATUS_OK_V0, STATUS_NI_V0, STATUS_INVALID_V0, STATUS_FAIL_V0]);

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..', '..');
async function loadGalleryManifestV0() {
  const manifestPath = path.join(rootDir, 'scripts', 'wasm_fixture_gallery_v0_manifest.json');
  const bytes = await readFile(manifestPath);
  let parsed;
  try {
    parsed = JSON.parse(bytes.toString('utf8'));
  } catch {
    throw new Error(`invalid gallery manifest json: ${manifestPath}`);
  }
  const casesRaw = Array.isArray(parsed?.cases) ? parsed.cases : [];
  if (casesRaw.length === 0) {
    throw new Error(`gallery manifest has no cases: ${manifestPath}`);
  }

  const byId = new Map();
  for (const raw of casesRaw) {
    const id = raw?.id;
    const tagsRaw = Array.isArray(raw?.tags) ? raw.tags : [];
    const expectedStatus = raw?.expected_status;
    const purpose = raw?.purpose;
    if (typeof id !== 'string' || id.length === 0) {
      throw new Error(`gallery manifest case has invalid id: ${manifestPath}`);
    }
    if (byId.has(id)) {
      throw new Error(`gallery manifest has duplicate case id '${id}': ${manifestPath}`);
    }
    if (!EXPECTED_STATUS_VALUES_V0.has(expectedStatus)) {
      throw new Error(`gallery manifest case '${id}' has invalid expected_status '${expectedStatus}'`);
    }
    if (typeof purpose !== 'string' || purpose.trim() === '') {
      throw new Error(`gallery manifest case '${id}' has invalid purpose`);
    }
    const tags = tagsRaw
      .filter((tag) => typeof tag === 'string' && tag.trim() !== '')
      .map((tag) => tag.trim());
    byId.set(id, {
      tags,
      expected_status: expectedStatus,
      purpose: purpose.trim(),
      ondemand_opt_in: raw?.ondemand_opt_in === true,
    });
  }
  return {
    path: manifestPath,
    byId,
  };
}

async function loadFixtureCasesV0() {
  const texliveFixtures = [
    {
      id: 'typeset_demo_minimal_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_minimal_v0.tex',
    },
    {
      id: 'typeset_demo_table_mixed_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_table_mixed_probe_v0.tex',
    },
    {
      id: 'typeset_demo_table_overflow_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_table_overflow_probe_v0.tex',
    },
    {
      id: 'typeset_demo_capabilities_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_capabilities_v0.tex',
    },
    {
      id: 'typeset_demo_toc_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_toc_probe_v0.tex',
    },
    {
      id: 'typeset_demo_labels_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_labels_probe_v0.tex',
    },
    {
      id: 'typeset_demo_hyperref_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_probe_v0.tex',
    },
    {
      id: 'typeset_demo_hyperref_links_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_links_probe_v0.tex',
    },
    {
      id: 'typeset_demo_cjk_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_cjk_probe_v0.tex',
    },
    {
      id: 'typeset_demo_math_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_math_probe_v0.tex',
    },
    {
      id: 'typeset_demo_math_short_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_math_short_probe_v0.tex',
    },
    {
      id: 'typeset_demo_math_long_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_math_long_probe_v0.tex',
    },
    {
      id: 'typeset_demo_math_invalid_payload_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_math_invalid_payload_probe_v0.tex',
    },
    {
      id: 'typeset_demo_fixedpoint_graphics_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_fixedpoint_graphics_probe_v0.tex',
    },
    {
      id: 'typeset_demo_fixedpoint_bibliography_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_fixedpoint_bibliography_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bib_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bib_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bibstyle_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bibstyle_probe_v0.tex',
    },
    {
      id: 'typeset_demo_bib_resources_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_bib_resources_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_width_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_width_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_scale_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_scale_probe_v0.tex',
    },
    {
      id: 'typeset_demo_float_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_float_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphicspath_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphicspath_explicit_ext_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_explicit_ext_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphicspath_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphicspath_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_multipath_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_multipath_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_opts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_opts_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pkgopt_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pkgopt_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_hyperref_input_label_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_input_label_probe_v0.tex',
    },
    {
      id: 'typeset_demo_hyperref_include_label_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_include_label_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pageref_probe_v2',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pageref_probe_v2.tex',
    },
    {
      id: 'typeset_demo_pageref_include_probe_v2',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pageref_include_probe_v2.tex',
    },
    {
      id: 'typeset_demo_pageref_unresolved_probe_v2',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pageref_unresolved_probe_v2.tex',
    },
    {
      id: 'typeset_demo_hyperref_toc_input_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_hyperref_toc_input_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_probe_v0.tex',
    },
    {
      id: 'typeset_demo_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_cycle_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_cycle_probe_v0.tex',
    },
    {
      id: 'typeset_demo_input_missing_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_input_missing_probe_v0.tex',
    },
    {
      id: 'typeset_demo_ondemand_input_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_ondemand_input_probe_v0.tex',
    },
    {
      id: 'typeset_demo_ondemand_include_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_ondemand_include_probe_v0.tex',
    },
    {
      id: 'typeset_demo_includeonly_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_includeonly_probe_v0.tex',
    },
    {
      id: 'typeset_demo_package_require_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_package_require_probe_v0.tex',
    },
    {
      id: 'typeset_demo_pkgopt_require_pass_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_pkgopt_require_pass_probe_v0.tex',
    },
    {
      id: 'typeset_demo_class_options_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_class_options_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_opts_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_opts_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_opts_multi_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_opts_multi_probe_v0.tex',
    },
    {
      id: 'typeset_demo_passoptionstoclass_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_passoptionstoclass_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_documentclass_emptyopts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_documentclass_emptyopts_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_opts_multi_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_opts_multi_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_multipackage_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_multipackage_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_capture_probe_v1',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_capture_probe_v1.tex',
    },
    {
      id: 'typeset_demo_usepackage_multi_capture_probe_v1',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_multi_capture_probe_v1.tex',
    },
    {
      id: 'typeset_demo_usepackage_opts_normalize_probe_v1',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_opts_normalize_probe_v1.tex',
    },
    {
      id: 'typeset_demo_usepackage_multipackage_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_multipackage_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_usepackage_emptyopts_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_usepackage_emptyopts_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_package_require_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_package_require_invalid_probe_v0.tex',
    },
    {
      id: 'typeset_demo_resource_hints_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_resource_hints_probe_v0.tex',
    },
    {
      id: 'typeset_demo_nested_path_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_nested_path_probe_v0.tex',
    },
    {
      id: 'typeset_demo_graphics_opts_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_graphics_opts_probe_v0.tex',
    },
    {
      id: 'typeset_demo_resource_hints_invalid_probe_v0',
      mode: 'typeset',
      fixtureRelPath: 'scripts/texlive_smoke/fixtures/typeset_demo_resource_hints_invalid_probe_v0.tex',
    },
  ];

  const okFixtureDir = path.join(rootDir, 'scripts', 'wasm_smoke_js', 'fixtures');
  const entries = await readdir(okFixtureDir, { withFileTypes: true });
  const okFixtures = entries
    .filter((entry) => entry.isFile() && entry.name.endsWith('.tex'))
    .map((entry) => entry.name)
    .sort()
    .map((name) => {
      const stem = name.slice(0, -4);
      return {
        id: stem.startsWith('ok_') ? stem : `ok_${stem}`,
        mode: 'ok',
        fixtureRelPath: `scripts/wasm_smoke_js/fixtures/${name}`,
      };
    });

  const discovered = [...texliveFixtures, ...okFixtures];
  const manifest = await loadGalleryManifestV0();

  const merged = discovered.map((caseSpec) => {
    const metadata = manifest.byId.get(caseSpec.id);
    if (!metadata) {
      throw new Error(`gallery manifest missing discovered case '${caseSpec.id}'`);
    }
    return {
      ...caseSpec,
      tags: metadata.tags,
      expected_status: metadata.expected_status,
      purpose: metadata.purpose,
      ondemand_opt_in: metadata.ondemand_opt_in,
    };
  });

  for (const manifestId of manifest.byId.keys()) {
    if (!merged.find((item) => item.id === manifestId)) {
      throw new Error(`gallery manifest contains unknown case '${manifestId}'`);
    }
  }

  return {
    cases: merged,
    manifestPath: manifest.path,
  };
}


export {
  loadGalleryManifestV0,
  loadFixtureCasesV0,
};
