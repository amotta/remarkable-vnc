use std::io::{Read, Seek, SeekFrom, Write};
use vnc::io::WriteBytesExt;
use vnc::protocol::{self, Message};

const EXE_PATH: &str = "/usr/bin/xochitl";
const TOUCHSCREEN_PATH: &str = "/dev/input/event2";

const SCREEN_WIDTH: usize = 1404;
const SCREEN_HEIGHT: usize = 1872;
const PIXEL_BYTE_DEPTH: usize = 2;
const PIXEL_IS_BIG_ENDIAN: bool = false;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum InputEventType {
    Syn(SynEventCode),
    Abs(AbsEventCode),
}

enum SynEventCode {
    Report = 0,
}

enum AbsEventCode {
    MtTrackingId = 0x39,
    MtPositionX = 0x35,
    MtPositionY = 0x36,
    MtPressure = 0x3a,
}

struct Touchscreen {
    file: std::fs::File,
    tracking_id: u32,
}

impl Touchscreen {
    const TIME_BYTES: [u8; 8] = [0x69, 0xAC, 0x3A, 0x63, 0xD8, 0x58, 0x05, 0x00];

    pub fn new() -> Result<Self> {
        let file = std::fs::File::create(TOUCHSCREEN_PATH)?;
        Ok(Self { file, tracking_id: 6426 })
    }

    pub fn press(&mut self, x: u32, y: u32) -> Result<()> {
        self.write_event(InputEventType::Syn(SynEventCode::Report), 0)?;
        self.write_event(InputEventType::Abs(AbsEventCode::MtTrackingId), self.tracking_id)?;
        self.write_event(InputEventType::Abs(AbsEventCode::MtPositionX), x)?;
        self.write_event(InputEventType::Abs(AbsEventCode::MtPositionY), y)?;
        self.write_event(InputEventType::Abs(AbsEventCode::MtPressure), 90)?;
        self.tracking_id += 1;

        self.write_event(InputEventType::Syn(SynEventCode::Report), 0)?;
        self.write_event(InputEventType::Abs(AbsEventCode::MtTrackingId), 4294967295)?;

        self.write_event(InputEventType::Syn(SynEventCode::Report), 0)?;

        Ok(())
    }

    fn write_event(&mut self, typ: InputEventType, data: u32) -> Result<()> {
        let mut buf = [0u8; 16];

        buf[0..8].copy_from_slice(&Self::TIME_BYTES);

        match typ {
            InputEventType::Syn(c) => {
                buf[8..10].copy_from_slice(&(0x00 as u16).to_ne_bytes());
                buf[10..12].copy_from_slice(&(c as u16).to_ne_bytes());
            },
            InputEventType::Abs(c) => {
                buf[8..10].copy_from_slice(&(0x03 as u16).to_ne_bytes());
                buf[10..12].copy_from_slice(&(c as u16).to_ne_bytes());
            },
        };

        buf[12..16].copy_from_slice(&data.to_ne_bytes());
        self.file.write_all(&buf);

        Ok(())
    }
}

struct ScreenMemory {
    file: std::fs::File,
    off: usize,
    len: usize,
}

