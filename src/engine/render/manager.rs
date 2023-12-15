use ash::vk;
use pyrite::app::resource::Resource;
use pyrite::vulkan::executor::{QueueExecutor, QueueExecutorSubmitInfo};
use pyrite::vulkan::objects::{
    CommandBufferHandle, CommandPool, DescriptorSetLayout, DescriptorSetLayoutBuilder,
    DescriptorSetPool, Fence, ImageMemoryBarrier, Semaphore,
};
use pyrite::vulkan::swapchain::{Swapchain, SwapchainError};
use pyrite::vulkan::Vulkan;
use pyrite::window::Window;

use crate::constants::{self, FRAMES_IN_FLIGHT};

use super::util::create_swapchain_info;

#[derive(Resource)]
pub struct FrameIndex(usize);

impl FrameIndex {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn increment(&mut self) {
        self.0 = (self.0 + 1) % FRAMES_IN_FLIGHT;
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

pub struct Frame {
    fence: Fence,
    image_available_semaphore: Semaphore,
    ready_to_present_semaphore: Semaphore,
    command_pool: CommandPool,
    main_command_buffer: CommandBufferHandle,
}

#[derive(Resource)]
pub struct RenderManager {
    frame_resources: [Frame; FRAMES_IN_FLIGHT],
    default_queue_executor: QueueExecutor<FRAMES_IN_FLIGHT>,
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
            frame_resources: frames,
            default_queue_executor,
        }
    }

    pub fn submit(
        &mut self,
        vulkan: &Vulkan,
        swapchain: &mut Swapchain,
        window: &Window,
        frame_index: usize,
    ) {
        let current_frame = &mut self.frame_resources[frame_index];

        // Wait for the current frame to be ready, any command buffers, etc... in flight will
        // become safe usable.
        current_frame.fence.wait();

        let swapchain_image_index =
            match swapchain.get_next_image_index(&current_frame.image_available_semaphore) {
                Ok(image_index) => image_index,
                Err(error) => match error {
                    SwapchainError::OutOfDate | SwapchainError::SubOptimal => {
                        swapchain.refresh(vulkan, &create_swapchain_info(window));
                        return;
                    }
                    _ => {
                        panic!("Failed to get next image index: {:?}", error);
                    }
                },
            };

        // Reset the fence and release the frame resources.
        current_frame.fence.reset();
        self.default_queue_executor
            .release_frame_resources(frame_index);

        current_frame.command_pool.reset();
        let main_command_buffer = current_frame
            .command_pool
            .get_mut(current_frame.main_command_buffer)
            .unwrap();

        main_command_buffer.begin();

        main_command_buffer.pipeline_barrier(
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vec![ImageMemoryBarrier {
                image: swapchain.image(swapchain_image_index as usize),
                old_layout: vk::ImageLayout::UNDEFINED,
                new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                src_access_mask: vk::AccessFlags::empty(),
                dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
            }],
        );

        // CLEAR COLOR
        main_command_buffer.clear_color_image(
            swapchain.image(swapchain_image_index as usize),
            vk::ClearColorValue {
                float32: [1.0, 0.0, 0.0, 1.0],
            },
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1),
        );

        main_command_buffer.pipeline_barrier(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vec![ImageMemoryBarrier {
                image: swapchain.image(swapchain_image_index as usize),
                old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                dst_access_mask: vk::AccessFlags::MEMORY_READ,
            }],
        );

        main_command_buffer.end();

        self.default_queue_executor.submit(QueueExecutorSubmitInfo {
            command_buffers: current_frame
                .command_pool
                .get_multiple_mut(vec![current_frame.main_command_buffer]),
            frame_index,
            wait_semaphores: vec![(
                &current_frame.image_available_semaphore,
                vk::PipelineStageFlags::TRANSFER, // Only need the image during the transfer stage.
            )],
            signal_semaphores: vec![&current_frame.ready_to_present_semaphore],
            fence: Some(&current_frame.fence),
        });

        self.default_queue_executor.present(
            swapchain,
            swapchain_image_index,
            vec![&current_frame.ready_to_present_semaphore],
        );
    }
}

impl Drop for RenderManager {
    fn drop(&mut self) {
        self.default_queue_executor.wait_idle();
    }
}
