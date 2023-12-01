use pyrite::{app::stage::DEFAULT_STAGE, prelude::*};

pub fn entry_point(mut app: Application) {
    app.execute_stage(DEFAULT_STAGE);
}
