//! Application endpoints.

use async_std::prelude::*;
use bson::doc;
use mongodb::options::FindOptions;
use tide::{Request, Response};

use super::state::State;

/// List all databases
pub(crate) async fn list_dbs(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let names = req.state().mongo().list_database_names(None, None).await?;
    Ok(names.join("\n"))
}

/// Get a single database
pub(crate) async fn list_colls(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?;
    log::info!("accessing database {}", name);
    let db = req.state().mongo().database(&name);
    let collections = db.list_collection_names(None).await?;
    Ok(collections.join("\n"))
}

/// Insert a document into a collection
pub(crate) async fn insert_doc(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?;
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?;
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];
    let _res = coll.insert_many(docs, None).await?;
    Ok("Insert successful!")
}

/// Insert a document into a collection
pub(crate) async fn find_doc(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?;
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?;
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };
    let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
    let mut cursor = coll.find(filter, find_options).await?;

    let mut s = String::new();
    while let Some(val) = cursor.next().await {
        s.push_str(&format!("{:?}", val));
        s.push('\n');
    }

    log::debug!("sending {} bytes", s.len());
    Ok(s)
}

/// Update a document in the collection.
pub(crate) async fn update_doc(req: Request<State>) -> tide::Result<impl Into<Response>> {
    let name: String = req.param("db")?;
    log::debug!("accessing database {}", name);
    let db = req.state().mongo().database(&name);

    let name: String = req.param("collection")?;
    log::debug!("accessing collection {}", name);
    let coll = db.collection(&name);

    // Query the documents in the collection with a filter and an option.
    let filter = doc! { "author": "George Orwell" };

    let other = doc! { "author": "[censored]" };
    coll.find_one_and_update(filter, other, None).await?;
    Ok("update successful!")
}
