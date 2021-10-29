extern crate blog2;
use blog2::Post;

fn main() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.clear_text();
    // 今日はお昼にポテトを食べた
    post.add_text("I ate a potato for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.clear_text();
    // 今日はお昼にトマトを食べた
    post.add_text("I ate a tomato for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a tomato for lunch today", post.content());

    post.clear_text();
    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("I ate a tomato for lunch today", post.content());
}
