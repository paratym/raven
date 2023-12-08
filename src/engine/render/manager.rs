use ash::vk;
use pyrite::app::resource::Resource;
use pyrite::vulkan::executor::{QueueExecutor, QueueExecutorSubmitInfo};
use pyrite::vulkan::objects::{CommandBufferHandle, CommandPool, Fence, Semaphore};
use pyrite::vulkan::swapchain::Swapchain;
use pyrite::vulkan::Vulkan;

use crate::constants::{self, FRAMES_IN_FLIGHT};

pub struct Frame {
    fence: Fence,
    image_available_semaphore: Semaphore,
    ready_to_present_semaphore: Semaphore,
    command_pool: CommandPool,
    main_command_buffer: CommandBufferHandle,
}

#[derive(Resource)]
pub struct RenderManager {
    frames: [Frame; FRAMES_IN_FLIGHT],
    default_queue_executor: QueueExecutor<FRAMES_IN_FLIGHT>,
    frame_index: usize,
}

impl RenderManager {
    pub fn new(vulkan: &Vulkan) -> Self {
        let frames = (0..FRAMES_IN_FLIGHT)
            .map(|_| {
                let fence = Fence::new(vulkan, true);
                let [image_available_semaphore, ready_to_present_semaphore] =
                    [Semaphore::new(vulkan), Semaphore::new(vulkan)];
                let mut command_pool = CommandPool::new(vulkan);
                let [main_command_buffer] = command_pool.allocate();

                Frame {
                    fence,
                    image_available_semaphore,
                    ready_to_present_semaphore,
                    command_pool,
                    main_command_buffer,
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to create frames in flight."));

        let default_queue_executor = QueueExecutor::new(vulkan, constants::DEFAULT_QUEUE);

        Self {
            frames,
            default_queue_executor,
            frame_index: 0,
        }
    }

    pub fn submit(&mut self, vulkan: &Vulkan, swapchain: &Swapchain) {
        let frame_index = self.frame_index;
        let current_frame = &mut self.frames[frame_index];

        // Wait for the current frame to be ready, any command buffers, etc... in flight will
        // become safe usable.
        current_frame.fence.wait();
        let swapchain_image_index =
            swapchain.get_next_image_index(&current_frame.image_available_semaphore);
        current_frame.fence.reset();

        current_frame.command_pool.reset();
        let main_command_buffer = current_frame
            .command_pool
            .get_mut(current_frame.main_command_buffer)
            .unwrap();

        main_command_buffer.begin();

        main_command_buffer.end();

        self.default_queue_executor.submit(QueueExecutorSubmitInfo {
            command_buffers: current_frame
                .command_pool
                .get_multiple_mut(vec![current_frame.main_command_buffer]),
            frame_index,
            wait_semaphores: vec![(
                &current_frame.image_available_semaphore,
                vk::PipelineStageFlags::TOP_OF_PIPE,
            )],
            signal_semaphores: vec![&current_frame.ready_to_present_semaphore],
            fence: Some(&current_frame.fence),
        });

        println!("Pre present");
        self.default_queue_executor.present(
            swapchain,
            swapchain_image_index,
            vec![&current_frame.ready_to_present_semaphore],
        );
        println!("Post present");

        self.frame_index = (self.frame_index + 1) % constants::FRAMES_IN_FLIGHT;
    }
}
