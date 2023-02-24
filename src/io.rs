pub trait ReadBytesExt: std::io::Read {
    #[inline]
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_u16_from_be(&mut self) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    #[inline]
    fn read_u32_from_be(&mut self) -> std::io::Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    #[inline]
    fn read_i32_from_be(&mut self) -> std::io::Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
}

impl<R: std::io::Read + ?Sized> ReadBytesExt for R {}

pub trait WriteBytesExt: std::io::Write {
    #[inline]
    fn write_u8(&mut self, v: u8) -> std::io::Result<()> {
        self.write_all(&[v])
    }

    #[inline]
    fn write_u16_as_be(&mut self, v: u16) -> std::io::Result<()> {
        let buf = v.to_be_bytes();
        self.write_all(&buf)
    }

    #[inline]
    fn write_u32_as_be(&mut self, v: u32) -> std::io::Result<()> {
        let buf = v.to_be_bytes();
        self.write_all(&buf)
    }

    #[inline]
    fn write_i32_as_be(&mut self, v: i32) -> std::io::Result<()> {
        let buf = v.to_be_bytes();
        self.write_all(&buf)
    }
}

impl<W: std::io::Write + ?Sized> WriteBytesExt for W {}
