extern crate blog;
use blog::Post;

fn main() {
  let mut post = Post::new();
  post.add_text("This is the text of the post");
  assert_eq!("", post.content());

  post.request_review();
  assert_eq!("", post.content());

  post.reject();
  assert_eq!("", post.content());

  post.request_review();
  post.add_text("This is the text of the post");
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("", post.content());

  post.approve();
  assert_eq!("This is the text of the post", post.content());

}