use crate::domain::entity::NbPost;
use crate::base::resp::Result;
use crate::db::RB;
use rbatis::wrapper::Wrapper;
use rbatis::crud::CRUD;
use crate::domain::dto::{PostDto, CreatePostDto};

pub async fn list(start: i32, limit: i32) -> Result<Vec<PostDto>> {
    #[py_sql(RB, "SELECT * FROM nb_post order by id desc limit #{start}, #{limit}")]
    async fn select_post(start: i32, limit: i32) -> Vec<NbPost> {}

    let res = select_post(start, limit).await?;
    Ok(res.iter().map(|x| x.into()).collect())
}

pub async fn create_post(post: CreatePostDto) -> Result<PostDto> {
    let res = &NbPost {
        title: post.title,
        content: post.content,
        group_id: post.group_id,
        ..Default::default()
    };

    RB.save("", res).await?;

    Ok(res.into())
}