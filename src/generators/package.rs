use super::{cargo_toml, file::generate_file_content, lib_rs, models_rs::ModelsRsGenerator};
use crate::schemas;
use std::{collections::HashMap, fs, path::PathBuf};

pub struct PackageGenerator<'a> {
    models_rs_generator: ModelsRsGenerator<'a>,
}

impl<'a> PackageGenerator<'a> {
    pub fn new(
        intermediate_data: &'a schemas::intermediate_a::SchemaJson,
        names: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            models_rs_generator: ModelsRsGenerator::new(intermediate_data, names),
        }
    }

    pub fn generate_package(
        &self,
        package_name: &str,
        package_version: &str,
        package_directory: &PathBuf,
    ) -> Result<(), &'static str> {
        fs::create_dir_all(package_directory).or(Err("create directory failed"))?;

        {
            let content = cargo_toml::generate_file_content(package_name, package_version)?;
            fs::write(package_directory.join("Cargo.toml"), content)
                .or(Err("write Cargo.toml fails"))?;
        }

        {
            let tokens = lib_rs::generate_file_token_stream()?;
            let content = generate_file_content(tokens)?;

            fs::write(package_directory.join("lib.rs"), content).or(Err("write lib.rs fails"))?;
        }

        {
            let tokens = self.models_rs_generator.generate_file_token_stream()?;
            let content = generate_file_content(tokens)?;

            fs::write(package_directory.join("models.rs"), content)
                .or(Err("write models.rs fails"))?;
        }

        Ok(())
    }
}
