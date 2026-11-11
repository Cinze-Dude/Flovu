use std::fmt;

pub enum MediaType {
    Plain,
    Html,
    Css,
    Js,
    Csv,
    Xml,
    Json,
    Gif,
    Avif,
    Webp,
    XIcon,
    Bmp,
    Tiff,
    Svg,
    Mpeg,
    Ogg,
    Wav,
    Mp4,
    Webm,
    Pdf,
    Zip,
    Octet,
    Custom(String),
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            MediaType::Plain => "text/plain",
            MediaType::Html => "text/html",
            MediaType::Css => "text/css",
            MediaType::Js => "text/javascript",
            MediaType::Csv => "text/csv",
            MediaType::Xml => "text/xml",
            MediaType::Json => "application/json",

            MediaType::Gif => "image/gif",
            MediaType::Avif => "image/avif",
            MediaType::Webp => "image/webp",
            MediaType::XIcon => "image/x-icon",
            MediaType::Bmp => "image/bmp",
            MediaType::Tiff => "image/tiff",
            MediaType::Svg => "image/svg+xml",

            MediaType::Mpeg => "audio/mpeg",
            MediaType::Ogg => "audio/ogg",
            MediaType::Wav => "audio/wav",

            MediaType::Mp4 => "video/mp4",
            MediaType::Webm => "video/webm",

            MediaType::Pdf => "application/pdf",
            MediaType::Zip => "application/zip",
            MediaType::Octet => "application/octet-stream",

            MediaType::Custom(value) => value,
        };

        write!(f, "{value}")
    }
}
