use clap::Parser;
use std::path::PathBuf;
use url::Url;

use crate::documents;
use crate::documents::context::Context;
use crate::documents::factory::Initializer;
use crate::generators::PackageGenerator;
use crate::schemas;
use crate::selectors::node::NodeSelectors;
use crate::utils::namer::Namer;

#[derive(Parser, Debug)]
pub struct CommandOptions {
    pub schema_url: Url,

    #[arg(long)]
    pub package_directory: PathBuf,

    #[arg(long)]
    pub package_name: String,

    #[arg(long)]
    pub package_version: String,

    #[arg(long, default_value = "schema")]
    pub root_name_part: String,
}

pub fn run_command(options: CommandOptions) -> Result<(), &'static str> {
    let CommandOptions {
        schema_url,
        root_name_part,
        package_name,
        package_version,
        package_directory,
        ..
    } = options;

    let mut context = Context::new();
    context.register_factory(
        schemas::intermediate_a::SCHEMA_ID.parse().unwrap(),
        Box::new(
            |Initializer {
                 given_url,
                 document_node,
                 ..
             }| {
                Box::new(documents::intermediate_a::Document::new(
                    given_url.clone(),
                    document_node,
                ))
            },
        ),
    );
    context.load_from_url(
        &schema_url,
        &schema_url,
        None,
        schemas::intermediate_a::SCHEMA_ID,
    )?;

    let intermediate_data = context.get_intermediate_data();

    let mut namer = Namer::new(root_name_part.as_str());
    for (key, node) in intermediate_data.nodes.iter() {
        if node.select_is_empty() && node.super_node_id.is_some() {
            continue;
        }

        let url: Url = key.parse().unwrap();
        let path = url.path().to_string() + url.fragment().unwrap_or_default();
        namer.register_path(key.clone(), &path);
    }

    let names = namer.get_names();

    let package_generator = PackageGenerator::new(&intermediate_data, &names);

    package_generator.generate_package(&package_name, &package_version, &package_directory)?;

    Ok(())
}
