use crate::progress_bar::{CustomProgressReporter, create_progress_bar};
use ripunzip::{UnzipEngine, UnzipOptions};
use std::{fs::File, io, path::Path, sync::Arc};

pub async fn decompress(archive_path: &str) -> io::Result<()> {
    let archive_path_buf = Path::new(archive_path);
    let output_dir = archive_path_buf.parent().unwrap().to_path_buf();
    let archive_path = archive_path.to_string();

    let file_count = {
        let archive_path = archive_path.clone();
        let file = File::open(&archive_path).map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to open archive: {}", e)))?;

        let engine = UnzipEngine::for_file(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to create unzip engine: {}", e)))?;
        let file_list: Vec<String> = engine
            .list()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to list files: {}", e)))?
            .filter(|name| !name.ends_with('/'))
            .collect();

        file_list.len() as u64
    };

    let pb = Arc::new(create_progress_bar(file_count));
    let im = format!("Extracting {} files...", file_count);
    pb.set_message(im);

    let progress_reporter = CustomProgressReporter::new(Arc::clone(&pb));
    let file = File::open(&archive_path).map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to open archive: {}", e)))?;
    let engine = UnzipEngine::for_file(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to create unzip engine: {}", e)))?;

    let options = UnzipOptions {
        output_directory: Some(output_dir),
        filename_filter: None,
        password: None,
        progress_reporter: Box::new(progress_reporter),
        single_threaded: false,
    };

    engine
        .unzip(options)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to decompress: {}", e)))?;

    let cm = format!("Extracted {} files successfully!", file_count);
    pb.finish_with_message(cm);
    Ok(())
}
