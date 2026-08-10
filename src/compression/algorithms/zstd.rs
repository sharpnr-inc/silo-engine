use std::io::{Read, Write};

fn compress<R: Read, W: Write>(reader: R, writer: W) -> Result<(), std::io::Error> {
    let mut encoder = zstd::stream::Encoder::new(writer, 3)?;

    std::io::copy(&mut std::io::BufReader::new(reader), &mut encoder)?;

    encoder.finish()?;

    Ok(())
}

fn decompress<R: Read, W: Write>(reader: R, writer: W) -> Result<(), std::io::Error> {
    let mut decoder = zstd::stream::Decoder::new(reader)?;

    std::io::copy(&mut decoder, &mut std::io::BufWriter::new(writer))?;

    Ok(())
}

crate::define_compression_algorithm!(ZstdCompressor, ZstdDecompressor, compress, decompress);
