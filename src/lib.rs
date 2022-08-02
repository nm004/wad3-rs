mod dir;
mod texture;

use dir::{WadDirEntry, WadDirEntryType};
use std::{
    collections::BTreeMap,
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
};
use tinyvec::ArrayVec;

pub use texture::WadData;
type DirEntryName = ArrayVec<[u8; 16]>;

#[repr(transparent)]
pub struct Wad<'a>(BTreeMap<DirEntryName, WadData<'a>>);

impl<'a> Wad<'a> {
    pub fn new(data: BTreeMap<DirEntryName, WadData<'a>>) -> Self
    {
        Wad(data)
    }

    pub fn into_inner(self) -> BTreeMap<DirEntryName, WadData<'a>> {
        self.0
    }

    pub fn save(&self, write: &mut impl Write) -> Result<(), io::Error> {
        let directory = self.0.iter().scan(WadHeader::size(), |s, d| {
            let e = match d.1 {
                WadData::MipMap(m) => WadDirEntry::new(*s, m.size(), WadDirEntryType::MipMap, d.0),
            };
            *s += e.size();
            Some(e)
        });

        WadHeader::new(
            directory.clone().count().try_into().unwrap(),
            if let Some(e) = directory.clone().last() {
                e.data_offset() + e.size()
            } else {
                0
            },
        )
        .write(write)?;

        for (n, d) in &self.0 {
            d.write(write, n)?;
        }

        for e in directory {
            e.write(write)?;
        }

        Ok(())
    }
}

struct WadHeader {
    signature: PhantomData<[u8; 4]>,
    dir_count: u32,
    dir_offset: u32,
}

impl WadHeader {
    const fn new(dir_count: u32, dir_offset: u32) -> Self {
        Self {
            signature: PhantomData,
            dir_count,
            dir_offset,
        }
    }
    const fn size() -> u32 {
        (size_of::<[u8; 4]>() + size_of::<u32>() + size_of::<u32>()) as u32
    }
    const fn signature(&self) -> [u8; 4] {
        *b"WAD3"
    }
    const fn dir_count(&self) -> u32 {
        self.dir_count
    }
    const fn dir_offset(&self) -> u32 {
        self.dir_offset
    }

    fn write(&self, write: &mut impl Write) -> Result<usize, io::Error> {
        write.write(&self.signature())?;
        write.write(&self.dir_count().to_le_bytes())?;
        write.write(&self.dir_offset().to_le_bytes())
    }
}
