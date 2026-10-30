#!/usr/bin/env python3
"""Measure a fresh development build without deleting any existing Cargo cache."""
import argparse
import json
import os
from pathlib import Path
import platform
import subprocess
import time

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--target-dir', type=Path, required=True, help='A new build-output directory')
parser.add_argument('--package', default='rusty-gpui-settings')
args = parser.parse_args()
target = args.target_dir.resolve()
if target.exists():
    parser.error('Choose a new target directory; existing build output is never deleted.')
target.mkdir(parents=True)
cargo = os.environ.get('CARGO', 'cargo')
command = [cargo, 'build', '--locked', '--offline', '-p', args.package,
           '--target-dir', str(target), '--message-format=json']
start = time.monotonic()
result = subprocess.run(command, cwd=root, text=True, capture_output=True,
                        env={**os.environ, "CARGO_PROFILE_DEV_DEBUG": "0"})
elapsed = time.monotonic() - start
(target / 'build.log').write_text(result.stderr)
if result.returncode:
    print(result.stderr)
    raise SystemExit(result.returncode)
executables = []
for line in result.stdout.splitlines():
    message = json.loads(line)
    if message.get('reason') == 'compiler-artifact' and message.get('executable'):
        path = Path(message['executable'])
        executables.append({'path': str(path), 'bytes': path.stat().st_size})
report = {
    'package': args.package,
    'profile': 'dev (workspace settings; no debug symbols)',
    'build_seconds': round(elapsed, 2),
    'build_cache': 'fresh target directory; dependency downloads already cached; offline',
    'platform': platform.platform(),
    'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
    'executables': executables,
    'limits': 'Development build only; not release size, startup latency or GPU performance.',
}
(target / 'measurement.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2), flush=True)
