use std::io::{Read, Write};

#[macro_use] extern crate strum_macros;

enum FormatType {
    Stream(StreamFormat),
    Archive(ArchiveFormat),
}

// a
// ar
// tar
// iso

// bz2
// gz
// lz
// lzma
// sz
// xz

// 7z
// alz
// apk
// cab
// dmg
// rar

#[derive(EnumString)]
enum StreamFormat {
    Gz,
    Lz4,
    Sz,
}


#[derive(EnumString)]
enum ArchiveFormat {
    A,
    Ar,
    Tar,
    Iso,
    Bz2,
    Lzma,
    Xz,
    P7z,
    Alz,
    Egg,
    Apk,
    Cab,
    Dmg,
    Rar,
}

fn detect_format(ext: &str) -> Option<FormatType> {
    if let Ok(format) = StreamFormat::FromStr(ext) {
        return Some(FormatType::Stream(format));
    }
    if let Ok(format) = ArchiveFormat::FromStr(ext) {
        return Some(FormatType::Archive(format));
    }
    None
}

impl StreamFormat {
    fn reader(&self, r: Box<dyn Read>) -> Box<dyn Read> {
        Box::new(match self {
            Gz=>{GzReader::new(r)},
            Lz4=>{lz4::Reader::new(r)},
            Sz=>{snap::Reader::new(r)},
            _=>todo!(),
        })
    }
    fn writer(&self, w: Box<dyn Write>) -> Box<dyn Write> {
        Box::new(match self {
            Gz=>{GzWriter::new(w)},
            Lz4=>{lz4::Writer::new(w)},
            Sz=>{snap::Writer::new(w)},
            _=>todo!(),
        })
    }
}

impl ArchiveFormat {
    fn unpack<R:Read, P:AsRef<std::path::Path>>(&self, src:R, dst: P) -> Result<(), Error>{
        match self {
            Rar => {
                unrar::Archive::new(src.str()).extract_to(dst)?;
            }
            Tar => {
                tar::Archive::new(src).unpack(dst)?;
            },
            Zip => {
                _ => todo!(),
                zip::read::read_zipfile_from_stream(src)?.unwrap().unpack(dst)?;
                // zip::ZipArchive::new(src)?.unpack(dst)?;
            },
            _ => todo!(),
        }
        ()
    }
}
