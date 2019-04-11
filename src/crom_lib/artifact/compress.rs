use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use libflate::gzip::Encoder;
use std::io::Write;
use tar::Builder as TarBuilder;
use tempfile::NamedTempFile;

use crate::crom_lib::config::file::*;
use crate::crom_lib::error::*;

pub fn compress_files(
    output_file: &NamedTempFile,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
    format: &ProjectArtifactCompressionFormat,
) -> Result<(), ErrorContainer> {
    debug!("Compressing {:?} into {:?}", root_path, output_file);
    match format {
        ProjectArtifactCompressionFormat::ZIP => zip(output_file, root_path, artifacts),
        ProjectArtifactCompressionFormat::TGZ => tgz(output_file, root_path, artifacts),
    }
}

fn zip(
    output_file: &NamedTempFile,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
) -> Result<(), ErrorContainer> {
    let mut zip = zip::ZipWriter::new(output_file);

    for (name, path) in artifacts {
        debug!("Compressing {} located at {}", name, path);
        let name = name.to_string();
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        if let Err(_e) = zip.start_file(name.clone(), options) {
            return Err(ErrorContainer::Compress(CompressError::ZipFileNameErr(
                name,
            )));
        }

        let mut art_path = root_path.clone();
        art_path.push(Path::new(path));

        if !art_path.exists() {
            return Err(ErrorContainer::Compress(
                CompressError::UnableToFindArtifact(art_path.to_str().unwrap().to_string()),
            ));
        }

        let mut file = File::open(art_path)?;
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;

        zip.write_all(&contents)?;
    }

    // Optionally finish the zip. (this is also done on drop)
    zip.finish()?;

    Ok(())
}

fn tgz(
    output_file: &NamedTempFile,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
) -> Result<(), ErrorContainer> {
    let mut ar = TarBuilder::new(Vec::new());

    for (name, path) in artifacts {
        let mut art_path = root_path.clone();
        art_path.push(path);

        let mut f = File::open(art_path)?;
        ar.append_file(name, &mut f)?;
    }

    let mut encoder = Encoder::new(output_file)?;
    let data = ar.into_inner()?;
    encoder.write_all(&data)?;
    encoder.finish().into_result()?;

    Ok(())
}
