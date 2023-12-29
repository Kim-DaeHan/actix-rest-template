use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

// <'a> 은 라이프타임 매개변수를 나타냄(a라는 라이프타임이 있다)
#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostData {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
// Query (조회)할 때 (Queryable):

// 조회된 데이터는 데이터베이스에서 읽어오는 것이기 때문에 소유권을 가질 필요가 있습니다. 그래서 String과 같이 소유권을 가지는 타입을 사용합니다.
// String은 동적으로 크기가 조절되는 문자열을 나타내며, 이는 데이터베이스에서 읽어온 문자열의 크기가 불확실할 수 있기 때문에 적합합니다.

// Insert (삽입)할 때 (Insertable):

// 데이터를 데이터베이스에 넣을 때는 소유권을 넘기는 것이 아니라 참조만 넘기는 것이 효율적일 수 있습니다. 특히 문자열을 데이터베이스에 넣을 때 크기를 미리 알 수 없는 경우가 많기 때문에 동적인 문자열을 소유하기보다는 참조를 사용하는 것이 좋습니다.
// 따라서 &'a str과 같이 라이프타임이 있는 참조를 사용하여 문자열을 참조하고, 데이터베이스에는 참조만 전달합니다.
