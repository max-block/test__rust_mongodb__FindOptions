use mongodb::{bson::doc, error::Error, options::FindOptions, sync::Client};

fn main() -> Result<(), Error> {
    let client = Client::with_uri_str("mongodb://localhost")?;
    let col = client.database("test").collection("test");
    col.drop(None)?;

    col.insert_many(vec![doc! {"name": "n1"}, doc! {"name": "n2"}], None)?;

    // FindOptions::builder <-- intellij-rust and vscode can't autocomplete it

    let find_options = FindOptions::builder().sort(doc! { "name": -1}).build();
    let res: Vec<_> = col.find(doc! {}, find_options).unwrap().collect();

    println!("{:#?}", res);

    Ok(())
}
