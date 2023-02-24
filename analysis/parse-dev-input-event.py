from enum import Enum
import io
import sys

class EventType(Enum):
    SYN = 0x00
    ABS = 0x03

class SynEventCode(Enum):
    SYN_REPORT = 0x00

class AbsEventCode(Enum):
    MT_SLOT         = 0x2f
    MT_TOUCH_MAJOR  = 0x30
    MT_TOUCH_MINOR  = 0x31
    MT_ORIENTATION  = 0x34
    MT_POSITION_X   = 0x35
    MT_POSITION_Y   = 0x36
    MT_TRACKING_ID  = 0x39
    MT_PRESSURE     = 0x3A

class Event:
    def __init__(self, time, type, code, data):
        self.time = time
        self.type = type
        self.code = code
        self.data = data

'''
with io.FileIO(FILE, mode='r') as f:
    data = f.readall()
'''

FILE = '-'
if len(sys.argv) >= 2:
    FILE = sys.argv[1]

if FILE == '-':
    print("Reading from stdin")
    file = sys.stdin.buffer
else:
    print(f"Reading from {FILE}")
    file = io.FileIO(FILE, mode='r')

events = list()
while True:
    event_bytes = file.read(16)
    if len(event_bytes) < 16: break

    event_time = int.from_bytes(event_bytes[ 0: 8], byteorder='little')
    event_type = int.from_bytes(event_bytes[ 8:10], byteorder='little')
    event_code = int.from_bytes(event_bytes[10:12], byteorder='little')
    event_data = int.from_bytes(event_bytes[12:16], byteorder='little')

    event_type = EventType(event_type)
    if event_type == EventType.SYN:
        event_code = SynEventCode(event_code)
    elif event_type == EventType.ABS:
        event_code = AbsEventCode(event_code)

    event = Event(event_time, event_type, event_code, event_data)

    # if (event.code == AbsEventCode.MT_POSITION_X or event.code == AbsEventCode.MT_POSITION_Y):
    # if event.code == AbsEventCode.MT_SLOT:
    if True:
        print(f"{event.time:<15} {event.type:<15} {event.code:<30} {event.data}")
