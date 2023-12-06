use pyrite::app::AppBuilder;

mod engine;

pub fn setup_raven_schedule(app_builder: &mut AppBuilder) {
    engine::time::update(app_builder);
    engine::assets::update(app_builder);

    // let schedule = AppBuilder::run_schedule(async_schedule(
    //     // Group = any system can execvute in any order.
    //     // Linear group = systems execute in order and block when waiting for dependencies.
    //     // Update group.
    //     engine::player::update -> {
    //         engine::enemies::update -> {
    //             engine:;enemies::stage -> {
    //                 engine::enemies::render
    //             }
    //         }
    //     }

    //     // Explicitly defining all systems
    //     engine::player::update;
    //     (engine::player::stage).depends_on([
    //         engine::player::update
    //     ]),
    //     (engine::player::render).depends_on([
    //         engine::player::stage,
    //         engine::vulkan::render_begin
    //     ]),
    //     (engine::enemies::update).depends_on([
    //         engine::player::update
    //     ]),
    //     (engine::enemies::stage).depends_on([engine::enemies::update]),
    //     (engine::enemies::render).depends_on([engine::enemies::stage, engine::vulkan::render_begin]),

    //     // Implicitly defining all systems
    //     (engine::player::render).depends_on([
    //         (engine::player::stage).depends_on([
    //             engine::player::update
    //         ]),
    //         engine::vulkan::render_begin
    //     ]),
    //     (engine::enemies::render).depends_on([
    //         (engine::enemies::stage).depends_on([
    //             (engine::enemies::update).depends_on([
    //                 engine::player::update
    //             ])
    //         ]),
    //         engine::vulkan::render_begin
    //     ]),

    // )
    // // (engine::player::update <- engine::player::stage, engine::vulkan::render_begin) <- engine::player::render
    // // (engine::player::update <- engine::enemies::update <- engine::enemies::stage, engine::vulkan::render_begin) <- engine::enemies::render
    // ));
}
