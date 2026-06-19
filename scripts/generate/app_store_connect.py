#!/usr/bin/env python3
"""
Generate App Store Connect API Rust client (app_store_connect) from Apple's OpenAPI spec.

Steps:
1. Download the OpenAPI specification from Apple
2. Prune to only keep paths and schemas used by the codebase
3. Generate Rust client via cargo progenitor
4. Clean up temporary files
"""
import json
import os
import subprocess
import sys
import urllib.request
import zipfile
from collections import deque
from pathlib import Path

# ── Paths ────────────────────────────────────────────────────────────────

ROOT = Path(__file__).resolve().parent.parent.parent
OUTPUT = ROOT / "crates/store_clients/app_store_connect"
CACHE_DIR = ROOT / ".cache/app_store_connect"


# ── Cache ────────────────────────────────────────────────────────────────

CACHE_DIR = ROOT / ".cache/app_store_connect"

# ── Step 1: Download ─────────────────────────────────────────────────────

def ensure_spec() -> Path:
    """Return cached spec path, downloading if not present."""
    cache_path = CACHE_DIR / "openapi.oas.json"
    if cache_path.exists():
        print(f"[1/4] Using cached spec: {cache_path.relative_to(ROOT)}")
        return cache_path

    url = "https://developer.apple.com/sample-code/app-store-connect/app-store-connect-openapi-specification.zip"
    zip_path = CACHE_DIR / "spec.zip"
    print(f"[1/4] Downloading OpenAPI spec from {url}...")

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    urllib.request.urlretrieve(url, zip_path)

    with zipfile.ZipFile(zip_path, "r") as zf:
        zf.extractall(CACHE_DIR)

    if not cache_path.exists():
        raise FileNotFoundError(f"Expected openapi.oas.json not found in {CACHE_DIR}")

    print(f"       Extracted to {cache_path.relative_to(ROOT)}")
    return cache_path


# ── Step 2: Prune ────────────────────────────────────────────────────────

def resolve_ref(ref, spec):
    parts = ref.lstrip('#/').split('/')
    obj = spec
    for part in parts:
        obj = obj.get(part, {})
    return obj


def collect_refs(obj, visited, spec):
    if isinstance(obj, dict):
        if '$ref' in obj:
            ref = obj['$ref']
            if ref not in visited:
                visited.add(ref)
                resolved = resolve_ref(ref, spec)
                collect_refs(resolved, visited, spec)
        for v in obj.values():
            collect_refs(v, visited, spec)
    elif isinstance(obj, list):
        for item in obj:
            collect_refs(item, visited, spec)


# Types we include via ?include=build,appStoreVersionLocalizations
KEEP_INCLUDE_TYPES = {
    'appInfoLocalizations': '#/components/schemas/AppInfoLocalization',
    'builds': '#/components/schemas/Build',
    'appStoreVersionLocalizations': '#/components/schemas/AppStoreVersionLocalization',
}

# Paths used by the codebase
KEEP_PATHS = {
    '/v1/apps',          # GET - list apps
    '/v1/apps/{id}',      # GET - get app
    '/v1/apps/{id}/appInfos',  # GET - list app info metadata
    '/v1/appInfos/{id}/appInfoLocalizations',  # GET - list app info localizations
    '/v1/appInfoLocalizations/{id}',  # GET/PATCH - app title/subtitle localization
    '/v1/apps/{id}/appStoreVersions',  # GET - list versions
    '/v1/appStoreVersions/{id}/appStoreVersionLocalizations',  # GET - list version localizations
    '/v1/appStoreVersionLocalizations/{id}',  # GET/PATCH - version listing localization
}

