/*
 * Copyright (c) 2024, MLC 'Strawmelonjuice' Bloeiman
 *
 * Licensed under the BSD 3-Clause License. See the LICENSE file for more info.
 */

use std::io::{Error, ErrorKind};
use std::process;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::LuminaConfig;
use crate::post::PostInfo;

/// Basic exchangable user information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IIExchangedUserInfo {
    /// User ID
    pub(crate) id: i64,
    /// Known username
    pub(crate) username: String,
    /// Instance ID
    pub(crate) instance: String,
}

/// Basic user-identifying information.
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicUserInfo {
    /// User ID
    pub(crate) id: i64,
    /// Known username
    pub(crate) username: String,
    /// Hashed password
    pub(crate) password: String,
    /// Given email
    pub(crate) email: String,
}
impl BasicUserInfo {
    pub fn to_exchangable(&self, config: &LuminaConfig) -> IIExchangedUserInfo {
        IIExchangedUserInfo {
            id: self.id,
            username: self.username.clone(),
            instance: config.interinstance.iid.clone(),
        }
    }
}

/// Create a database connection
pub(crate) fn create_con(config: &LuminaConfig) -> Connection {
    match Connection::open(
        config
            .clone()
            .run
            .cd
            .join(config.clone().database.sqlite.unwrap().file),
    ) {
        Ok(d) => d,
        Err(_e) => {
            error!("Could not create a database connection!");
            process::exit(1);
        }
    }
}
/// # `storage::fetch()`
/// Fetches well-known data from the database.
pub fn fetch(
    config: &LuminaConfig,
    table: String,
    searchr: &str,
    searchv: String,
) -> Result<Option<String>, Error> {
    if config.database.method.as_str() == "sqlite" {
        match table.as_str() {
            "Users" | "PostsStore" => {}
            _ => {
                error!("Unknown table requisted!");
                panic!("Unknown table requisted!");
            }
        };
        let conn = create_con(config);
        dbconf(&conn);

        let mut stmt = conn
            .prepare(format!(r#"select * from {table} where {searchr} = '{searchv}'"#).trim())
            .unwrap();
        debug!("{:?}", stmt);
        let mut res = stmt
            .query_map((), |row| {
                Ok(match table.as_str() {
                    "Users" => serde_json::to_string(&BasicUserInfo {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password: row.get(2)?,
                        email: row.get(3)?,
                    })
                    .unwrap(),
                    "PostsStore" => {
                        let s = PostInfo {
                            lpid: row.get(0)?,
                            pid: row.get(1)?,
                            instance: row.get(2)?,
                            author_id: row.get(3)?,
                            timestamp: row.get(4)?,
                            content_type: row.get(5)?,
                            content: row.get(6)?,
                            tags: row.get(7)?,
                        };
                        serde_json::to_string(&s).unwrap()
                    }
                    _ => {
                        error!("Unknown table requisted!");
                        panic!("Unknown table requisted!");
                    }
                })
            })
            .unwrap();
        // println!("{:?}", res.nth(0));
        match res.next() {
            None => Ok(None),
            Some(r) => match r {
                Ok(s) => Ok(Some(s)),
                Err(f) => {
                    eprintln!("{:?}", f);
                    Err(Error::new(ErrorKind::Other, "Unparseable data."))
                }
            },
        }
    } else {
        error!("Unknown or unsupported database type! Only SQLITE is supported as of now.");
        process::exit(1);
    }
}
fn dbconf(conn: &Connection) {
    fn emergencyabort() {
        error!("Could not configure the database correctly!");
        process::exit(1);
    }

    match conn.execute(
        "
CREATE TABLE if not exists Users (
    id    INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
    username  TEXT NOT NULL,
    password  TEXT NOT NULL,
    email     TEXT NOT NULL
)
",
        (),
    ) {
        Ok(_) => {}
        Err(_e) => emergencyabort(),
    };
    match conn.execute(
        "
CREATE TABLE if not exists PostsStore (
    lpid            INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
    pid             INTEGER,
    instance        TEXT,
    author_id      INTEGER NOT NULL,
    timestamp      INTEGER NOT NULL,
    content_type    INTEGER NOT NULL,
    content        TEXT NOT NULL,
    tags            TEXT NOT NULL
)
",
        (),
    ) {
        Ok(_) => {}
        Err(_e) => emergencyabort(),
    }
}

pub(crate) mod users;
