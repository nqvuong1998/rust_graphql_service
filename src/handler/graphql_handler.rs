use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use crate::{
    persistence::postgres::DBPostgres,
    persistence::model::*,
};

pub struct Query;

#[Object(extends)]
impl Query {
    async fn get_user(&self, ctx: &Context<'_>, user_id: i32) -> FieldResult<User> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.get_user(&user_id).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err.into())
        }
        
    }

    async fn get_post(&self, ctx: &Context<'_>, post_id: i32, author: i32) -> FieldResult<Post> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.get_post(&post_id, &author).await {
            Ok(post) => Ok(post),
            Err(err) => Err(err.into())
        }
    }

    async fn get_posts(&self, ctx: &Context<'_>, author: i32) -> FieldResult<Vec<Post>> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.get_posts(&author).await {
            Ok(posts) => Ok(posts),
            Err(err) => Err(err.into())
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUser) -> FieldResult<User> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.create_user(input).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err.into())
        }
    }

    async fn update_user(&self, ctx: &Context<'_>, user_id: i32, input: UpdateUser) -> FieldResult<User> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.update_user(&user_id, input).await {
            Ok(user) => Ok(user),
            Err(err) => Err(err.into())
        }
    }

    async fn create_post(&self, ctx: &Context<'_>, input: CreatePost) -> FieldResult<Post> {
        let status = input.status.to_string().trim().to_uppercase();
        if !(status.eq ("PUBLISHED") || status.eq("DRAFT")){
            return Err("Wrong status format".into());
        }

        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.create_post(input).await {
            Ok(post) => Ok(post),
            Err(err) => Err(err.into())
        }
    }

    async fn update_post(&self, ctx: &Context<'_>, post_id: i32, author: i32, input: UpdatePost) -> FieldResult<Post> {
        let status = input.status.to_string().trim().to_uppercase();
        if !(status.eq ("PUBLISHED") || status.eq("DRAFT")){
            return Err("Wrong status format".into());
        }

        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.update_post(&post_id, &author, input).await {
            Ok(post) => Ok(post),
            Err(err) => Err(err.into())
        }
    }

    async fn delete_post(&self, ctx: &Context<'_>, post_id: i32, author: i32) -> FieldResult<Post> {
        let db = &ctx.data_unchecked::<DBPostgres>();
        match db.delete_post(&post_id, &author, DeletePost { status: "DELETED".to_string() }).await {
            Ok(post) => Ok(post),
            Err(err) => Err(err.into())
        }
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;