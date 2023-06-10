use std::env;
use dotenv::dotenv;
use diesel::result::Error;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection}
  };
use crate::persistence::schema::{users, posts};
use crate::persistence::model::*;
use crate::persistence::schema::users::{dsl::*, id as user_id};
use crate::persistence::schema::posts::{dsl::*, id as post_id};
use diesel::{self, prelude::*};
use diesel::dsl::not;

pub struct DBPostgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DBPostgres {
    pub async fn init() -> Self {
        dotenv().ok();
        let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder().build(manager).expect("Error building a connection pool");
        DBPostgres { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Unable to establish connection")
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        diesel::insert_into(users)
            .values(user)
            .returning((
                user_id,
                username,
                display_name.nullable(),
                about_me.nullable(),
                description.nullable(),
                avatar.nullable()
              ))
            .get_result::<User>(&mut conn)
    }

    pub async fn update_user(&self, _user_id: &i32, user: UpdateUser) -> Result<User, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        diesel::update(users)
            .filter(user_id.eq(_user_id))
            .set::<UpdateUser>(user)
            .get_result::<User>(&mut conn)
  }

    pub async fn get_user(&self, _user_id: &i32) -> Result<User, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        users.filter(user_id.eq(_user_id)).first::<User>(&mut conn)
    }

    pub async fn create_post(&self, post: CreatePost) -> Result<Post, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        diesel::insert_into(posts)
            .values(post)
            .returning((
                post_id,
                author,
                blocks.nullable(),
                created_at.nullable(),
                updated_at.nullable(),
                status
            ))
            .get_result::<Post>(&mut conn)
    }

    pub async fn update_post(&self, _post_id: &i32, _author: &i32, post: UpdatePost) -> Result<Post, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        diesel::update(posts)
            .filter(post_id.eq(_post_id).and(author.eq(_author)))
            .set::<UpdatePost>(post)
            .get_result::<Post>(&mut conn)
    }

    pub async fn delete_post(&self, _post_id: &i32, _author: &i32, post: DeletePost) -> Result<Post, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        diesel::update(posts)
            .filter(post_id.eq(_post_id))
            .set::<DeletePost>(post)
            .get_result::<Post>(&mut conn)
    }

    pub async fn get_post(&self, _post_id: &i32, _author: &i32) -> Result<Post, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        posts.filter(post_id.eq(_post_id).and(author.eq(_author)).and(not(status.eq("DELETED")))).first::<Post>(&mut conn)
    }

    pub async fn get_posts(&self, _author: &i32) -> Result<Vec<Post>, Error> {
        let mut conn = DBPostgres::get_connection(&self);
        posts.filter(author.eq(_author).and(not(status.eq("DELETED")))).get_results::<Post>(&mut conn)
    }
}