#!/usr/bin/env python3
"""Regression checks for nested scrolling, functional search and full-height sheets.

Run against a freshly launched gallery with RUSTY_GPUI_SMOKE_STATE set, under X11.
Uses the same optional native test dependencies as native_smoke.py.
"""
import ctypes as C
import re
import subprocess
import time
from PIL import Image
from native_smoke import ROOT, SHOT, X, XT, display, state, wait_for, capture, click, click_text, category, key, position

def window_info():
    text = subprocess.run(['xwininfo','-name','Rusty GPUI — Component Library'],capture_output=True,text=True,check=True).stdout
    return (int(re.search(r'Window id: (0x[0-9a-f]+)',text).group(1),16),
            int(re.search(r'Absolute upper-left X:\s*(-?\d+)',text).group(1)),
            int(re.search(r'Absolute upper-left Y:\s*(-?\d+)',text).group(1)))

def wheel(x,y,button=5,count=5):
    _,ox,oy = window_info()
    XT.XTestFakeMotionEvent(display,-1,x+ox,y+oy,0)
    for _ in range(count):
        XT.XTestFakeButtonEvent(display,button,1,0)
        XT.XTestFakeButtonEvent(display,button,0,0)
        X.XFlush(display)
        time.sleep(.04)

def clear_input():
    ctrl = X.XKeysymToKeycode(display,0xffe3)
    XT.XTestFakeKeyEvent(display,ctrl,1,0)
    key(ord('a'))
    XT.XTestFakeKeyEvent(display,ctrl,0,0)
    key(0xff08)

def type_text(text):
    for character in text:
        key(ord(character))

wait_for(lambda s:s.get('theme',{}).get('mode')=='light','fresh light appearance')

category('Actions',2)
click(500,198)
type_text('drawer')
wait_for(lambda s:s.get('collections')==1,'component search finds other categories')
# The overlay controls must actually be rendered, not just matched by the model.
position('Open drawer',lambda x,y:210<x<1040)
clear_input()
category('Inputs',3)
_,label_y = position('Search',lambda x,y:210<x<1040 and y>550)
click(500,label_y+32)
type_text('beacon')
wait_for(lambda s:s.get('project_results')==['Beacon'],'project search filters visible results')
clear_input()
type_text('unknown')
wait_for(lambda s:s.get('project_results')==[],'project search empty state')
clear_input()
wait_for(lambda s:len(s.get('project_results',[]))==5,'clearing project search restores results')

category('Performance lab',9)
position('Name',lambda x,y:210<x<1040)
before = state().get('page_offset',0)
wheel(600,450,count=6)
wait_for(lambda s:s.get('table_offset',0)<-40,'wheel over table moves its rows')
assert abs(state().get('page_offset',0)-before)<1, 'table wheel moved the page'
print('PASS: table wheel leaves the page stationary',flush=True)
wheel(600,450,button=4,count=40)
wait_for(lambda s:abs(s.get('table_offset',-1))<1,'table scroll clamps at the top')
assert abs(state().get('page_offset',0)-before)<1, 'wheel escaped at table boundary'

wheel(1015,810,count=8)
wait_for(lambda s:s.get('page_offset',0)<before,'wheel outside table still scrolls the page')
time.sleep(1) # Drain queued outside-wheel events before testing a new gesture.
outside = state()['page_offset']
wheel(600,450+int(outside),button=4,count=4)
time.sleep(1)
assert abs(state()['page_offset']-outside)<1, 'table boundary chained to page'
print('PASS: boundary gestures stay inside table',flush=True)

# Make the content narrower than the fixed table columns, then exercise x scroll.
X.XResizeWindow.argtypes=[C.c_void_p,C.c_ulong,C.c_uint,C.c_uint]
identifier,_,_ = window_info()
X.XResizeWindow(display,identifier,1100,920)
X.XFlush(display)
time.sleep(.5)
category('Performance lab',9)
position('Name',lambda x,y:210<x<850)
wheel(550,450,button=7,count=8)
wait_for(lambda s:s.get('table_x',0)<0,'horizontal wheel moves table columns')
assert abs(state()['page_offset'])<1,'horizontal gesture moved page'
X.XResizeWindow(display,identifier,1280,920)
X.XFlush(display)
time.sleep(.5)

category('Overlays',6)
position('Open drawer',lambda x,y:210<x<1040)
capture(SHOT)
before_image=Image.open(SHOT).convert('RGB')
before_pixel=before_image.getpixel((500,12))
click_text('Open drawer',lambda x,y:210<x<1040)
wait_for(lambda s:s.get('sheet') is True,'drawer opens')
position('Project details',lambda x,y:x>900)
time.sleep(.3)
capture(ROOT/'docs/native-drawer.png')
after=Image.open(ROOT/'docs/native-drawer.png').convert('RGB')
after_pixel=after.getpixel((500,12))
assert sum(after_pixel)<sum(before_pixel)-30, f'top of client area has no backdrop: {before_pixel} -> {after_pixel}'
assert after.getpixel((1150,12))!=after_pixel,'drawer panel does not reach the top'
print('PASS: drawer and backdrop cover the top of the client area',flush=True)
key(0xff1b)
wait_for(lambda s:s.get('sheet') is False,'drawer dismisses with Escape')
print('Native regression checks passed.',flush=True)
