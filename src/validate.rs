pub fn valid_path(path: &str) -> bool {
    !path.is_empty() && path.starts_with('/')
}

pub fn valid_query(query: Option<&str>) -> bool {
    query.is_none_or(|query| !query.is_empty())
}

pub fn valid_version(version: &str) -> bool {
    matches!(version, "HTTP/1.0" | "HTTP/1.1")
}

pub fn valid_header_name(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|b| {
            b.is_ascii_alphanumeric()
                || matches!(
                    b,
                    b'!' | b'#'
                        | b'$'
                        | b'%'
                        | b'&'
                        | b'\''
                        | b'*'
                        | b'+'
                        | b'-'
                        | b'.'
                        | b'^'
                        | b'_'
                        | b'`'
                        | b'|'
                        | b'~'
                )
        })
}

pub fn valid_header_value(value: &str) -> bool {
    !value.contains('\r') && !value.contains('\n')
}
