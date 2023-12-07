use ash::vk;
use pyrite::app::resource::Resource;
use pyrite::vulkan::objects::{CommandBuffer, CommandBufferHandle, CommandPool};
use pyrite::vulkan::util::VulkanGenericResourceDep;
use pyrite::vulkan::{Vulkan, VulkanDep};

use crate::constants::{self, FRAMES_IN_FLIGHT};

pub struct Frame {
    command_pool: CommandPool,
    main_command_buffer: CommandBufferHandle,
    in_flight_resources: Vec<VulkanGenericResourceDep>,
}

#[derive(Resource)]
pub struct RenderManager {
    vulkan_dep: VulkanDep,
    frames: [Frame; FRAMES_IN_FLIGHT],
    frame_index: usize,
}

impl RenderManager {
    pub fn new(vulkan: &Vulkan) -> Self {
        let frames = (0..FRAMES_IN_FLIGHT)
            .map(|_| {
                let mut command_pool = CommandPool::new(vulkan);
                let [main_command_buffer] = command_pool.allocate();
                let in_flight_resources = Vec::new();

                Frame {
                    command_pool,
                    main_command_buffer,
                    in_flight_resources,
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to create frames in flight."));

        Self {
            vulkan_dep: vulkan.create_dep(),
            frames,
            frame_index: 0,
        }
    }

    pub fn submit(&mut self) {
        let default_queue = self.vulkan_dep.default_queue();

        let frame_index = self.frame_index;
        unsafe {
            self.vulkan_dep
                .device()
                .queue_wait_idle(default_queue.queue())
                .expect("Failed to wait for queue");
        }

        let current_frame = &mut self.frames[frame_index];

        current_frame.command_pool.reset();
        let main_command_buffer = current_frame
            .command_pool
            .get(current_frame.main_command_buffer)
            .unwrap();

        main_command_buffer.begin();
        main_command_buffer.end();

        unsafe {
            let command_buffers = [main_command_buffer.command_buffer()];
            let submit_info = vk::SubmitInfo::default().command_buffers(&command_buffers);
            self.vulkan_dep
                .device()
                .queue_submit(default_queue.queue(), &[submit_info], vk::Fence::null())
                .expect("Failed to submit queue");
        }

        self.frame_index = (self.frame_index + 1) % constants::FRAMES_IN_FLIGHT;
    }
}
