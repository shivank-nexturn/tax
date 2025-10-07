use std::io::Read;
use zip::ZipArchive;
use anyhow::Result;

pub async fn extract_zip_async<R: Read + Send + 'static>(
    reader: R,
    extract_to: &str,
) -> Result<Vec<String>> {
    tokio::task::spawn_blocking(move || {
        let mut archive = ZipArchive::new(reader)?;
        let mut extracted_files = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = format!("{}/{}", extract_to, file.name());
            
            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = std::path::Path::new(&outpath).parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = std::fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
                extracted_files.push(outpath);
            }
        }

        Ok(extracted_files)
    })
    .await?
}
