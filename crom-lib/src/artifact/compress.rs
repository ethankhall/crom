use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use libflate::gzip::Encoder;
use std::io::Write;
use tar::Builder as TarBuilder;
use tempfile::NamedTempFile;

use crate::config::file::*;
use crate::error::*;

pub fn compress_files(
    output_file: &NamedTempFile,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
    format: &ProjectArtifactCompressionFormat,
) -> Result<(), ErrorContainer> {
    return match format {
        ProjectArtifactCompressionFormat::ZIP => zip(output_file, root_path, artifacts),
        ProjectArtifactCompressionFormat::TGZ => tgz(output_file, root_path, artifacts),
    };
}

fn zip(
    output_file: &NamedTempFile,
    root_path: PathBuf,
    artifacts: &HashMap<String, String>,
) -> Result<(), ErrorContainer> {
    use std::io::Write;

    let mut zip = zip::ZipWriter::new(output_file);

    for (name, path) in artifacts {
        let name = format!("{}", name);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        if let Err(_e) = zip.start_file(name.clone(), options) {
            return Err(ErrorContainer::Compress(CompressError::ZipFileNameErr(
                name,
            )));
        }

        let mut art_path = root_path.clone();
        art_path.push(path);

        let mut file = File::open(art_path)?;
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;

        zip.write(&contents)?;
    }

    // Optionally finish the zip. (this is also done on drop)
    zip.finish()?;

    return Ok(());
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

        let mut f = File::open(art_path).unwrap();
        ar.append_file(name, &mut f).unwrap();
    }

    let mut encoder = Encoder::new(output_file)?;
    let data = ar.into_inner()?;
    encoder.write_all(&data)?;
    encoder.finish().into_result()?;

    return Ok(());
}
