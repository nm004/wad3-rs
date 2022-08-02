use std::{io::{Write, self}, marker::PhantomData};
use super::DirEntryName;

pub(super) struct WadDirEntry<'a> {
    data_offset: u32,
    disk_size: PhantomData<u32>,
    size: u32,
    entry_type: WadDirEntryType,
    compression: PhantomData<u8>,
    padding: PhantomData<[u8; 2]>,
    name: &'a DirEntryName
}

impl<'a> WadDirEntry<'a> {
    pub(super) const fn new(data_offset: u32, size: u32, entry_type: WadDirEntryType, name: &'a DirEntryName) -> Self {
        Self {
            data_offset,
            disk_size: PhantomData,
            size,
            entry_type,
            compression: PhantomData,
            padding: PhantomData,
            name,
        }
    }

    pub(super) const fn data_offset(&self) -> u32 {
        self.data_offset
    }
    const fn disk_size(&self) -> u32 {
        self.size
    }
    pub(super) const fn size(&self) -> u32 {
        self.size
    }
    const fn entry_type(&self) -> WadDirEntryType {
        self.entry_type
    }
    const fn compression(&self) -> u8 {
        0
    }
    const fn padding(&self) -> [u8; 2] {
        [0; 2]
    }
    const fn name(&self) -> &DirEntryName {
        self.name
    }
    pub(super) fn write(&self, write: &mut impl Write) -> Result<usize, io::Error> {
        write.write(&self.data_offset().to_le_bytes())?;
        write.write(&self.disk_size().to_le_bytes())?;
        write.write(&self.size().to_le_bytes())?;
        write.write(&self.entry_type().as_u8().to_le_bytes())?;
        write.write(&self.compression().to_le_bytes())?;
        write.write(&self.padding())?;
        write.write(&self.name())
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub(super) enum WadDirEntryType {
    MipMap = 0x43,
}

impl WadDirEntryType {
    const fn as_u8(&self) -> u8 {
        *self as u8
    }
}
