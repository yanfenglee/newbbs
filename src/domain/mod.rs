use chrono::NaiveDateTime;

/// 帖子表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBPost {
    pub id: i64,                        // primary key
    pub title: String,                  // 标题
    pub content: String,                // 帖子内容
    pub author_id: i64,                 // 作者
    pub author_name: String,            // 作者 username
    pub up_cnt: u32,                    // 顶数量
    pub down_cnt: u32,                  // 踩数量
    pub comments_cnt: u32,              // 评论数量
    pub parent_id: i64,                 // 父帖子id
    pub group_id: i64,                  // 群名称
    pub created_at: NaiveDateTime,      // 创建时间
    pub updated_at: NaiveDateTime,      // 更新时间
}

/// 群表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBGroup {
    pub id: i64,                        // primary key
    pub name: String,                   // 群名(unique key)
    pub description: String,            // 群描述
    pub member_cnt: u32,                // 群成员数量
    pub owner_id: i64,                  // 群主
    pub created_at: NaiveDateTime,      // 创建时间
    pub updated_at: NaiveDateTime,      // 更新时间
}

/// 群成员表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBGroupMember {
    pub id: i64,                        // primary key
    pub member_id: i64,                 // 成员id
    pub member_name: String,            // 成员名字
    pub created_at: NaiveDateTime,      // 创建时间
}

/// 消息通知表
#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NBMessage {
    pub id: i64,                        // primary key
    pub member_id: i64,                 // 成员id
    pub msg_type: i32,                  // 消息类型
    pub msg: String,                    // 消息内容
    pub link: Option<String>,           // 消息链接地址
    pub read_flag: bool,                // read 标志
    pub created_at: NaiveDateTime,      // 创建时间
}