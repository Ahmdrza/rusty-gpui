#!/usr/bin/env python3
"""Optional Linux/X11 integration smoke test for the real native window.

Requires a running gallery launched with RUSTY_GPUI_SMOKE_STATE pointing at STATE,
ImageMagick, tesseract, X11 and XTest. Not required to run Rusty GPUI on macOS.
"""
import ctypes as C
import csv
import io
import json
import os
from pathlib import Path
import re
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
STATE = Path(os.environ.get('RUSTY_GPUI_SMOKE_STATE', ROOT / '.smoke/state.json'))
SHOT = ROOT / '.smoke/current.png'
SHOT.parent.mkdir(exist_ok=True)
(ROOT / 'target/native-review').mkdir(parents=True, exist_ok=True)
X = C.CDLL('libX11.so.6')
XT = C.CDLL('libXtst.so.6')
X.XOpenDisplay.argtypes = [C.c_char_p]
X.XOpenDisplay.restype = C.c_void_p
X.XFlush.argtypes = [C.c_void_p]
X.XKeysymToKeycode.argtypes = [C.c_void_p, C.c_ulong]
X.XKeysymToKeycode.restype = C.c_uint
XT.XTestFakeMotionEvent.argtypes = [C.c_void_p, C.c_int, C.c_int, C.c_int, C.c_ulong]
XT.XTestFakeButtonEvent.argtypes = [C.c_void_p, C.c_uint, C.c_int, C.c_ulong]
XT.XTestFakeKeyEvent.argtypes = [C.c_void_p, C.c_uint, C.c_int, C.c_ulong]
display = X.XOpenDisplay(None)
if not display:
    raise SystemExit('No X11 display available')

def state():
    try:
        return json.loads(STATE.read_text())
    except (OSError, ValueError):
        return {}

def wait_for(predicate, message, timeout=8):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate(state()):
            print('PASS:', message, flush=True)
            return
        time.sleep(.08)
    raise AssertionError(message + ': ' + json.dumps(state()))

def capture(path=SHOT):
    subprocess.run(['import','-window','Rusty GPUI — Component Library',str(path)],check=True)

def words():
    capture()
    rows = []
    for mode in ['6','11']:
        result = subprocess.run(['tesseract',str(SHOT),'stdout','--psm',mode,'tsv'],capture_output=True,text=True,check=True)
        rows.extend(r for r in csv.DictReader(io.StringIO(result.stdout),delimiter='\t') if r.get('text','').strip())
    return rows

def normalize(value):
    return re.sub(r'[^a-z0-9]','',value.lower())

def position(label, region=None):
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        found = position_once(label, region)
        if found:
            return found
        time.sleep(.2)
    raise AssertionError('Text not found in native capture: ' + label)

def position_once(label, region=None):
    wanted = normalize(label)
    rows = words()
    for index in range(len(rows)):
        for length in range(1,len(label.split())+3):
            group = rows[index:index+length]
            if normalize(''.join(r['text'] for r in group)) != wanted:
                continue
            x = int(group[0]['left']); y = int(group[0]['top'])
            if region and not region(x,y):
                continue
            right = max(int(r['left'])+int(r['width']) for r in group)
            bottom = max(int(r['top'])+int(r['height']) for r in group)
            return (x+right)//2,(y+bottom)//2
    return None

def click(x,y):
    info = subprocess.run(['xwininfo','-name','Rusty GPUI — Component Library'],capture_output=True,text=True,check=True).stdout
    ox = int(re.search(r'Absolute upper-left X:\s*(-?\d+)',info).group(1))
    oy = int(re.search(r'Absolute upper-left Y:\s*(-?\d+)',info).group(1))
    XT.XTestFakeMotionEvent(display,-1,x+ox,y+oy,0)
    XT.XTestFakeButtonEvent(display,1,1,0)
    XT.XTestFakeButtonEvent(display,1,0,0)
    X.XFlush(display)
    time.sleep(.2)

def click_text(label,region=None):
    click(*position(label,region))

def key(sym):
    code = X.XKeysymToKeycode(display,sym)
    XT.XTestFakeKeyEvent(display,code,1,0)
    XT.XTestFakeKeyEvent(display,code,0,0)
    X.XFlush(display)
    time.sleep(.15)

def category(label,index):
    click_text(label,lambda x,y:x<210)
    wait_for(lambda s:s.get('category')==index,'native category: '+label)

def run():
    wait_for(lambda s:s.get('theme',{}).get('mode')=='light','native startup renders light theme')
    capture(ROOT/'target/native-review/native-light.png')
    category('Actions',2)
    baseline_clicks = state().get('clicks',0)
    click_text('Create workspace',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('clicks')==baseline_clicks+1,'native action callback')
    click(355,412) # Disabled control in the canonical comfortable Actions specimen.
    wait_for(lambda s:s.get('clicks')==baseline_clicks+1,'disabled action does not dispatch')
    click(500,198) # Focus the header search field, immediately before action buttons.
    key(0xff09) # Tab to the first action button.
    key(0x20) # Space dispatches its keyboard click.
    wait_for(lambda s:s.get('clicks')==baseline_clicks+2,'native keyboard button activation with Tab and Space')

    category('Selection',4)
    expected_checked = not state().get('checked', True)
    click_text('Assign to me',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('checked') == expected_checked,'native checkbox state')
    expected_switched = not state().get('switched', True)
    click_text('Private workspace',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('switched') == expected_switched,'native switch state')
    click_text('Team',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('radio')==1,'native radio state')

    category('Navigation',5)
    click_text('Activity',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('tab')==1,'native tab state')
    expected_page = min(4, state().get('page',0)+1)
    click_text('Next',lambda x,y:210<x<1040 and y>400)
    wait_for(lambda s:s.get('page')==expected_page,'native pagination state')

    category('Overlays',6)
    click_text('Open modal',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('dialog') is True,'native modal opens')
    capture(ROOT/'target/native-review/native-modal.png')
    key(0xff1b) # Escape
    wait_for(lambda s:s.get('dialog') is False,'native modal Escape closes')
    click_text('Open drawer',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('sheet') is True,'native drawer opens')
    key(0xff1b)
    wait_for(lambda s:s.get('sheet') is False,'native drawer Escape closes')

    category('Performance lab',9)
    click_text('10000 rows',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('rows')==10000,'native 10,000-row virtualized table')
    click_text('Sample 120 frames',lambda x,y:210<x<1040)
    wait_for(lambda s:s.get('remaining')==0 and '120 samples' in s.get('benchmark',''),'native frame sampling completes',timeout=25)
    capture(ROOT/'target/native-review/native-lab.png')

    click_text('Dark',lambda x,y:x>1040)
    wait_for(lambda s:s.get('theme',{}).get('mode')=='dark','native dark appearance')
    capture(ROOT/'target/native-review/native-dark.png')
    print('Native pointer/state/overlay/benchmark smoke checks passed.',flush=True)

if __name__ == '__main__':
    run()
