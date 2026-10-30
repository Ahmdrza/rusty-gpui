#!/usr/bin/env python3
"""Package Rusty GPUI in publication order and check a patch-free registry consumer.

Uses Cargo's local-registry source replacement only in temporary tooling. No
crates are uploaded, and published manifests contain normal registry versions.
Requires Python 3.11+, Rust, native GPUI prerequisites and cached dependencies.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tomllib

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--skip-consumer', action='store_true', help='Package and resolve only; skip compiling the isolated consumer')
parser.add_argument('--build-consumer', action='store_true', help='Also build the packaged starter as an independent native executable')
args = parser.parse_args()
if args.skip_consumer and args.build_consumer:
    parser.error('--skip-consumer and --build-consumer cannot be combined')
cargo = os.environ.get('CARGO', 'cargo')
cargo_home = Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo')).expanduser().resolve()
work = root / 'target' / 'release-check'
registry = work / 'registry'
report = work / 'report.json'
report.unlink(missing_ok=True)
registry.mkdir(parents=True, exist_ok=True)
if (registry / 'index').exists():
    shutil.rmtree(registry / 'index')
(registry / 'index').mkdir(exist_ok=True)

def index_path(name):
    if len(name) == 1: return Path('1') / name
    if len(name) == 2: return Path('2') / name
    if len(name) == 3: return Path('3') / name[0] / name
    return Path(name[:2]) / name[2:4] / name

def run(command, cwd=root):
    print('+', ' '.join(str(arg) for arg in command), flush=True)
    subprocess.run(command, cwd=cwd, check=True)

# Reuse registry metadata and archives already fetched by Cargo. Cargo's sparse
# cache is converted into the documented newline-delimited registry index form.
# A local registry cannot fetch missing archives: expose only cached versions,
# even if a newer index entry was fetched by an unrelated Cargo operation.
cached_archives = {
    archive.stem: archive
    for archive in (cargo_home / 'registry' / 'cache').glob('*/*.crate')
    if archive.is_file()
}
index_entries = 0
for cache in (cargo_home / 'registry' / 'index').glob('*/.cache'):
    for file in cache.rglob('*'):
        if not file.is_file(): continue
        entries = []
        for part in file.read_bytes().split(b'\0'):
            if part.startswith(b'{"name":'):
                record = json.loads(part)
                if record.get('name') and f"{record['name']}-{record['vers']}" in cached_archives:
                    entries.append(part)
        if entries:
            index_entries += 1
            destination = registry / 'index' / index_path(json.loads(entries[0])['name'])
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(b'\n'.join(entries) + b'\n')
if not index_entries:
    raise SystemExit('No Cargo sparse-index cache found. Run cargo fetch --locked before release checks.')
for archive in cached_archives.values():
    destination = registry / archive.name
    if destination.is_symlink() and not destination.exists(): destination.unlink()
    if not destination.exists(): destination.symlink_to(archive.resolve())

config = work / 'sources.toml'
config.write_text('[source.crates-io]\nreplace-with = "rusty-gpui-release-check"\n\n[source.rusty-gpui-release-check]\nlocal-registry = ' + json.dumps(str(registry)) + '\n')
common = ['--offline', '--config', str(config)]
version = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['package']['version']
order = ['rusty-gpui-http', 'rusty-gpui-runtime', 'rusty-gpui-theme', 'rusty-gpui-components', 'rusty-gpui']
artifacts = []

# Remove only previous staging copies, never genuine crates.io sources. The same
# unpublished version can change while preparing a release; Cargo assumes real
# registry versions are immutable and otherwise reuses the old extracted files.
previous = work / 'consumer-metadata.json'
if previous.exists():
    for package in json.loads(previous.read_text())['packages']:
        directory = Path(package['manifest_path']).parent
        if package['name'] in order and directory.parent.name.startswith('-') and directory.is_relative_to(cargo_home / 'registry' / 'src'):
            shutil.rmtree(directory, ignore_errors=True)

for name in order:
    run([cargo, 'package', '-p', name, '--allow-dirty', '--no-verify', *common])
    archive = root / 'target' / 'package' / f'{name}-{version}.crate'
    destination = registry / archive.name
    if destination.is_symlink(): destination.unlink()
    shutil.copyfile(archive, destination)
    with tarfile.open(archive) as tar:
        member = tar.getmember(f'{name}-{version}/Cargo.toml')
        manifest = tomllib.loads(tar.extractfile(member).read().decode())
        files = tar.getnames()
        assert not any('/target/' in item or '/apps/gallery/' in item for item in files)
        assert any(item.endswith('/README.md') for item in files)
        assert any('/LICENSE' in item for item in files)
        if name == 'rusty-gpui':
            for relative in ['assets/Inter.ttf', 'assets/Inter-OFL.txt', 'assets/icons-LICENSE-ISC', 'examples/minimal.rs', 'examples/hello.rs', 'examples/form.rs', 'examples/overlays.rs']:
                assert f'{name}-{version}/{relative}' in files, relative
            assert sum('/assets/icons/' in item and item.endswith('.svg') for item in files) == 86
        if name == 'rusty-gpui-components':
            assert f'{name}-{version}/UPSTREAM.md' in files
            assert f'{name}-{version}/locales/ui.yml' in files
    dependencies = []
    def add_deps(table, kind, target=None):
        for alias, spec in table.items():
            if isinstance(spec, str): spec = {'version': spec}
            assert 'path' not in spec and 'git' not in spec, (name, alias, spec)
            dependencies.append({'name': alias, 'req': spec.get('version', '*'),
                'features': spec.get('features', []), 'optional': spec.get('optional', False),
                'default_features': spec.get('default-features', True), 'target': target,
                'kind': kind, 'registry': None, **({'package': spec['package']} if 'package' in spec else {})})
    for section, kind in [('dependencies', 'normal'), ('build-dependencies', 'build'), ('dev-dependencies', 'dev')]:
        add_deps(manifest.get(section, {}), kind)
        for target, tables in manifest.get('target', {}).items(): add_deps(tables.get(section, {}), kind, target)
    record = {'name': name, 'vers': version, 'deps': dependencies,
        'cksum': hashlib.sha256(archive.read_bytes()).hexdigest(), 'features': {},
        'features2': manifest.get('features', {}), 'yanked': False, 'v': 2}
    destination = registry / 'index' / index_path(name)
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(record) + '\n')
    artifacts.append({'file': archive.name, 'sha256': record['cksum'],
                      'bytes': archive.stat().st_size, 'files': len(files)})

consumer = work / 'consumer'
(consumer / 'src').mkdir(parents=True, exist_ok=True)
(consumer / 'Cargo.lock').unlink(missing_ok=True)
(consumer / 'Cargo.toml').write_text(f'''[package]
name = "rusty-gpui-release-consumer"
version = "0.0.0"
edition = "2024"
publish = false
[workspace]
[profile.dev]
debug = 0
[dependencies]
rusty-gpui = "={version}"
''')
# Exercise the complete packaged app, including assets, initialization, entities,
# editing and overlays, with the facade as the consumer's only dependency.
starter = registry / f'rusty-gpui-{version}.crate'
with tarfile.open(starter) as tar:
    source = tar.extractfile(f'rusty-gpui-{version}/examples/minimal.rs').read().decode()
    # Check the copyable examples as a real consumer, not just workspace targets.
    if (consumer / 'examples').exists():
        shutil.rmtree(consumer / 'examples')
    (consumer / 'examples').mkdir(exist_ok=True)
    for member in tar.getmembers():
        path = Path(member.name)
        if path.parent.as_posix() == f'rusty-gpui-{version}/examples' and path.suffix == '.rs' and path.name != 'minimal.rs':
            (consumer / 'examples' / path.name).write_bytes(tar.extractfile(member).read())
(consumer / 'src' / 'main.rs').write_text(source.replace('use gpui::', 'use rusty_gpui::gpui;\nuse gpui::', 1))
(consumer / 'src' / 'lib.rs').write_text('''use rusty_gpui::gpui;
use gpui::{prelude::*, *};
use rusty_gpui::prelude::{input, stack, button, RustyField, InputState, ActionStyle};
#[derive(IntoElement)]
pub struct Custom;
impl RenderOnce for Custom {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement { div().child("Macro API") }
}
pub fn view(state: &Entity<InputState>, cx: &App) -> impl IntoElement {
    stack(cx).child(RustyField::new("Name", input(state, cx)).description("Published API"))
        .child(button("save", "Save", ActionStyle::Primary, cx)).child(Custom)
}
''')
# Resolve without build output noise, then inspect source provenance.
resolved = subprocess.check_output([cargo, 'metadata', '--format-version', '1', '--manifest-path', str(consumer / 'Cargo.toml'), *common], cwd=consumer, text=True)
metadata = json.loads(resolved)
for name in order:
    package = next(p for p in metadata['packages'] if p['name'] == name)
    assert package['source'] and package['source'].startswith('registry+'), (name, package['source'])
assert not ({'gpui', 'gpui_http_client', 'gpui-component', 'xattr'} & {p['name'] for p in metadata['packages']})
assert 'rusty-gpui-gallery' not in {p['name'] for p in metadata['packages']}
assert not any(p['name'].startswith('form-') for p in metadata['packages'])
assert all(p['source'] and p['source'].startswith('registry+') for p in metadata['packages'] if p['name'] != 'rusty-gpui-release-consumer')
(work / 'consumer-metadata.json').write_text(resolved)
if not args.skip_consumer:
    run([cargo, 'check', '--all-targets', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'target'), *common], cwd=consumer)
    run([cargo, 'test', '--doc', '-p', 'rusty-gpui', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'target'), *common], cwd=consumer)
    if args.build_consumer:
        run([cargo, 'build', '--bins', '--examples', '--manifest-path', str(consumer / 'Cargo.toml'), '--target-dir', str(root / 'target'), *common], cwd=consumer)
repository = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['package'].get('repository')
report.write_text(json.dumps({
    'project': 'Rusty GPUI', 'version': version, 'published': False,
    'repository': repository, 'repository_url_set': bool(repository),
    'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
    'validation': {'package_manifests_and_assets': 'passed',
                   'isolated_registry_resolution': 'passed',
                   'isolated_registry_consumer': 'skipped' if args.skip_consumer else 'passed',
                   'packaged_quickstart_doctest': 'skipped' if args.skip_consumer else 'passed',
                   'packaged_examples': 'skipped' if args.skip_consumer else 'passed',
                   'packaged_starter_build': 'passed' if args.build_consumer else 'skipped'},
    'artifacts': artifacts,
}, indent=2) + '\n')
print(f'Packaged {", ".join(order)} {version}; isolated registry consumer resolved without path dependencies or patches.')
print(f'Validation evidence: {report}')
print('Nothing was published. Registry-name availability and remote publication remain separate steps.')