# Resource schemas whose `relationships` field can be stripped
RESOURCE_SCHEMAS = [
    'App', 'AppStoreVersion', 'Build', 'AppStoreVersionLocalization',
    'AppClip', 'AppClipDefaultExperience', 'AppCustomProductPage',
    'AppEncryptionDeclaration', 'AppEvent', 'AppInfo',
    'AppStoreReviewDetail', 'AppStoreVersionExperiment', 'AppStoreVersionExperimentV2',
    'AppStoreVersionPhasedRelease', 'AppStoreVersionSubmission',
    'BetaAppLocalization', 'BetaAppReviewDetail', 'BetaGroup',
    'BetaLicenseAgreement', 'AlternativeDistributionPackage',
    'EndUserLicenseAgreement', 'GameCenterDetail', 'GameCenterEnabledVersion',
    'GameCenterAppVersion', 'InAppPurchase', 'PrereleaseVersion',
    'PromotedPurchase', 'ReviewSubmission', 'RoutingAppCoverage',
    'SubscriptionGracePeriod', 'SubscriptionGroup', 'CiProduct',
    'AndroidToIosAppMappingDetail', 'BuildIcon',
]


def strip_included(schemas, response_schema_name):
    rs = schemas.get(response_schema_name)
    if not rs:
        return
    included = rs.get('properties', {}).get('included', {}).get('items', {})
    oneof = included.get('oneOf', [])
    if not oneof:
        return
    included['oneOf'] = [
        entry for entry in oneof
        if entry.get('$ref') in KEEP_INCLUDE_TYPES.values()
    ]
    disc = included.get('discriminator', {})
    mapping = disc.get('mapping', {})
    disc['mapping'] = {
        k: v for k, v in mapping.items()
        if v in KEEP_INCLUDE_TYPES.values()
    }
    if not included['oneOf']:
        del rs['properties']['included']


def _strip_attrs(schemas, resource_name, attrs_to_remove):
    """Remove unused attribute fields from a resource schema."""
    schema = schemas.get(resource_name)
    if not schema:
        return
    attrs = schema.get('properties', {}).get('attributes', {})
    props = attrs.get('properties', {})
    for attr in attrs_to_remove:
        props.pop(attr, None)


