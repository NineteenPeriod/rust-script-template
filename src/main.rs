use std::{env, str::FromStr};

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use envia_pub_telegram::{config::Config, inicio::inicio::inicio, models::databases::Databases};
use lockfile::Lockfile;
use sqlx::mysql::MySqlPoolOptions;
use tracing::{Level, info};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    // initialize env
    dotenv().ok();

    // Obtenemos la configuracion del env en un Struct parse
    let config = Config::parse();

    // initializae tracing log file
    let tracing_appender = tracing_appender::rolling::daily(RUTA_SCRIPT, "___.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(
            Level::from_str(
                env::var("LOG_LEVEL")
                    .expect("El valor LOG_LEVEL debe estar presente en el .env")
                    .as_str(),
            )
            .unwrap(),
        )
        .init();

    // TODO: CERRAR LOCK MANUALMENTE AL FINAL DE LA EXECUCION - FOR GOOD MEASURE
    info!("Creando lockfile");
    let _ = Lockfile::create(PATH)
        .expect("No fue posible crear el lockfile, ya hay una instancia del proceso ejecutandose");

    // Build database database_url
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Inicializamos la conexion de la base
    let database_connection = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .context("Could not connect to database __")?;

    let databases = Databases {
        fc: database_connection,
    };

    inicio(config, databases)
        .await
        .expect("El proceso termino inesperadamente");

    Ok(())
}
