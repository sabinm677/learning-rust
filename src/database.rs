use std::collections::HashMap;

use rusqlite::{Connection, Result, Error};

fn main() -> Result<()> {
    get_cats()?;

    Ok(())
}

#[allow(dead_code)]
fn create_table() -> Result<(), Error> {
    let conn = get_connection()?;
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        [],
    )?;

    Ok(())
}

fn get_connection() -> Result<Connection, Error> {
    #[allow(unused_variables)]
    let conn= Connection::open("/tmp/rust/database.sqlite")?;
    
    return Ok(conn);
}

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

#[allow(dead_code)]
fn store() -> Result<()> {
    let conn = get_connection()?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        println!("Found cat {:?}", cat.unwrap().name);
    }

    Ok(())
}

fn get_cats() -> Result<()>{
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id
         WHERE c.name='Sammy'",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        let cat_unwrapped = cat.unwrap();
        println!("Found cat ->\nName: {:?} \nColor: {:?} \n", cat_unwrapped.name, cat_unwrapped.color);
    }

    Ok(())
}