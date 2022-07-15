trait Web {
    fn publish(&self);
    fn deletePost(&self) {
        println!("Deleting post on this Web");
    }
}

struct Blog {}
impl Web for Blog {
    fn publish(&self) {
        println!("Publishing new post on blog");
    }
    fn deletePost(&self) {
        println!("Deleting post on this Blog");
    }
}

struct Forum {}
impl Web for Forum {
    fn publish(&self) {
        println!("Publishing new post on forum");
    }
}

pub fn init() {
    let newBlog = Blog {};
    let newForum = Forum {};
    newBlog.deletePost();
    newForum.deletePost();
}