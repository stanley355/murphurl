use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::web;

use futures::{StreamExt, TryStreamExt};
use office::{DataType, Excel};
use std::io::Write;

use crate::shorten::model::ShortURL;
use crate::shorten::utils::hash_url;

#[derive(Debug, Clone)]
pub struct ExcelFile;

impl ExcelFile {
  pub fn new() -> Self {
    Self
  }

  pub async fn upload_and_read(
    self,
    mut payload: Multipart,
  ) -> Result<Vec<ShortURL>, MultipartError> {
    // Validate file
    let field = payload
      .try_next()
      .await
      .unwrap()
      .expect("Fail to find payload data");
    let content_type = field.content_disposition().unwrap();
    let filename = format!("uploads/{:?}", content_type.get_filename());

    self.clone().save_file(field, filename.clone()).await?;

    let url_list = self.collect_url(filename).await;
    return Ok(url_list);
  }

  pub async fn save_file(self, mut field: Field, filename: String) -> Result<(), MultipartError> {
    // Create file without inserting the data
    let mut file = web::block(|| std::fs::File::create(filename))
      .await
      .expect("Fail to create file");
    // Inserts file data to the filename
    let data_chunk = field.next().await.unwrap();
    web::block(|| file.write_all(&data_chunk.unwrap()).map(|_| file))
      .await
      .expect("Fail to insert file data");

    Ok(println!("File uploaded successfully"))
  }

  pub async fn collect_url(self, path: String) -> Vec<ShortURL> {
    // sheet reader
    let mut excel = Excel::open(path.clone()).expect("Fail to find workbook");
    let sheet_reader = excel.worksheet_range("Sheet1").unwrap();

    // Loop over excel and push the url into vec
    let mut url_list: Vec<ShortURL> = vec![];
    let mut count = 0;
    let row_len = sheet_reader.rows().count();

    while count < row_len {
      let url_tuple: (String, String) = match (
        sheet_reader.get_value(count, 0).clone(),
        sheet_reader.get_value(count, 1).clone(),
      ) {
        (DataType::String(main_url), DataType::String(custom_url)) => (main_url, custom_url),
        (DataType::String(main_url), _) => (main_url, "".to_string()),
        _ => ("".to_string(), "".to_string()),
      };

      let short_url = ShortURL {
        origin_url: url_tuple.0.clone(),
        hashed_url: hash_url(&url_tuple.0),
        custom_url: url_tuple.1,
      };

      url_list.push(short_url);
      count += 1;
    }

    return url_list;
  }
}
