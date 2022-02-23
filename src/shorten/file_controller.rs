use futures::{StreamExt, TryStreamExt};
use office::{DataType, Excel};
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web;

use crate::shorten::model::ShortURL;
use crate::shorten::utils::hash_url;

pub async fn save_file(mut payload: Multipart) -> Option<bool> {
  let mut field = payload.try_next().await.unwrap()?;
  let content_type = field.content_disposition()?;
  let filename = format!("uploads/{}", content_type.get_filename()?);
  let a = filename.clone();

  // Create filename without inserting the data
  // File::create is blocking operation, use threadpool
  let mut file = web::block(|| std::fs::File::create(filename))
    .await
    .expect("Fail to create file");

  // Inserts file data to the filename
  // Field in turn is stream of *Bytes* object
  let data_chunk = field.next().await.unwrap().unwrap();
  // filesystem operations are blocking, we have to use threadpool
  web::block(move || file.write_all(&data_chunk).map(|_| file))
    .await
    .expect("Fail to insert file data");

  read_file(a);

  Some(true)
}

pub fn read_file(path: String) {
  // opens a new workbook
  let mut excel = Excel::open(path).expect("Fail to find workbook");
  let mut sheet = excel.worksheet_range("Sheet1").unwrap();

  sheet.set_value((1, 1), office::DataType::String("wkakwkw".to_string()));
  let mut count = 1;
  let row_len = sheet.rows().count();

  while count < row_len  {
    let main_url = match sheet.get_value(count, 0).clone() {
      DataType::String(value) => value,
      _ => "".to_string(),
    };
    let customized = match sheet.get_value(count, 2).clone() {
      DataType::String(value) => value,
      _ => "".to_string(),
    };
    let short_url = ShortURL {
      origin_url: main_url.clone(),
      hashed_url: hash_url(&main_url),
      custom_url: customized,
    };

    let processed_url = short_url.verify_and_hash();
    sheet.set_value((count as u32, 1), office::DataType::String(processed_url.unwrap().hashed_url));

    count += 1;
  }
}
