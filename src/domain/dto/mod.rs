use crate::domain::entity::NbPost;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePostDto {
    pub title: String,                  // 标题
    pub content: String,                // 帖子内容
    pub group_id: i64,                  // 群id
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditPostDto {
    pub id: i64,                        // primary key
    pub title: Option<String>,          // 标题
    pub content: Option<String>,        // 帖子内容
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostDto {
    pub id: i64,                        // primary key
    pub title: String,                  // 标题
    pub content: String,                // 帖子内容
    pub author_id: i64,                 // 作者
    pub author_name: String,            // 作者 username
    pub up_cnt: u32,                    // 顶数量
    pub down_cnt: u32,                  // 踩数量
    pub comments_cnt: u32,              // 评论数量
    pub group_id: i64,                  // 群id
    pub created_at: i64,                // 创建时间
    pub updated_at: i64,                // 更新时间
}

impl From<&NbPost> for PostDto {
    fn from(src: &NbPost) -> Self {
        Self {
            id: src.id,
            title: src.title.clone(),
            content: src.content.clone(),
            author_id: src.author_id,
            author_name: src.author_name.clone(),
            up_cnt: src.up_cnt,
            down_cnt: src.down_cnt,
            comments_cnt: src.comments_cnt,
            group_id: src.group_id,
            created_at: src.created_at.timestamp_millis(),
            updated_at: src.updated_at.timestamp_millis()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentDto {
    pub id: i64,                        // primary key
    pub content: String,                // 帖子内容
    pub author_id: i64,                 // 作者
    pub author_name: String,            // 作者 username
    pub up_cnt: u32,                    // 顶数量
    pub down_cnt: u32,                  // 踩数量
    pub root_id: i64,                   // 根帖子id
    pub parent_id: i64,                 // 父帖子id
    pub created_at: i64,                // 创建时间
    pub updated_at: i64,                // 更新时间
}