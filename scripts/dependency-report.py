#!/usr/bin/env python3
"""Report the platform-selected dependency graph, without building the app."""
import argparse
import collections
import json
import os
import re
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--target', help='Rust target triple; defaults to the current Rust host')
parser.add_argument('--online', action='store_true', help='Allow Cargo to fetch missing registry data')
parser.add_argument('--json', action='store_true', help='Print a machine-readable report')
parser.add_argument('--why', metavar='PACKAGE', help='Show which packages depend on this package')
args = parser.parse_args()

def run(command):
    result = subprocess.run(command, cwd=root, text=True, capture_output=True)
    if result.returncode:
        parser.exit(result.returncode, result.stderr)
    return result.stdout

try:
    target = args.target or next(line.split(': ', 1)[1] for line in run(['rustc', '-vV']).splitlines() if line.startswith('host: '))
    base = [os.environ.get('CARGO', 'cargo'), 'tree', '--locked', '--target', target, '--edges', 'normal,build']
    if not args.online:
        base.append('--offline')
    if args.why:
        print(run(base + ['-p', 'rusty-gpui-gallery', '--invert', args.why]), end='')
        raise SystemExit(0)

    # Read one app-selected graph. Selecting the runtime as a separate root would
    # activate its defaults and inflate the subtree with unused backend features.
    lines = run(base + ['-p', 'rusty-gpui-gallery', '--prefix', 'depth', '--format', '{p}']).splitlines()
    edges = collections.defaultdict(set)
    all_names = set()
    stack = []
    for line in lines:
        match = re.match(r'^(\d+)(.+ v\S+)', line)
        if not match: continue
        depth = int(match[1])
        package = match[2].removesuffix(' (*)').split(' (', 1)[0]
        all_names.add(package)
        stack = stack[:depth]
        if stack: edges[stack[-1]].add(package)
        stack.append(package)
    runtime = next(name for name in all_names if name.startswith('rusty-gpui-runtime v'))
    gpui_names = set()
    pending = [runtime]
    while pending:
        package = pending.pop()
        if package in gpui_names: continue
        gpui_names.add(package)
        pending.extend(edges[package])
    all_packages = sorted(all_names)
    gpui_packages = sorted(gpui_names)
    versions = collections.defaultdict(list)
    for package in all_packages:
        name, version = package.rsplit(' v', 1)
        versions[name].append(version)
    report = {
        'target': target,
        'package_versions': len(all_packages),
        'gpui_subtree': len(gpui_packages),
        'outside_gpui': len(set(all_packages) - set(gpui_packages)),
        'multiple_versions': {name: values for name, values in versions.items() if len(values) > 1},
        'packages': all_packages,
        'count_definition': 'Unique name/version pairs in cargo tree normal+build edges; includes the gallery and host build tools. Not compilation jobs, binary contents, or installed applications.',
    }
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        print(f'Target: {target}')
        print(f'Unique package versions: {report["package_versions"]}')
        print(f'Under GPUI: {report["gpui_subtree"]}; outside GPUI: {report["outside_gpui"]}')
        print(f'Names with multiple versions: {len(report["multiple_versions"])}')
        print(report['count_definition'])
except FileNotFoundError as error:
    parser.exit(1, f'{error.filename} was not found. Install Rust and ensure cargo/rustc are on PATH.\n')
