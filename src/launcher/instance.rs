pub struct Instance {
    title: String,
    slug: String
}

impl Instance {
    pub fn new(title: String, slug: String) -> Self {
        Self {
            title,
            slug
        }
    }

    pub async fn launch() {
        // Use a result to ensure it launches well
    }
}