impl ScreenMemory {
    pub fn new() -> Result<Self> {
        let mut file = Self::find_and_open_exe_mem_file()?;

        let off = {
            let mut buf = [0u8; std::mem::size_of::<usize>()];
            // NOTE(amotta): This is specific to version 2.7
            file.seek(SeekFrom::Start(0x004ac7f8))?;
            file.read_exact(&mut buf)?;
            usize::from_ne_bytes(buf)
        };

        let len = SCREEN_WIDTH * SCREEN_HEIGHT * PIXEL_BYTE_DEPTH;
        Ok(ScreenMemory { file, off, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.len {
            return Err("Buffer too small".into());
        }

        self.file.seek(SeekFrom::Start(self.off as u64))?;
        self.file.read_exact(&mut buf[..self.len])?;
        Ok(())
    }

    fn find_and_open_exe_mem_file() -> Result<std::fs::File> {
        let mut dir = std::fs::read_dir("/proc")?;
        while let Some(Ok(entry)) = dir.next() {
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let pid = {
                let name_osstring = entry.file_name();
                let name_str = name_osstring.to_str().unwrap();
                usize::from_str_radix(&name_str, 10).ok()
            };

            let Some(pid) = pid else {
                continue;
            };

            // Keep handle of directory to avoid PID reuse race condition.
            let pid_dir_handle = std::fs::File::open(entry.path());

            let Ok(exe_path) = std::fs::read_link(entry.path().join("exe")) else {
                // From `man 5 proc`:
                // In a multithreaded process, the contents of this symbolic
                // link are not available if the main thread has already terminated
                // (typically by calling pthread_exit(3)).
                continue;
            };

            if exe_path.to_str().unwrap() != EXE_PATH {
                continue;
            }

            // We have found the executable. Let's open the memory.
            return std::fs::File::open(entry.path().join("mem"))
                .map_err(|_| "Could not open mem file".into());
        }

        Err("No process with searched executable".into())
    }
}

fn main() -> Result<()> {
    println!("Init");
    let mut touchscreen = Touchscreen::new()?;
    let mut screen = ScreenMemory::new()?;
    let mut buf = vec![0u8; screen.len()];
    println!("Screen memory initialized");

    let listener = std::net::TcpListener::bind(("localhost", 5900))?;
    println!("Listening for connection");

    while let (mut stream, addr) = listener.accept()? {
        println!("Got incoming connection: {:?}", addr);

        println!("Advertising protocol version");
        protocol::Version::Rfb38.write_to(&mut stream)?;

        let version = protocol::Version::read_from(&mut stream)?;
        println!("Client requested version {:?}", version);
        assert!(version == protocol::Version::Rfb38);

        println!("Advertising security types");
        let sec_types = protocol::SecurityTypes(vec![protocol::SecurityType::None]);
        sec_types.write_to(&mut stream)?;

        let sec_type = protocol::SecurityType::read_from(&mut stream)?;
        println!("Client requested security type {:?}", &sec_type);
        assert!(sec_type == protocol::SecurityType::None);

        println!("Security handshake succeeded");
        protocol::SecurityResult::Succeeded.write_to(&mut stream)?;

        let client_init = protocol::ClientInit::read_from(&mut stream)?;
        println!("Client requested shared desktop {}", client_init.shared);

        let server_pixel_format = protocol::PixelFormat {
            bits_per_pixel: (8 * PIXEL_BYTE_DEPTH) as u8,
            depth:          (8 * PIXEL_BYTE_DEPTH) as u8,
            big_endian:     PIXEL_IS_BIG_ENDIAN,
            true_colour:    true,
            red_max:        0b00011111,
            green_max:      0b00111111,
            blue_max:       0b00011111,
            red_shift:      6 + 5 + 0,
            green_shift:    5 + 0,
            blue_shift:     0,
        };

        protocol::ServerInit {
            framebuffer_width:  SCREEN_WIDTH as u16,
            framebuffer_height: SCREEN_HEIGHT as u16,
            pixel_format:       server_pixel_format,
            name:               "remarkable2".to_string(),
        }.write_to(&mut stream)?;

        while let msg = protocol::C2S::read_from(&mut stream)? {
            match msg {
                protocol::C2S::SetPixelFormat(pixel_format) => {
                    assert!(
                        pixel_format == server_pixel_format,
                        "Mismatch between client and server pixel formats"
                    );
                },
                protocol::C2S::SetEncodings(_) => {
                    // The spec says that "pixel data may always be sent in raw encoding even if
                    // not specified explicitly here." So, let's ignore this message for now.
                },
                protocol::C2S::PointerEvent { button_mask, x_position, y_position } => {
                    // Left mouse button pressed
                    if button_mask & 0x01 != 0x00 {
                        let x_position = x_position as u32;
                        // NOTE(amotta): Input event system uses reversed Y axis
                        let y_position = SCREEN_HEIGHT as u32 - y_position as u32;
                        touchscreen.press(x_position, y_position)?;
                    }
                },
                protocol::C2S::KeyEvent { .. }
                | protocol::C2S::CutText(_) => {
                    // We don't care about input events for now.
                },
                protocol::C2S::FramebufferUpdateRequest { .. } => {
                    stream.write_u8(0)?;        // message type
                    stream.write_u8(0)?;        // padding
                    stream.write_u16_as_be(1)?; // number of rectangle
                    protocol::Rectangle {
                        x_position: 0,
                        y_position: 0,
                        width:      SCREEN_WIDTH as u16,
                        height:     SCREEN_HEIGHT as u16,
                        encoding:   protocol::Encoding::Raw,
                    }.write_to(&mut stream)?;

                    screen.read(buf.as_mut_slice())?;
                    stream.write_all(buf.as_slice())?;
                }
            }
        }
    }

    Ok(())
}

