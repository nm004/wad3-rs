use crate::DirEntryName;
use rgb::RGB8;
use tinyvec::ArrayVec;
use std::{
    borrow::Cow,
    io::{self, Write},
    marker::PhantomData,
    mem::size_of,
};

type WadPalette = ArrayVec<[RGB8;256]>;

pub enum WadData<'a> {
    MipMap(MipMap<'a>),
}

impl<'a> WadData<'a> {
    pub const fn new_mipmap(
        width: u32,
        height: u32,
        data: (Cow<'a, [u8]>, Cow<'a, [u8]>, Cow<'a, [u8]>, Cow<'a, [u8]>),
        palette: WadPalette,
    ) -> Self {
        let m = MipMap {
            _name: PhantomData,
            width,
            height,
            data_offset: PhantomData,
            data,
            palette_count: PhantomData,
            palette,
            padding: PhantomData,
        };
        Self::MipMap(m)
    }

    pub(super) fn write(
        &self,
        write: &mut impl Write,
        name: &DirEntryName
    ) -> Result<usize, io::Error> {
        match self {
            WadData::MipMap(m) => m.write(write, name),
        }
    }
}

pub struct MipMap<'a> {
    _name: PhantomData<DirEntryName>,
    width: u32,
    height: u32,
    data_offset: PhantomData<(u32, u32, u32, u32)>,
    data: (Cow<'a, [u8]>, Cow<'a, [u8]>, Cow<'a, [u8]>, Cow<'a, [u8]>),
    palette_count: PhantomData<u16>,
    palette: WadPalette,
    padding: PhantomData<[u8; 2]>,
}

impl<'a> MipMap<'a> {
    const fn width(&self) -> u32 {
        self.width
    }
    const fn height(&self) -> u32 {
        self.height
    }
    fn data_offset(&self) -> (u32, u32, u32, u32) {
        let wh = self.width() * self.height();

        let o0 = 40;
        let o1 = o0 + wh as u32;
        let o2 = o1 + wh / 4 as u32;
        let o3 = o2 + wh / 16 as u32;

        (o0, o1, o2, o3)
    }
    fn data(&self) -> &(Cow<[u8]>, Cow<[u8]>, Cow<[u8]>, Cow<[u8]>) {
        &self.data
    }
    const fn palette_count(&self) -> u16 {
        (size_of::<WadPalette>() / size_of::<RGB8>()) as u16
    }
    fn palette(&self) -> WadPalette {
        self.palette
    }
    const fn padding(&self) -> [u8; 2] {
        [0; 2]
    }

    pub(super) fn size(&self) -> u32 {
        (size_of::<[u8; 16]>()
            + size_of::<u32>()
            + size_of::<u32>()
            + size_of::<(u32, u32, u32, u32)>()
            + &self.data.0.len()
            + &self.data.1.len()
            + &self.data.2.len()
            + &self.data.3.len()
            + size_of::<u16>()
            + size_of::<WadPalette>()
            + size_of::<[u8; 2]>()) as u32
    }

    fn write(
        &self,
        write: &mut impl Write,
        name: &DirEntryName,
    ) -> Result<usize, io::Error> {
        write.write(&name)?;
        write.write(&self.width().to_le_bytes())?;
        write.write(&self.height().to_le_bytes())?;
        write.write(&self.data_offset().0.to_le_bytes())?;
        write.write(&self.data_offset().1.to_le_bytes())?;
        write.write(&self.data_offset().2.to_le_bytes())?;
        write.write(&self.data_offset().3.to_le_bytes())?;
        write.write(&self.data().0)?;
        write.write(&self.data().1)?;
        write.write(&self.data().2)?;
        write.write(&self.data().3)?;
        write.write(&self.palette_count().to_le_bytes())?;
        write.write(&palette_to_u8_slice(self.palette().as_ref()))?;
        write.write(&self.padding())
    }
}

fn palette_to_u8_slice(palette: &[RGB8]) -> Box<[u8]> {
    palette.iter().map(|i| [i.r, i.g, i.b]).flatten().collect()
}

//                 Texture::Unknown(u) => (
//                     (size_of::<[u8; 16]>()
//                         + size_of::<u32>()
//                         + size_of::<u32>()
//                         + size_of::<(u32, u32, u32, u32)>()
//                         + u.qpic.image.len()
//                         + u.mips.mip1.len()
//                         + u.mips.mip2.len()
//                         + u.mips.mip3.len()
//                         + size_of::<[u8; 2]>()
//                         + size_of::<[u8; 2]>()) as u32,
//                     EntryType::Unknown,
//                 ),
//                 Texture::StatusBar(s) => (
//                     (size_of::<u32>()
//                         + size_of::<u32>()
//                         + s.image.0.len()
//                         + size_of::<[u8; 2]>()
//       2                  + size_of::<[u8; 2]>()) as u32,
//                     EntryType::StatusBar,
//                 ),
//                 Texture::Font(f) => (
//                     (size_of::<[u8; 16]>()
//                         + size_of::<u32>()
//                         + size_of::<u32>()
//                         + size_of::<u32>()
//                         + size_of::<u32>()
//                         + size_of::<[u32; PALETTE_MAX_COUNT * 2]>()
//                         + f.image.len()
//                         + size_of::<[u8; 2]>()
//                         + size_of::<[u8; 2]>()) as u32,
//                     EntryType::Font,
//                 ),

//             Texture::Font(f) => {
// 		    write.write(&(k.to_owned() + &"\0".repeat(16)).as_bytes()[..16])?;
//                 write.write(&f.size.width.to_le_bytes())?;
//                 write.write(&f.size.height.to_le_bytes())?;
//                 write.write(&f.row_data.count.to_le_bytes())?;
//                 write.write(&f.row_data.height.to_le_bytes())?;
//                 write.write(
//                     &f.image
//                         .0
//                         .iter()
//                         .map(|i| i.0)
//                         .collect::<Box<_>>(),
//                 )?;
//                 write.write(&f.char_data.len().to_le_bytes())?;
//                 // write.write(
//                 //     &f.char_data
//                 //         .iter()
//                 //         .map(|i|  [i.0, i.1, i.2])
//                 //         // .flatten()
//                 //         .collect::<Box<_>>(),
//                 // )?;
//                 write.write(&[2; 0])?; // padding
// 		}
