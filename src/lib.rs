use anyhow::Result;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

#[macro_use]
extern crate strum_macros;

enum FormatType {
    Stream(StreamFormat),
    Archive(ArchiveFormat),
}

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
enum StreamFormat {
    Gz,
    Lz4,
    Sz,
}

#[derive(EnumString)]
#[strum(serialize_all = "snake_case")]
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
    Zip,
}

fn detect_format(ext: &str) -> Option<FormatType> {
    if let Ok(format) = StreamFormat::from_str(ext) {
        return Some(FormatType::Stream(format));
    }
    if let Ok(format) = ArchiveFormat::from_str(ext) {
        return Some(FormatType::Archive(format));
    }
    None
}

impl StreamFormat {
    fn reader(&self, r: Box<dyn Read>) -> Box<dyn Read> {
        match self {
            Self::Gz => Box::new(flate2::read::GzDecoder::new(r)),
            Self::Lz4 => Box::new(lz4::Decoder::new(r).unwrap()),
            Self::Sz => Box::new(snap::Reader::new(r)),
            // _ => todo!(),
        }
    }
    fn writer(&self, w: Box<dyn Write>) -> Box<dyn Write> {
        match self {
            Self::Gz => Box::new(flate2::write::GzEncoder::new(
                w,
                flate2::Compression::default(),
            )),
            Self::Lz4 => Box::new(lz4::EncoderBuilder::new().build(w).unwrap()),
            Self::Sz => Box::new(snap::Writer::new(w)),
            // _ => todo!(),
        }
    }
}

impl ArchiveFormat {
    fn is_support_streaming() -> bool {
        false
    }
    fn is_support_seekable_streaming() -> bool {
        false
    }
    fn is_support_memory() -> bool {
        false
    } // std::io::Cursor(vec)
      // if small, memory-mapped

    fn unpack<P: AsRef<std::path::Path>>(&self, src: P, dst: P) -> Result<()> {
        let src = src.as_ref();
        let dst = dst.as_ref();
        match self {
            Self::Rar => {
                unrar::Archive::new(src.to_str().unwrap().to_string())
                    .extract_to(dst.to_str().unwrap().to_string())
                    .map_err(|e| anyhow::anyhow!(format!("{}", e)))?;
            }
            Self::Tar => {
                let r = std::io::BufReader::new(std::fs::File::open(src)?);
                tar::Archive::new(r).unpack(dst)?;
            }
            Self::Zip => {
                // zip::read::read_zipfile_from_stream(src)?.unwrap().unpack(dst)?;
                let f = std::io::BufReader::new(std::fs::File::open(src)?);
                let mut arc = zip::ZipArchive::new(f)?;
                for i in 0..arc.len() {
                    let zipfile = arc.by_index(i)?;
                    println!(
                        "{} {}",
                        zipfile.name(),
                        String::from_utf8_lossy(&zipfile.name_raw())
                    );
                }
            }
            _ => todo!(),
        }
        Ok(())
    }
}

pub fn decompress_once<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: Option<P2>) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref().map(|p| p.as_ref());
    let dst = match dst {
        Some(p) => p.to_path_buf(),
        None => src.with_file_name(src.file_stem().unwrap()),
    };

    match src.extension() {
        Some(ext) => {
            let ext = ext.to_str().unwrap();
            match detect_format(ext) {
                Some(FormatType::Stream(format)) => {
                    let f = Box::new(std::io::BufReader::new(std::fs::File::open(src)?));
                    let mut decoded = format.reader(f);

                    let dst_file = if dst.exists() {
                        if dst.is_file() {
                            return Err(anyhow::anyhow!("The dst {:?} already exists", dst));
                        } else {
                            // aa.gz
                            // dst/aa
                            let new_filename = src.file_stem().unwrap().to_str().unwrap();
                            dst.join(new_filename)
                        }
                    } else {
                        dst.to_path_buf()
                    };

                    let mut sink = std::io::BufWriter::new(std::fs::File::create(&dst_file)?);

                    println!("decompress:\n");
                    println!("src file: {}", src.display());
                    println!("dst file: {}", dst_file.display());
                    std::io::copy(&mut decoded, &mut sink)?;
                    println!("done");
                }
                Some(FormatType::Archive(format)) => {
                    let dst_dir = if dst.exists() {
                        if dst.is_file() {
                            return Err(anyhow::anyhow!(
                                "The dst {} already exists",
                                dst.display()
                            ));
                        } else {
                            dst.join(src.file_stem().unwrap().to_str().unwrap())
                        }
                    } else {
                        dst.to_path_buf()
                    };

                    println!("decompress:\n");
                    println!("src file: {}", src.display());
                    println!("dst  dir: {}", dst_dir.display());
                    format.unpack(src, &dst_dir)?;
                    println!("done");
                }
                None => Err(anyhow::anyhow!("no compression detected: {}", ext))?,
            }
        }
        None => Err(anyhow::anyhow!("no extension detected: {}", src.display()))?,
    }
    Ok(())
}

fn decompress_repeat() {
    todo!();
}

fn decompress_nested() {
    todo!();
}

fn decompress_nested_all() {
    todo!();
}
