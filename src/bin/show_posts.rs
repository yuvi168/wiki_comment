use self::models::*;
use diesel::prelude::*;
use demo::*;

fn main() {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published).eq(true)
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading post");
    println!("Displaying {} posts", results.len());
    for post in result {
        println!("{}", post.title);
        println!("--------------");
        println!("{}", post.body);
    }
}
