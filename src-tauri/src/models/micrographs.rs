use std::path::PathBuf;

use crate::schema::micrographs;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Queryable};
use tauri::AppHandle;
use ts_rs::TS;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
pub struct Micrograph {
    pub uuid: String,
    pub name: String,
    pub import_path: String,
    pub thumbnail_img: Option<Vec<u8>>,
    pub display_img: Option<Vec<u8>>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(TS)]
#[ts(export)]
pub struct PortableMicrograph {
    pub uuid: String,
    pub name: String,
    pub import_path: String,
    pub thumbnail_img: Option<PathBuf>,
    pub display_img: Option<PathBuf>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Micrograph {
    pub fn to_portable(&self, app: &AppHandle) -> PortableMicrograph {
        PortableMicrograph {
            uuid: self.uuid.clone(),
            name: self.name.clone(),
            import_path: self.import_path.clone(),
            thumbnail_img: self.get_thumbnail_path(app),
            display_img: self.get_display_path(app),
            width: self.width,
            height: self.height,
            status: self.status.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    fn get_thumbnail_path(&self, app: &AppHandle) -> Option<std::path::PathBuf> {
        // check if thumbnail exists
        if self.thumbnail_img.is_none() {
            return None;
        }

        // generate thumbnail path
        let path = app
            .path_resolver()
            .app_cache_dir()
            .unwrap()
            .join("thumbnails")
            .join(format!("{}.png", self.uuid));

        // check if thumbnail already exists
        if path.exists() {
            return Some(path);
        }

        // load thumbnail from database and write to file
        std::fs::write(&path, &self.thumbnail_img.clone().unwrap()).unwrap();

        Some(path)
    }

    fn get_display_path(&self, app: &AppHandle) -> Option<std::path::PathBuf> {
        // check if display image exists
        if self.display_img.is_none() {
            return None;
        }

        // generate display image path
        let path = app
            .path_resolver()
            .app_cache_dir()
            .unwrap()
            .join("display_images")
            .join(format!("{}.png", self.uuid));

        // check if display image already exists
        if path.exists() {
            return Some(path);
        }

        // load display image from database and write to file
        std::fs::write(&path, &self.display_img.clone().unwrap()).unwrap();

        Some(path)
    }
}
