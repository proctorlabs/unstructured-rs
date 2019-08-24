use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use unstructured::Document;

arg_enum! {
    #[derive(Debug)]
    pub enum Format {
        PrettyJson,
        Json,
        Yaml,
        Toml,
        Xml,
    }
}

impl Format {
    fn formatter(&self) -> Box<dyn Formatting> {
        match self {
            Format::Json => Box::new(Json),
            Format::PrettyJson => Box::new(PrettyJson),
            Format::Yaml => Box::new(Yaml),
            Format::Toml => Box::new(Toml),
            Format::Xml => Box::new(Xml),
        }
    }

    fn all() -> Vec<Format> {
        vec![
            Format::Json,
            Format::PrettyJson,
            Format::Yaml,
            Format::Toml,
            Format::Xml,
        ]
    }

    pub fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        self.formatter().serialize(value)
    }

    pub fn parse_stdin(&self) -> Result<Document, String> {
        let read = Box::new(std::io::stdin());
        self.formatter().parse(read)
    }

    pub fn parse_file(path: &PathBuf) -> Result<Document, String> {
        let mut ext = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        let ext_lower = ext.to_lowercase();
        ext = ext_lower.as_str();

        let rdr = Box::new(File::open(path).map_err(|e| format!("{}", e))?);
        for frmt in Format::all() {
            if frmt.formatter().get_extensions().contains(&ext) {
                return frmt.formatter().parse(rdr);
            }
        }
        Format::Json.formatter().parse(rdr)
    }
}

trait Formatting {
    fn get_extensions(&self) -> Vec<&'static str>;
    fn serialize(&self, value: &unstructured::Document) -> Result<String, String>;
    fn parse(&self, rdr: Box<dyn Read>) -> Result<Document, String>;
}

pub struct PrettyJson;

impl Formatting for PrettyJson {
    fn get_extensions(&self) -> Vec<&'static str> {
        vec!["json", "js"]
    }

    fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        serde_json::to_string_pretty(value).map_err(|e| format!("{}", e))
    }

    fn parse(&self, rdr: Box<dyn Read>) -> Result<Document, String> {
        serde_json::from_reader(rdr).map_err(|e| format!("{}", e))
    }
}

pub struct Json;

impl Formatting for Json {
    fn get_extensions(&self) -> Vec<&'static str> {
        vec!["json", "js"]
    }

    fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        serde_json::to_string(value).map_err(|e| format!("{}", e))
    }

    fn parse(&self, rdr: Box<dyn Read>) -> Result<Document, String> {
        serde_json::from_reader(rdr).map_err(|e| format!("{}", e))
    }
}

pub struct Yaml;

impl Formatting for Yaml {
    fn get_extensions(&self) -> Vec<&'static str> {
        vec!["yml", "yaml"]
    }

    fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        serde_yaml::to_string(value).map_err(|e| format!("{}", e))
    }

    fn parse(&self, rdr: Box<dyn Read>) -> Result<Document, String> {
        serde_yaml::from_reader(rdr).map_err(|e| format!("{}", e))
    }
}

pub struct Toml;

impl Formatting for Toml {
    fn get_extensions(&self) -> Vec<&'static str> {
        vec!["toml"]
    }

    fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        toml::to_string(value).map_err(|e| format!("{}", e))
    }

    fn parse(&self, mut rdr: Box<dyn Read>) -> Result<Document, String> {
        let mut s = Vec::new();
        rdr.read_to_end(&mut s).map_err(|e| format!("{}", e))?;
        Ok(toml::from_slice(&s).map_err(|e| format!("{}", e))?)
    }
}

pub struct Xml;

impl Formatting for Xml {
    fn get_extensions(&self) -> Vec<&'static str> {
        vec!["xml"]
    }

    fn serialize(&self, value: &unstructured::Document) -> Result<String, String> {
        serde_xml_rs::to_string(value).map_err(|e| format!("{}", e))
    }

    fn parse(&self, rdr: Box<dyn Read>) -> Result<Document, String> {
        serde_xml_rs::from_reader(rdr).map_err(|e| format!("{}", e))
    }
}
