extern crate blog;
use blog::Post;

fn main() {
  let mut post = Post::new();
  post.add_text("This is the text of the post");
  let post = post.request_review();
  let post = post.approve();
  assert_eq!("This is the text of the post", post.content());
}