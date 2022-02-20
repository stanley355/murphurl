use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;

pub async fn save_file(mut payload: Multipart) -> Option<bool> {
  let mut field = payload.try_next().await.unwrap()?;
  let content_type = field.content_disposition().unwrap();
  let filename = format!("uploads/{}", content_type.get_filename()?);

  // File::create is blocking operation, use threadpool
  let mut file = web::block(|| std::fs::File::create(filename))
    .await
    .unwrap();

  // Field in turn is stream of *Bytes* object
  let data_chunk = field.next().await.unwrap().unwrap();
  // filesystem operations are blocking, we have to use threadpool
  web::block(move || file.write_all(&data_chunk).map(|_| file))
    .await
    .unwrap();

  Some(true)
}
