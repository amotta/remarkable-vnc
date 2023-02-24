#!/usr/bin/env bash
python toggle-menu.py | ssh rm2-tablet 'cat - > /dev/input/event2'
