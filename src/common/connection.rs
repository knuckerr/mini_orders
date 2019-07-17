use percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};
use std::fs::File;
use failure::{Error,err_msg};
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json;
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use postgres::rows::Row;
use postgres_shared::types::FromSql;



define_encode_set! {
    /// This encode set is used in the URL parser for query strings.
    pub QUERY_ENCODE_SET = [SIMPLE_ENCODE_SET] | {' ', '"', '#', '<', '>','%','&','@'}
}

pub type PgPool = Pool<PostgresConnectionManager>;

#[derive(Debug, Clone, Copy,Serialize, Deserialize)]
pub struct Database<'a> {
    pub user: &'a str,
    pub pass: &'a str,
    pub db: &'a str,
    pub ip: &'a str,
    pub port: i32,
}

impl<'a> Database<'a> {
    pub fn new(
        user_: &'a str,
        pass_: &'a str,
        db_: &'a str,
        ip_: &'a str,
        port_: i32,
    ) -> Database<'a> {
        Database {
            user: user_,
            pass: pass_,
            db: db_,
            ip: ip_,
            port: port_,
        }
    }
    pub fn connection(&self) -> Result<PgPool,Error> {
        let pass = utf8_percent_encode(self.pass, QUERY_ENCODE_SET).to_string();
        let user = utf8_percent_encode(self.user, QUERY_ENCODE_SET).to_string();
        let url_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, pass, self.ip, self.port, self.db
        );
        let config = PostgresConnectionManager::new(url_string,r2d2_postgres::TlsMode::None)?;
        let pool = Pool::builder().build(config)?;
        Ok(pool)
    }
    pub fn json_connection(path:&str)->Result<PgPool,Error>{
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let db : Database =  serde_json::from_str(&data)?;
        db.connection()

    }
}


pub fn handle_error<T: Default + FromSql>(key:&str,row:&Row) -> Result<T,Error> {
    let err = format!("{} not exist",key);
    let data = row.get_opt(key)
        .ok_or_else(|| err_msg(err))?
        .unwrap_or_default(); // this is more idiomatic as Adam pointed out
    Ok(data)
}
