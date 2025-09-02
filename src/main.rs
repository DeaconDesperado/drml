use clap::{Parser, ValueEnum};
use env_logger::{Builder, Env};
use log;
use oxrdfio::{JsonLdProfileSet, RdfFormat};
use std::path::PathBuf;

pub mod parse;
pub mod process;

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

impl OutputFormat {
    fn to_rdf_format(self) -> RdfFormat {
        match self {
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

impl RmlCommand {
    pub fn run(&self) -> Result<i32, Box<dyn std::error::Error>> {
        // TODO: Implement RML processing logic here
        // This would include:
        // 1. Parse the mapping file
        // 2. Process the RML mappings
        // 3. Generate RDF data
        // 4. Output to file or stdout based on format

        log::info!("Processing RML mapping: {:?}", self.mapping_file);
        log::info!("Output format: {:?}", self.format);

        if let Some(output_file) = &self.output_file {
            log::info!("Output file: {:?}", output_file);
        } else {
            log::info!("Output to stdout");
        }

        if let Some(base_iri) = &self.base_iri {
            log::info!("Base IRI: {}", base_iri);
        }

        Ok(0)
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
