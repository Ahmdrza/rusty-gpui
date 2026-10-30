#!/usr/bin/env python3
"""Set the GitHub repository URL in publishable package metadata."""
import argparse
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('url', help='https://github.com/OWNER/rusty-gpui')
args = parser.parse_args()
url = args.url.rstrip('/').removesuffix('.git')
if not re.fullmatch(r'https://github\.com/[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', url):
    parser.error('Use a GitHub HTTPS owner/repository URL.')
root = Path(__file__).resolve().parents[1]
p = root / 'Cargo.toml'
s = p.read_text()
a = s.index('[workspace.package]')
b = s.index('\n[', a + 1)
section = s[a:b]
if re.search(r'^repository\s*=', section, re.M):
    section = re.sub(r'^repository\s*=.*$', f'repository = "{url}"', section, flags=re.M)
else:
    section = section.rstrip() + f'\nrepository = "{url}"\n'
p.write_text(s[:a] + section + s[b:])
for name in ['rusty-gpui-http', 'rusty-gpui-runtime', 'rusty-gpui-theme', 'rusty-gpui-components', 'rusty-gpui']:
    p = root / 'crates' / name / 'Cargo.toml'
    s = p.read_text()
    if 'repository.workspace = true' not in s:
        s = s.replace('[package]\n', '[package]\nrepository.workspace = true\n', 1)
    p.write_text(s)
print(f'Repository metadata set to {url}. Review the changes before committing.')
