use std::io::{Read, Cursor};

use ureq;

pub use zip::ZipArchive;

pub type Zipped = ZipArchive<Cursor<Vec<u8>>>;

pub fn basic_hash(data: &[u8]) -> u32 {
    adler32::adler32(data).unwrap_or(0)
}

/// given a valid region, we form a url like:
/// https://schema.cloudformation.{region}.amazonaws.com/CloudformationSchema.zip
/// and fetch and return the zip file as a vec of bytes.
/// optionally, if cache is set to true, we will:
/// - try to read the file from cache if it exists, if not: make url request.
/// - if we made a url request, we save the file to disk.
pub fn get_cfn_resource_provider_schema(
    region: &str,
    cache: bool,
) -> Result<Zipped, String> {
    let url = format!("https://schema.cloudformation.{region}.amazonaws.com/CloudformationSchema.zip");
    let url_hash = basic_hash(url.as_bytes());
    let file_name = format!("{url_hash}.zip");
    if cache {
        match std::fs::File::open(&file_name) {
            Ok(mut f) => {
                // if we're able to read it then we're good.
                // but if we fail, then we'll try to make a request.
                let mut bytes = vec![];
                if let Ok(_) = f.read_to_end(&mut bytes) {
                    return Ok(bytes_to_zip(bytes)?);
                }
            }
            Err(_) => {
                // we assume it doesnt exist, so we just make a request instead.
            }
        }
    }
    // either cache is false, or we failed to read the file.
    // in any case: download it.
    let contents = download_file(&url)?;
    // if cache is set, we'd like to save this file
    if cache {
        let _ = std::fs::write(&file_name, &contents);
    }
    bytes_to_zip(contents)
}

pub fn download_file(
    url: &str,
) -> Result<Vec<u8>, String> {
    let body = ureq::get(url)
        .call().map_err(|e| format!("Failed to make http request to {}\n{:?}", url, e))?;
    let mut data_reader = body.into_reader();
    let mut data = vec![];
    data_reader.read_to_end(&mut data)
        .map_err(|e| format!("Failed to read request bytes of {}\n{:?}", url, e))?;
    Ok(data)
}

pub fn bytes_to_zip(
    bytes: Vec<u8>,
) -> Result<Zipped, String> {
    let reader = std::io::Cursor::new(bytes);
    let archive = zip::ZipArchive::new(reader)
        .map_err(|e| format!("Failed to create zip archive\n{:?}", e))?;
    Ok(archive)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_download() {
        let contents = get_cfn_resource_provider_schema("us-east-2", true).expect("Failed");
        // bytes_to_zip(contents).expect("Failed to make zip");
    }
}
