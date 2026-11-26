trait Screen {
    // update current screen state
    fn update(&mut self);
    // handels rendering of the current screen state
    fn render(&self);
    // since we want to put the screens on a stack
    // handels the permissions of the screens below the current one
    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}


struct MainMenuScreen;

impl Screen for MainMenuScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}

struct GameplayScreen;

impl Screen for GameplayScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        true
    }
}

struct RunScreen;

impl Screen for RunScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        true
    }
}

struct UIScreen;

impl Screen for UIScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        false
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}

struct PauseScreen;

impl Screen for PauseScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}

struct RecipeMenuScreen;

impl Screen for RecipeMenuScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        true
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}

struct VictoryScreen;

impl Screen for VictoryScreen {
    fn update(&mut self) {
        // to be done
    }

    fn render(&self) {
        // to be done
    }

    fn blocks_update_below(&self) -> bool {
        false
    }

    fn blocks_render_below(&self) -> bool {
        false
    }
}