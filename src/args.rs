
use clap::Parser; 

#[derive(Parser)]
pub struct AppConfig {
    /// Define a porta do servidor
    #[clap(short, long, default_value_t = 3048)]
    pub porta: u16,
}