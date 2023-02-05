pub struct ReviewManager;

impl ReviewManager {

    pub fn new() -> Self {
        let review_manager = Self;
        review_manager.load();
        review_manager
    }

    pub fn load(&self) {

    }

}