def prune(spec_path: Path) -> Path:
    print(f"[2/4] Pruning spec to only keep used paths and schemas...")

    with open(spec_path) as f:
        spec = json.load(f)

    schemas = spec['components']['schemas']

    # Strip included types we don't use
    strip_included(schemas, 'AppsResponse')
    strip_included(schemas, 'AppResponse')
    strip_included(schemas, 'AppStoreVersionsResponse')

    # Strip relationships from resource schemas
    for name in RESOURCE_SCHEMAS:
        schema = schemas.get(name)
        if schema and 'relationships' in schema.get('properties', {}):
            del schema['properties']['relationships']

    # Strip unused attribute fields to avoid pulling in unnecessary enum schemas
    # Code only uses: App.{name, bundleId}, AppStoreVersion.{versionString, appStoreState, earliestReleaseDate, createdDate}
    # Build.{version}, AppInfoLocalization.{name,subtitle}, AppStoreVersionLocalization.{description,promotionalText,whatsNew}
    _strip_attrs(schemas, 'App', [
        'subscriptionStatusUrl', 'subscriptionStatusUrlVersion',
        'subscriptionStatusUrlForSandbox', 'subscriptionStatusUrlVersionForSandbox',
        'contentRightsDeclaration', 'streamlinedPurchasingEnabled',
        'accessibilityUrl', 'sku', 'primaryLocale', 'isOrEverWasMadeForKids',
    ])
    _strip_attrs(schemas, 'Build', [
        'iconAssetToken', 'buildAudienceType',
        'uploadedDate', 'expirationDate', 'expired',
        'minOsVersion', 'lsMinimumSystemVersion',
        'computedMinMacOsVersion', 'computedMinVisionOsVersion',
        'processingState', 'usesNonExemptEncryption',
    ])
    _strip_attrs(schemas, 'AppStoreVersion', [
        'platform', 'appVersionState',
        'copyright', 'reviewType', 'releaseType',
        'usesIdfa', 'downloadable',
    ])

    # Build pruned spec
    new_spec = {
        'openapi': spec['openapi'],
        'info': spec['info'],
        'servers': spec.get('servers', []),
        'paths': {},
        'components': {
            'schemas': {},
            'securitySchemes': spec.get('components', {}).get('securitySchemes', {}),
        },
    }

    # Collect refs from kept paths
    refs = set()
    for path, methods in spec['paths'].items():
        if path in KEEP_PATHS:
            new_spec['paths'][path] = {}
            for method, operation in methods.items():
                if method == 'parameters':
                    new_spec['paths'][path][method] = operation
                elif method.upper() in {'GET', 'PATCH'}:
                    new_spec['paths'][path][method] = operation
                    collect_refs(operation, refs, spec)

    # BFS transitive schema resolution
    schema_queue = deque()
    for ref in refs:
        parts = ref.lstrip('#/').split('/')
        if len(parts) >= 3 and parts[0] == 'components' and parts[1] == 'schemas':
            name = parts[2]
            if name not in new_spec['components']['schemas']:
                schema_queue.append(name)

    while schema_queue:
        name = schema_queue.popleft()
        if name in new_spec['components']['schemas']:
            continue
        schema = schemas.get(name)
        if schema is None:
            continue
        new_spec['components']['schemas'][name] = schema

        new_refs = set()
        collect_refs(schema, new_refs, spec)
        for ref in new_refs:
            parts = ref.lstrip('#/').split('/')
            if len(parts) >= 3 and parts[0] == 'components' and parts[1] == 'schemas':
                dep_name = parts[2]
                if dep_name not in new_spec['components']['schemas']:
                    schema_queue.append(dep_name)

    pruned_path = spec_path.with_name("openapi.pruned.json")
    with open(pruned_path, 'w') as f:
        json.dump(new_spec, f, indent=2)

    old_paths = len(spec['paths'])
    old_schemas = len(schemas)
    new_paths = len(new_spec['paths'])
    new_schemas = len(new_spec['components']['schemas'])
    print(f"       Paths:  {old_paths:4d} → {new_paths:4d}  (removed {old_paths - new_paths})")
    print(f"       Schemas: {old_schemas:4d} → {new_schemas:4d}  (removed {old_schemas - new_schemas})")

    return pruned_path


# ── Step 3: Generate ─────────────────────────────────────────────────────

def generate(pruned_path: Path):
    print(f"[3/4] Generating Rust client via cargo progenitor...")
    if OUTPUT.exists():
        import shutil
        shutil.rmtree(OUTPUT)

    result = subprocess.run(
        [
            "cargo", "progenitor",
            "-i", str(pruned_path),
            "-o", str(OUTPUT),
            "--name", "fastforge_app_store_connect",
            "--version", "0.1.0",
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        print(f"       Error: {result.stderr}")
        sys.exit(1)

    lib = OUTPUT / "src" / "lib.rs"
    if lib.exists():
        lines = sum(1 for _ in lib.open())
        size = lib.stat().st_size
        print(f"       Generated {lib.relative_to(ROOT)} ({lines} lines, {size / 1024:.0f} KB)")

    # Fix up Cargo.toml metadata (progenitor overwrites it)
    cargo_toml = OUTPUT / "Cargo.toml"
    content = cargo_toml.read_text()
    content = content.replace(
        'version = "0.1.0"\nedition = "2024"\nlicense = "SPECIFY A LICENSE BEFORE PUBLISHING"',
        'version.workspace = true\nedition.workspace = true\nauthors.workspace = true\ndescription = "App Store Connect API client"\nlicense.workspace = true',
    )
    cargo_toml.write_text(content)
    print(f"       Fixed up {cargo_toml.relative_to(ROOT)}")


# ── Main ─────────────────────────────────────────────────────────────────

def main():
    spec_path = ensure_spec()
    pruned_path = prune(spec_path)
    generate(pruned_path)


if __name__ == "__main__":
    main()
