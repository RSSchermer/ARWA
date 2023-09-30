#![feature(try_blocks)]

use std::error::Error;

use arwa::indexed_db::{
    key_path, DatabaseVersion, IndexedDbContext, KeyConfig, OpenDataBaseRequest, UpgradeTransaction,
};
use arwa::window::window;
use arwa::{console, spawn_local};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub author: String,
    pub title: String,
}

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    spawn_local(async {
        example().await.unwrap();
    })
}

async fn example() -> Result<(), Box<dyn Error>> {
    let indexed_db = window().indexed_db();

    // Delete database if it exists, so we can start this example with a clean slate
    indexed_db.delete_database("book-store")?.await?;

    // Open a connection to the database. If it did not yet exist, set it up in the `upgrade_needed`
    // callback.
    let database = indexed_db
        .open_database(OpenDataBaseRequest {
            name: "book-store",
            version: DatabaseVersion::Number(1),
            upgrade_needed: |transaction: UpgradeTransaction, old_version, _new_version| {
                if old_version < 1 {
                    let store = transaction
                        .create_object_store(
                            "books",
                            Some(KeyConfig {
                                path: key_path!("id"),
                                auto_increment: false,
                            }),
                        )
                        .unwrap();

                    store
                        .create_index("author", &key_path!("author"), &Default::default())
                        .unwrap();
                }
            },
            blocked: || (),
        })?
        .await?;

    // Run a read-write transaction to populate the database with an initial book list.
    let transaction = database.transaction_rw(&["books"])?;
    let books_store = transaction.object_store("books")?;

    books_store
        .add(&serde_wasm_bindgen::to_value(&Book {
            id: 0,
            author: "Herman Melville".to_string(),
            title: "Moby-Dick".to_string(),
        })?)?
        .await?;
    books_store
        .add(&serde_wasm_bindgen::to_value(&Book {
            id: 1,
            author: "Charles Dickens".to_string(),
            title: "Great Expectations".to_string(),
        })?)?
        .await?;
    books_store
        .add(&serde_wasm_bindgen::to_value(&Book {
            id: 2,
            author: "Charles Dickens".to_string(),
            title: "A Tale of Two Cities".to_string(),
        })?)?
        .await?;
    books_store
        .add(&serde_wasm_bindgen::to_value(&Book {
            id: 3,
            author: "Bram Stoker".to_string(),
            title: "Dracula".to_string(),
        })?)?
        .await?;

    transaction.commit();

    // Run a read-only transaction to retrieve a list of all books currently in the books store and
    // log them to the console.
    let transaction = database.transaction(&["books"])?;
    let books_store = transaction.object_store("books")?;

    let books = books_store.get_all()?.await?;

    console::group!("Initial book list");

    for book in books {
        let book: Book = serde_wasm_bindgen::from_value(book)?;

        console::log!("%s by %s", book.title, &book.author);
    }

    console::group_end();

    transaction.commit();

    // Another read-write transaction, in which we open a cursor through the "author" index over all
    // books written by Charles Dickens and delete them from the store
    console::log!("Removing books by Charles Dickens...");

    let transaction = database.transaction_rw(&["books"])?;
    let books_store = transaction.object_store("books")?;
    let author_index = books_store.index("author")?;

    let mut next = author_index
        .open_cursor_with_query(&JsValue::from_str("Charles Dickens"), Default::default())?
        .await?;

    while let Some(mut cursor) = next.take() {
        if let Some(value) = cursor.value() {
            let book: Book = serde_wasm_bindgen::from_value(value)?;

            console::log!("- Deleting %s...", book.title);

            cursor = cursor.delete()?.await?;
        }

        next = cursor.advance(1)?.await?;
    }

    transaction.commit();

    let transaction = database.transaction(&["books"])?;
    let books_store = transaction.object_store("books")?;

    let books = books_store.get_all()?.await?;

    console::group!("Book list after modification");

    for book in books {
        let book: Book = serde_wasm_bindgen::from_value(book)?;

        console::log!("%s by %s", book.title, &book.author);
    }

    console::group_end();

    transaction.commit();

    Ok(())
}
