// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Commands that the GPU will execute (includes draw commands).
//!
//! With Vulkan, before the GPU can do anything you must create a `CommandBuffer`. A command buffer
//! is a list of commands that will executed by the GPU. Once a command buffer is created, you can
//! execute it. A command buffer must be created even for the most simple tasks.
//!
//! # Pools
//!
//! Command buffers are allocated from pools. You must first create a command buffer pool which
//! you will create command buffers from.
//!
//! A pool is linked to a queue family. Command buffers that are created from a certain pool can
//! only be submitted to queues that belong to that specific family.
//!
//! # Primary and secondary command buffers.
//!
//! There are three types of command buffers:
//!
//! - **Primary command buffers**. They can contain any command. They are the only type of command
//!   buffer that can be submitted to a queue.
//! - **Secondary "graphics" command buffers**. They contain draw and clear commands. They can be
//!   called from a primary command buffer once a framebuffer has been selected.
//! - **Secondary "compute" command buffers**. They can contain non-draw and non-clear commands
//!   (eg. copying between buffers) and can be called from a primary command buffer outside of a
//!   render pass.
//!
//! Note that secondary command buffers cannot call other command buffers.
//!

// Implementation note.
// There are various restrictions about which command can be used at which moment. Therefore the
// API has several different command buffer wrappers, but they all use the same internal
// struct. The restrictions are enforced only in the public types.

pub use self::inner::Submission;
pub use self::outer::submit;
pub use self::outer::PrimaryCommandBufferBuilder;
pub use self::outer::PrimaryCommandBufferBuilderInlineDraw;
pub use self::outer::PrimaryCommandBufferBuilderSecondaryDraw;
pub use self::outer::PrimaryCommandBuffer;
pub use self::outer::SecondaryGraphicsCommandBufferBuilder;
pub use self::outer::SecondaryGraphicsCommandBuffer;
pub use self::outer::SecondaryComputeCommandBufferBuilder;
pub use self::outer::SecondaryComputeCommandBuffer;
pub use self::submit::CommandBuffer;
pub use self::submit::Submit;

use pipeline::viewport::Viewport;
use pipeline::viewport::Scissor;

mod inner;
mod outer;

pub mod pool;
pub mod std;
pub mod submit;
pub mod sys;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: u32,
    pub first_instance: u32,
}

/// The dynamic state to use for a draw command.
#[derive(Debug, Clone)]
pub struct DynamicState {
    pub line_width: Option<f32>,
    pub viewports: Option<Vec<Viewport>>,
    pub scissors: Option<Vec<Scissor>>,
}

impl DynamicState {
    #[inline]
    pub fn none() -> DynamicState {
        DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        }
    }
}

impl Default for DynamicState {
    #[inline]
    fn default() -> DynamicState {
        DynamicState::none()
    }
}
