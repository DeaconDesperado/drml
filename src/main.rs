use clap::{Parser, ValueEnum};
use env_logger::{Builder, Env};
use log;
use oxiri::IriParseError;
use oxrdf::Dataset;
use oxrdfio::{JsonLdProfileSet, RdfFormat, RdfParseError, RdfParser, RdfSerializer};
use parse::RmlMappingParser;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};
use thiserror::Error;

pub mod model;
pub mod parse;
pub mod process;
pub mod vocab;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Turtle format
    #[clap(alias = "ttl")]
    Turtle,
    /// N-Triples format
    Nt,
    /// N-Quads format  
    Nq,
    /// JSON-LD format
    Jsonld,
    /// Jelly format
    Jelly,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Nq
    }
}

impl From<OutputFormat> for RdfFormat {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Turtle => RdfFormat::Turtle,
            OutputFormat::Nt => RdfFormat::NTriples,
            OutputFormat::Nq => RdfFormat::NQuads,
            OutputFormat::Jsonld => RdfFormat::JsonLd {
                profile: JsonLdProfileSet::empty(),
            },
            OutputFormat::Jelly => todo!("Jelly-RS not yet available"),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Turtle => write!(f, "turtle"),
            OutputFormat::Nt => write!(f, "nt"),
            OutputFormat::Nq => write!(f, "nq"),
            OutputFormat::Jsonld => write!(f, "jsonld"),
            OutputFormat::Jelly => write!(f, "jelly"),
        }
    }
}

#[derive(Parser)]
#[command(name = "rml", about = "Process RML mappings to generate RDF data")]
pub struct RmlCommand {
    /// The RML mapping file
    #[arg(short = 'm', long = "mappingFile", required = true)]
    pub mapping_file: PathBuf,

    /// The output file
    #[arg(short = 'o', long = "outputFile")]
    pub output_file: Option<PathBuf>,

    /// The output file format
    #[arg(short = 'f', long = "format", default_value_t = OutputFormat::default())]
    pub format: OutputFormat,

    /// Used in resolving relative IRIs produced by the RML mapping
    #[arg(short = 'b', long = "baseIRI")]
    pub base_iri: Option<String>,
}

#[derive(Error, Debug)]
pub enum RmlError {
    #[error("Could determine format for file: no extension")]
    NoFileExtension(),
    #[error("Could determine format for file `{0}` extension `{0}`")]
    UnknownMappingFileType(String, String),
    #[error("Output file not availble")]
    OutputUnavailable(#[from] io::Error),
    #[error("Invalid mapping file")]
    InvalidMapping(#[from] RdfParseError),
    #[error("Invalid base IRI")]
    InvalidBaseIri(#[from] IriParseError),
}

fn mapping_format_from_ext(path: &PathBuf) -> Result<RdfFormat, RmlError> {
    if let Some(path_str) = path.to_str() {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("ttl") => Ok(RdfFormat::Turtle),
            Some("nq") | Some("nquads") => Ok(RdfFormat::NQuads),
            Some(e) => Err(RmlError::UnknownMappingFileType(
                path_str.to_string(),
                e.to_string(),
            )),
            None => Err(RmlError::NoFileExtension()),
        }
    } else {
        Err(RmlError::NoFileExtension())
    }
}

fn write<W: Write>(writer: W, format: RdfFormat, dataset: Dataset) -> Result<i32, RmlError> {
    Ok(0)
}

impl RmlCommand {
    pub fn run(&self) -> Result<i32, RmlError> {
        // TODO: Implement RML processing logic here
        // This would include:
        // 1. Parse the mapping file
        // 2. Process the RML mappings
        // 3. Generate RDF data
        // 4. Output to file or stdout based on format

        let mapping_format = mapping_format_from_ext(&self.mapping_file)?;
        log::info!("Reading mapping file with format: {}", mapping_format);
        let mapping_parser =
            RdfParser::from_format(mapping_format).for_reader(File::open(&self.mapping_file)?);

        let quads = mapping_parser.collect::<Result<Vec<_>, _>>()?;
        let mut mapping_dataset = Dataset::new();
        mapping_dataset.extend(quads);

        log::info!("Processing RML mapping: {:?}", self.mapping_file);
        log::info!("Output format: {:?}", self.format);
        let rml_parser = RmlMappingParser::new(mapping_dataset, self.mapping_file.to_owned());
        let output_dataset = Dataset::new();
        match &self.output_file {
            Some(path) => write(File::create(path)?, self.format.into(), output_dataset),
            None => write(io::stdout(), self.format.into(), output_dataset),
        }
    }
}

fn main() {
    let args = RmlCommand::parse();
    Builder::from_env(Env::default()).init();

    match args.run() {
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
        Err(e) => {
            log::error!("Error occurred during RML processing: {}", e);
            std::process::exit(1);
        }
    }
}
