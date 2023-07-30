use crate::documents;
use crate::documents::context::Context;
use crate::documents::factory::Initializer;
use crate::schemas;
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

    #[arg(long)]
    pub generate_test: bool,

    #[arg(long, default_value_t = 0)]
    pub unique_name_seed: usize,
}

pub fn run_command(options: CommandOptions) -> Result<(), &'static str> {
    let CommandOptions { schema_url, .. } = options;

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

    Ok(())
}
