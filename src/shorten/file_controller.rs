use actix_multipart::{Multipart, MultipartError};
use actix_web::web;

use futures::{StreamExt, TryStreamExt};
use office::{DataType, Excel};
use std::io::Write;

use dotenv::dotenv;
use std::env;

use crate::shorten::model::ShortURL;
use crate::shorten::utils::hash_url;

pub struct ExcelFile;

impl ExcelFile {
  pub async fn process_file(mut payload: Multipart) -> Option<bool> {
    let field = payload.try_next().await.unwrap()?;
    let content_type = field.content_disposition()?;
    let filename = format!("uploads/{}", content_type.get_filename()?);
    ExcelFile::save_file(field, filename.clone())
      .await
      .expect("Fail to upload file");

    ExcelFile::process_data(filename);
    Some(true)
  }

  pub async fn save_file(
    mut field: actix_multipart::Field,
    filename: String,
  ) -> Result<(), MultipartError> {
    let filename_clone = filename.clone();

    // Create filename without inserting the data
    let mut file = web::block(|| std::fs::File::create(filename))
      .await
      .expect("Fail to create file");
    // Inserts file data to the filename
    let data_chunk = field.next().await.unwrap().unwrap();
    web::block(move || file.write_all(&data_chunk).map(|_| file))
      .await
      .expect("Fail to insert file data");

    Ok(println!("File {} uploaded successfully", filename_clone))
  }

  pub fn process_data(path: String) {
    dotenv().ok();
    let marph_url = &env::var("MARPH_URL").expect("Can't find MARPH URL");

    // sheet writer
    let workbook = xlsxwriter::Workbook::new(&path);
    let mut sheet_writer = workbook.add_worksheet(None).unwrap();

    // sheet reader
    let mut excel = Excel::open(path.clone()).expect("Fail to find workbook");
    let sheet_reader = excel.worksheet_range("Sheet1").unwrap();

    // Loop over excel and write
    let mut count = 0;
    let row_len = sheet_reader.rows().count();

    while count < row_len {
      let url_tuple: (String, String) = match (
        sheet_reader.get_value(count, 0).clone(),
        sheet_reader.get_value(count, 2).clone(),
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
      let processed_data = short_url.verify_and_hash().unwrap();

      let hashed = format!("{}/{}", marph_url, processed_data.hashed_url);
      let custom = format!("{}/{}", marph_url, processed_data.custom_url);

      sheet_writer
        .write_string(count as u32, 0, &processed_data.origin_url, None)
        .expect("Fail to write excel file");
      sheet_writer
        .write_url(count as u32, 1, &hashed, None)
        .expect("Fail to write excel file");
      sheet_writer
        .write_url(count as u32, 2, &custom, None)
        .expect("Fail to write excel file");
      count += 1;
    }
  }
}
