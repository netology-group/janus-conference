use std::fmt;
use std::path::Path;

use failure::Error;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client};
use s4::{self, S4};

use config::Uploading as UploadingConfig;

pub struct Uploader {
    client: S3Client,
}

impl fmt::Debug for Uploader {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "<<Uploader>>")?;

        Ok(())
    }
}

const PART_SIZE: usize = 1024 * 1024 * 100;

impl Uploader {
    pub fn new(config: UploadingConfig) -> Result<Self, Error> {
        let region = Region::Custom {
            name: config.region,
            endpoint: config.endpoint,
        };

        let client =
            s4::new_s3client_with_credentials(region, config.access_key, config.secret_key)?;

        Ok(Self { client })
    }

    pub fn upload_file(&self, file: &Path, bucket: &str, object: &str) -> Result<(), Error> {
        let req = PutObjectRequest {
            bucket: String::from(bucket),
            key: String::from(object),
            ..Default::default()
        };
        self.client
            .upload_from_file_multipart(file, &req, PART_SIZE)?;
        Ok(())
    }
}