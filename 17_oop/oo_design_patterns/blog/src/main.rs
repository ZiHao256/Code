use blog::*;

fn main(){
    let mut post = Post::new();

    post.add_text("hello world");

    println!("after add: {}", post.content());

    post.request_review();

    println!("after request_review: {}", post.content());

    post.approve();

    println!("after approve: {}", post.content());

}