#Ash

A very lightweight wrapper around Vulkan

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/ash/badge.svg)](https://docs.rs/ash)
[![Build Status](https://travis-ci.org/MaikKlein/ash.svg?branch=master)](https://travis-ci.org/MaikKlein/ash)
[![Build status](https://ci.appveyor.com/api/projects/status/ed7757as3a4ebexn/branch/master?svg=true)](https://ci.appveyor.com/project/MaikKlein/ash/branch/master)
[![Join the chat at https://gitter.im/MaikKlein/ash](https://badges.gitter.im/MaikKlein/ash.svg)](https://gitter.im/MaikKlein/ash?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Crates.io Version](https://img.shields.io/crates/v/ash.svg)](https://crates.io/crates/ash)
[![](https://tokei.rs/b1/github/maikklein/ash)](https://github.com/MaikKlein/ash)

## Stable yet?
I don't expect any big changes anymore. The library will still remain < 1.0 until I had time to use it in a real project. If you encounter any problems, feel free to open an [Issue](https://github.com/MaikKlein/ash/issues).

## Why Ash?
- [x] Lightweight Vulkan wrapper
- [x] Low overhead
- [x] Additional type safety
- [x] Trait based, version specific loader

## What does it do?

### Explicit returns with `Result`
Functions return a `type VkResult<T> = Result<T, vk::Result>` instead of an error code. No mutable references for the output are required.
```Rust
pub fn create_instance(&self,
                       create_info: &vk::InstanceCreateInfo,
                       allocation_callbacks: Option<&vk::AllocationCallbacks>)
                       -> Result<Instance, InstanceError> {
let instance = entry.create_instance(&create_info, None)
    .expect("Instance creation error");
```


Always returns a `Vec<T>` for functions that output multiple values.
```Rust
pub fn get_swapchain_images_khr(&self,
                                swapchain: vk::SwapchainKHR)
                                -> VkResult<Vec<vk::Image>>;
let present_images = swapchain_loader.get_swapchain_images_khr(swapchain).unwrap();
```

### Slices
Ash always uses slices in functions.
```Rust
// C
void vkCmdPipelineBarrier(
    VkCommandBuffer                             commandBuffer,
    VkPipelineStageFlags                        srcStageMask,
    VkPipelineStageFlags                        dstStageMask,
    VkDependencyFlags                           dependencyFlags,
    uint32_t                                    memoryBarrierCount,
    const VkMemoryBarrier*                      pMemoryBarriers,
    uint32_t                                    bufferMemoryBarrierCount,
    const VkBufferMemoryBarrier*                pBufferMemoryBarriers,
    uint32_t                                    imageMemoryBarrierCount,
    const VkImageMemoryBarrier*                 pImageMemoryBarriers);

// Rust
pub fn cmd_pipeline_barrier(&self,
                            command_buffer: vk::CommandBuffer,
                            src_stage_mask: vk::PipelineStageFlags,
                            dst_stage_mask: vk::PipelineStageFlags,
                            dependency_flags: vk::DependencyFlags,
                            memory_barriers: &[vk::MemoryBarrier],
                            buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                            image_memory_barriers: &[vk::ImageMemoryBarrier]);

device.cmd_pipeline_barrier(setup_command_buffer,
                            vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                            vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                            vk::DependencyFlags::empty(),
                            &[],
                            &[],
                            &[layout_transition_barrier]);

// or

let slice = device.map_memory::<Vertex>(vertex_input_buffer_memory,
                          0,
                          vertex_input_buffer_info.size,
                          vk::MemoryMapFlags::empty())
    .unwrap();
slice.copy_from_slice(&vertices);
```

### Type safety
Ash still uses raw Vulkan structs. The only difference is type safety. Everything that can be an enum is an enum like `vk::StructureType`, flags are implemented similar to the `Bitflags` crate. Ash also follows the Rust style guide. The reason that Ash uses raw Vulkan structs is to be extensible, just like the Vulkan spec.
```Rust
let pool_create_info = vk::CommandPoolCreateInfo {
    s_type: vk::StructureType::CommandPoolCreateInfo,
    p_next: ptr::null(),
    flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
    queue_family_index: queue_family_index,
};
let pool = device.create_command_pool(&pool_create_info, None).unwrap();
```

Additionally pointers like `Instance`, `Device`, `Queue` etc are hidden behind a type. Those pointers can only be constructed from within `Ash` which eliminates some invalid API usage and has the benefit of making some functions in Vulkan **safe**.

### Function pointer loading
Ash also takes care of loading the function pointers. Function pointers are split into 3 categories, Entry, Instance and Device. The reason for not loading it into a global is that in Vulkan you can have multiple devices and each device will load its own function pointers to achieve better performance. Click [here](https://github.com/KhronosGroup/Vulkan-LoaderAndValidationLayers/blob/master/loader/LoaderAndLayerInterface.md) for more information.

Ash also manages multiple versions of Vulkan without any breakage. You will never run into any runtime error because you tried to access a function pointer that failed to load. Function pointers either load successfully or fail and return an error.

```Rust
use ash::{Device, Instance};
// Specifies the version that you want to load
use ash::version::V1_0;
// Those traits implement the version specific functions
use ash::version::{InstanceV1_0, DeviceV1_0, EntryV1_0};
let entry = Entry::<V1_0>::new().unwrap();
let instance = entry.create_instance(...).expect("Instance creation error.");
let device = instance.create_device(...).expect("Device creation error.");
```

A `V1_X` struct is used to indicate the version.
```Rust
// Define your types
type YourDevice = Device<V1_0>;
type YourInstance = Instance<V1_0>;
```

You can upgrade to a future version without any breakage.
```Rust
// For example, switching from V1_0 to V1_3 will not cause any breakage.
type YourDevice = Device<V1_3>;
type YourInstance = Instance<V1_3>;
```

A newer version can always be converted to an older version.
```Rust
let newer_device: Device<V1_5> = ...;
let older_device: Device<V1_0> = newer_device.into();
```

Or specify the *minimum* version that you require with a trait.
```Rust
fn do_something_with_a_device<Device: DeviceV1_0>(device: &Device){}
```

### Extension loading
Additionally, every Vulkan extension has to be loaded explicitly. You can find all extensions under [ash::extensions](https://github.com/MaikKlein/ash/tree/master/src/extensions). You still have to tell Vulkan which instance or device extensions you want to load.
```Rust
use ash::extensions::Swapchain;
let swapchain_loader = Swapchain::new(&instance, &device).expect("Unable to load swapchain");
let swapchain = swapchain_loader.create_swapchain_khr(&swapchain_create_info).unwrap();
```

### Support for extension names
```Rust
use ash::extensions::{Swapchain, XlibSurface, Surface, DebugReport};
#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr()
    ]
}
```

### Implicit handles
You don't have to pass an Instance or Device handle anymore, this is done implicitly for you. This makes sure that you will always use the most optimal implementation for your `Device`.
```Rust
// C
VkResult vkCreateCommandPool(
    VkDevice                                    device,
    const VkCommandPoolCreateInfo*              pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkCommandPool*                              pCommandPool);

// Rust
pub fn create_command_pool(&self,
                           create_info: &vk::CommandPoolCreateInfo)
                           -> VkResult<vk::CommandPool>;

let pool = device.create_command_pool(&pool_create_info).unwrap();
```

## Example
You can find the examples [here](https://github.com/MaikKlein/ash/tree/master/examples).
All examples currently require: the LunarG Validation layers and a Vulkan library that is visible in your `PATH`. An easy way to get started is to use the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/)
#### Windows
Make sure that you have a Vulkan ready driver and install the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/).
#### Linux
Make sure that you have a Vulkan ready driver and install the [LunarG Vulkan SDK](https://lunarg.com/vulkan-sdk/). You also have to add the library and layers to your path. Have a look at my [post](http://askubuntu.com/a/803110/77183) if you are unsure how to do that.
### [Triangle](https://github.com/MaikKlein/ash/blob/master/examples/src/bin/triangle.rs)
Displays a triangle with vertex colors.
```
cd examples
cargo run --bin triangle
```

![screenshot](http://i.imgur.com/PQZcL6w.jpg)

### [Texture](https://github.com/MaikKlein/ash/blob/master/examples/src/bin/texture.rs)
Displays a texture on a quad.
```
cd examples
cargo run --bin texture
```
![texture](http://i.imgur.com/trow00H.png)

## Roadmap

### Extensions
- [x] Swapchain
- [x] Surface
- [x] XlibSurface
- [x] DebugReport
- [x] Win32Surface
- [x] MirSurface
- [x] XcbSurface
- [x] AndroidSurface
- [x] WaylandSurface
- [ ] Display

### In progress
- Wrapping the complete spec
- Version specific loader

## A thanks to

* [Api with no secrets](https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-1)
* [Vulkan tutorial](http://av.dfki.de/~jhenriques/development.html)
* [Vulkan examples](https://github.com/SaschaWillems/Vulkan)
* [Vulkan tutorial](https://vulkan-tutorial.com/)
* [Vulkano](https://github.com/tomaka/vulkano/)
* [vk-rs](https://github.com/Osspial/vk-rs)
