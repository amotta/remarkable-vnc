import sys
import time

x, y, pressure, zero = 50, 1800, 90, 0
time_bytes = b'\x69\xac\x3a\x63\xd8\x58\x05\x00'
out = sys.stdout.buffer

# EventType.SYN   SynEventCode.SYN_REPORT        0
out.write(time_bytes); out.write(b'\x00\x00'); out.write(b'\x00\x00'); out.write(zero.to_bytes(length=4, byteorder='little'));
# EventType.ABS   AbsEventCode.MT_TRACKING_ID    1
tracking = int(time.time()) % 0x0000ffff
out.write(time_bytes); out.write(b'\x03\x00'); out.write(b'\x39\x00'); out.write(tracking.to_bytes(length=4, byteorder='little'));
# EventType.ABS   AbsEventCode.MT_POSITION_X     118
out.write(time_bytes); out.write(b'\x03\x00'); out.write(b'\x35\x00'); out.write(x.to_bytes(length=4, byteorder='little'));
# EventType.ABS   AbsEventCode.MT_POSITION_Y     1795
out.write(time_bytes); out.write(b'\x03\x00'); out.write(b'\x36\x00'); out.write(y.to_bytes(length=4, byteorder='little'));
# EventType.ABS   AbsEventCode.MT_PRESSURE       90
out.write(time_bytes); out.write(b'\x03\x00'); out.write(b'\x3a\x00'); out.write(pressure.to_bytes(length=4, byteorder='little'));
# EventType.SYN   SynEventCode.SYN_REPORT        0
out.write(time_bytes); out.write(b'\x00\x00'); out.write(b'\x00\x00'); out.write(zero.to_bytes(length=4, byteorder='little'));
# EventType.ABS   AbsEventCode.MT_TRACKING_ID    4294967295
tracking = 4294967295
out.write(time_bytes); out.write(b'\x03\x00'); out.write(b'\x39\x00'); out.write(tracking.to_bytes(length=4, byteorder='little'));
# EventType.SYN   SynEventCode.SYN_REPORT        0
out.write(time_bytes); out.write(b'\x00\x00'); out.write(b'\x00\x00'); out.write(zero.to_bytes(length=4, byteorder='little'));

out.flush()
