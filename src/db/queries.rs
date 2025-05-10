
use sqlx::MySqlPool;
use crate::db::models::{BlogPost,NewBlogPost};

// SQLx Functions
// pub async fn create_post(pool: &MySqlPool, post: &NewBlogPost) -> Result<BlogPost,sqlx::Error> {
//     let post = sqlx::query_as::<_, BlogPost>( 
//         "INSERT INTO blog_posts (title, content, author) VALUES ($1, $2, $3) RETURNING *",
// ).bind(&post.title)
// .bind(&post.content)
// .bind(&post.author)
// .fetch_one(pool)
// .await?;
// Ok(post)
// }
pub async fn create_post(pool: &MySqlPool, post: &NewBlogPost) -> Result<BlogPost, sqlx::Error> {
    // MySQL doesn't support RETURNING, so we need to do this in two steps
    let result = sqlx::query!(
        r#"
        INSERT INTO blog_posts (title, content, author)
        VALUES (?, ?, ?)
        "#,
        post.title,
        post.content,
        post.author
    )
    .execute(pool)
    .await?;

    // Now fetch the inserted record
    let post = sqlx::query_as::<_, BlogPost>(
        r#"
        SELECT id, title, content, author 
        FROM blog_posts 
        WHERE id = ?
        "#,
    )
    .bind(result.last_insert_id())
    .fetch_one(pool)
    .await?;

    Ok(post)
}


pub async fn get_posts(pool: &MySqlPool) -> Result<Vec<BlogPost>, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(pool)
        .await
}

pub async fn get_post(pool: &MySqlPool, id: i32) -> Result<BlogPost, sqlx::Error> {
    sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_post(pool: &MySqlPool, id: i32, post: &BlogPost) -> Result<(), sqlx::Error> {
    let q = "UPDATE blog_posts SET title = ?, content = ?, author = ? WHERE id = ?";
    sqlx::query(q)
    .bind(&post.title)
    .bind(&post.content)
    .bind(&post.author)
    .bind(&id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_post(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    let q = "DELETE FROM blog_posts WHERE id = ?";
    sqlx::query(q)
    .bind(&id)
    .execute(pool)
    .await?;
    Ok(())
}
