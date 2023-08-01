use super::{
    cargo_toml, file::generate_file_content, lib_rs, models_rs::ModelsRsGenerator,
    validators_rs::ValidatorsRsGenerator,
};
use crate::{schemas::InterpreterContext, utils::Namer};
use std::{fs, path::PathBuf};
use url::Url;

pub struct PackageGenerator<'a> {
    models_rs_generator: ModelsRsGenerator<'a>,
    validators_rs_generator: ValidatorsRsGenerator<'a>,
}

impl<'a> PackageGenerator<'a> {
    pub fn new(schema_loader: &'a InterpreterContext<'a>, namer: &'a Namer<Url>) -> Self {
        Self {
            models_rs_generator: ModelsRsGenerator::new(schema_loader, namer),
            validators_rs_generator: ValidatorsRsGenerator::new(schema_loader, namer),
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

        {
            let tokens = self.validators_rs_generator.generate_file_token_stream()?;
            let content = generate_file_content(tokens)?;

            fs::write(package_directory.join("validators.rs"), content)
                .or(Err("write validators.rs fails"))?;
        }

        Ok(())
    }
}
