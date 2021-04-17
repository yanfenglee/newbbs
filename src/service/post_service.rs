use crate::domain::NBPost;
use crate::base::resp::Result;

pub async fn list(start: i32, limit: i32) -> Result<Vec<NBPost>> {
    info!("call list {},{}", start, limit);
    let res = NBPost::default();

    Ok(vec![res.clone(),res.clone(),res.clone()])
}