use crate::progress_bar::create_progress_bar;
use mtzip::ZipArchive;
use std::{fs::File, io, path::Path, sync::Arc};
use walkdir::WalkDir;

pub async fn compress(source: &str) -> io::Result<()> {
    let source_path = Path::new(source);

    if !source_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Source '{}' not found", source)));
    }

    let source_name = source_path.file_name().unwrap().to_str().unwrap().to_string();
    let output_path = source_path.parent().unwrap().join(format!("{}.zip", source_name));
    let source_path_buf = source_path.to_path_buf();

    let file_count = if source_path_buf.is_file() {
        1
    } else {
        WalkDir::new(&source_path_buf)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .count() as u64
    };

    let pb = Arc::new(create_progress_bar(file_count));
    let im = format!("Compressing {} files...", file_count);
    pb.set_message(im);

    let mut zipper = ZipArchive::default();
    if source_path_buf.is_file() {
        let display_name = if source_name.len() > 60 {
            format!("...{}", &source_name[source_name.len() - 57..])
        } else {
            source_name.clone()
        };
        pb.set_message(format!("Compressing: {}", display_name));
        zipper.add_file_from_fs(&source_path_buf, source_name).done();
        pb.inc(1);
    } else {
        for entry in WalkDir::new(&source_path_buf).into_iter().filter_map(|e| e.ok()).filter(|e| e.path().is_file()) {
            let path = entry.path();
            let rpath = path.strip_prefix(&source_path_buf).unwrap();
            let zip_path = rpath.to_string_lossy().replace('\\', "/");
            let display_name = if zip_path.len() > 60 {
                format!("...{}", &zip_path[zip_path.len() - 57..])
            } else {
                zip_path.clone()
            };
            pb.set_message(format!("Compressing: {}", display_name));
            let path_buf = path.to_path_buf();
            zipper.add_file_from_fs(path_buf, zip_path).done();
            pb.inc(1);
        }
    }

    pb.set_message("Writing zip archive...");
    let mut output_file = File::create(&output_path)?;
    zipper.write(&mut output_file)?;

    let cm = format!("Compressed {} files successfully!", file_count);
    pb.finish_with_message(cm);
    Ok(())
}
