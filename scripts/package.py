#!/usr/bin/env python3
"""Package native source only; never include build caches or host-specific files."""
from pathlib import Path
from zipfile import ZipFile, ZIP_DEFLATED
import argparse

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('output', nargs='?', default=str(root.parent / 'rusty-gpui-source.zip'))
args = parser.parse_args()
include_dirs = ('crates', 'apps', 'vendor', 'scripts', 'tests', '.github')
include_files = ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.gitignore', 'README.md', 'HANDOFF.md', 'LICENSE', 'Open Rusty GPUI.command', 'CHANGELOG.md', 'CONTRIBUTING.md', 'RELEASING.md')
docs = ('components.md', 'dependencies.md', 'review.md', 'validation.md', 'changes-0.1.1.md', 'changes-0.1.2.md', 'changes-0.1.3.md', 'changes-0.1.4.md', 'fields.md', 'architecture.md', 'changes-0.2.0.md', 'changes-0.3.0.md', 'changes-0.4.0.md', 'release-validation-0.4.0.json', 'dependency-audit.md', 'dependencies-apple-silicon.json', 'native-light.png', 'native-dark.png', 'native-modal.png', 'native-lab.png', 'native-drawer.png')
files = [root / p for p in include_files]
files += [p for name in include_dirs for p in (root / name).rglob('*') if p.is_file() and '__pycache__' not in p.parts and not p.name.startswith('.cargo')]
files += [root / 'docs' / name for name in docs if (root / 'docs' / name).exists()]
with ZipFile(args.output, 'w', ZIP_DEFLATED) as archive:
    for path in sorted(set(files)):
        archive.write(path, Path('rusty-gpui') / path.relative_to(root))
print(f'{args.output}: {len(files)} source and review files')
