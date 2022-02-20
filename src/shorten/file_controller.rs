use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;

pub async fn save_file(mut payload: Multipart) -> Option<bool> {
  // iterate over multipart stream
  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_type = field.content_disposition().unwrap();
    let filepath = format!("uploads/{}", content_type.get_filename()?);

    // File::create is blocking operation, use threadpool
    let mut file = web::block(|| std::fs::File::create(filepath))
      .await
      .unwrap();

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
      let data = chunk.unwrap();
      // filesystem operations are blocking, we have to use threadpool
      file = web::block(move || file.write_all(&data).map(|_| file))
        .await
        .unwrap();
    }
  }

  Some(true)
}
