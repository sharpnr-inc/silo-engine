#[macro_export]
macro_rules! define_compression_algorithm {
    (
        $compressor:ident,
        $decompressor:ident,
        $compress_fn:path,
        $decompress_fn:path
    ) => {
        pub struct $compressor;

        pub struct $decompressor;

        impl crate::compression::Compressor for $compressor {
            fn compress<R: std::io::Read, W: std::io::Write>(
                &self,
                reader: R,
                writer: W,
            ) -> Result<(), crate::compression::CompressionError> {
                $compress_fn(reader, writer).map_err(|error| {
                    crate::compression::CompressionError::CompressionFailed(error.to_string())
                })
            }
        }

        impl crate::compression::Decompressor for $decompressor {
            fn decompress<R: std::io::Read, W: std::io::Write>(
                &self,
                reader: R,
                writer: W,
            ) -> Result<(), crate::compression::CompressionError> {
                $decompress_fn(reader, writer).map_err(|error| {
                    crate::compression::CompressionError::DecompressionFailed(error.to_string())
                })
            }
        }
    };
}
