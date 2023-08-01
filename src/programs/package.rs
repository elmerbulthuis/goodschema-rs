use crate::documents;
use crate::documents::context::Context;
use crate::documents::factory::Initializer;
use crate::schemas;
use crate::utils::namer::Namer;
use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
pub struct CommandOptions {
    pub schema_url: Url,

    #[arg(long)]
    pub package_directory: String,

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
    for key in intermediate_data.nodes.keys() {
        let url: Url = key.parse().unwrap();
        let path = url.path().to_string() + url.fragment().unwrap_or_default();
        namer.register_path(key.clone(), &path);
    }

    let names = namer.get_names();

    Ok(())
}
