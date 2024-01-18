use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct CliArgs {
    #[arg(short, long, default_value = "Operator")]
    pub name: String,

    #[arg(short, long, default_value_t = 50)]
    pub core_radius: u8
}