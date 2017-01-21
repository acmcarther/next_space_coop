// Copyright (c) 2013-2015 Sandstorm Development Group, Inc. and contributors
// Licensed under the MIT License:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::cell::Cell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use std::slice;
use std::u64;

use private::units::*;
use message;
use message::{Allocator, ReaderSegments};
use {Error, OutputSegments, Result, Word};



pub type SegmentId = u32;

pub struct SegmentReader {
    pub arena: ArenaPtr,
    pub ptr: *const Word,
    pub size: WordCount32,
    pub read_limiter: Rc<ReadLimiter>,
}

impl SegmentReader {
    #[inline]
    pub unsafe fn get_start_ptr(&self) -> *const Word {
        self.ptr.offset(0)
    }

    #[inline]
    pub fn contains_interval(&self, from: *const Word, to: *const Word) -> bool {
        let this_begin: usize = self.ptr as usize;
        let this_end: usize = unsafe { self.ptr.offset(self.size as isize) as usize };
        from as usize >= this_begin && to as usize <= this_end && from as usize <= to as usize &&
            self.read_limiter.can_read((to as usize - from as usize) as u64 / BYTES_PER_WORD as u64)
    }

    #[inline]
    pub fn amplified_read(&self, virtual_amount: u64) -> bool {
        self.read_limiter.can_read(virtual_amount)
    }
}

pub struct SegmentBuilder {
    pub reader: SegmentReader,
    pub id: SegmentId,
    pos: *mut Word,
}

unsafe impl Send for SegmentBuilder {}

impl SegmentBuilder {
    pub fn new(arena: *mut BuilderArena,
               limiter: Rc<ReadLimiter>,
               id: SegmentId,
               ptr: *mut Word,
               size: WordCount32) -> SegmentBuilder {
        SegmentBuilder {
            reader: SegmentReader {
                arena: ArenaPtr::Builder(arena),
                ptr: unsafe {mem::transmute(ptr)},
                size: size,
                read_limiter: limiter,
            },
            id: id,
            pos: ptr,
        }
    }

    pub fn get_word_offset_to(&mut self, ptr: *mut Word) -> WordCount32 {
        let this_addr: usize = self.reader.ptr as usize;
        let ptr_addr: usize = ptr as usize;
        assert!(ptr_addr >= this_addr);
        let result = (ptr_addr - this_addr) / BYTES_PER_WORD;
        result as u32
    }

    #[inline]
    pub fn current_size(&self) -> WordCount32 {
        ((self.pos as usize - self.reader.ptr as usize) / BYTES_PER_WORD) as u32
    }

    #[inline]
    pub fn allocate(&mut self, amount: WordCount32) -> Option<*mut Word> {
        if amount > self.reader.size - self.current_size() {
            None
        } else {
            let result = self.pos;
            self.pos = unsafe { self.pos.offset(amount as isize) };
            Some(result)
        }
    }

    #[inline]
    pub fn get_ptr_unchecked(&self, offset: WordCount32) -> *mut Word {
        unsafe {
            mem::transmute(self.reader.ptr.offset(offset as isize))
        }
    }

    #[inline]
    pub fn get_segment_id(&self) -> SegmentId { self.id }

    #[inline]
    pub fn get_arena<'a> (&'a mut self) -> *mut BuilderArena {
        match self.reader.arena {
            ArenaPtr::Builder(b) => b,
            _ => unreachable!()
        }
    }

    pub fn currently_allocated<'a>(&'a self) -> &'a [Word] {
        unsafe { slice::from_raw_parts(self.get_ptr_unchecked(0), self.current_size() as usize) }
    }
}

pub struct ReadLimiter {
    pub limit: Cell<u64>,
}

impl ReadLimiter {
    pub fn new(limit: u64) -> ReadLimiter {
        ReadLimiter { limit: Cell::new(limit) }
    }

    #[inline]
    pub fn can_read(&self, amount: u64) -> bool {
        let current = self.limit.get();
        if amount > current {
            // TODO arena->reportReadLimitReached()
            false
        } else {
            self.limit.set(current - amount);
            true
        }
    }
}

pub struct ReaderArena {
    raw_segments: &'static ReaderSegments,
    pub segment0: SegmentReader,
    pub more_segments: HashMap<SegmentId, Box<SegmentReader>>,
    pub read_limiter: Rc<ReadLimiter>,
}

impl ReaderArena {
    pub fn new(segments: &'static ReaderSegments,
               options: message::ReaderOptions)
               -> Box<ReaderArena> {

        let limiter = Rc::new(ReadLimiter::new(options.traversal_limit_in_words));

        let segment0 = segments.get_segment(0).expect("segment zero does not exist");

        let segment0_reader =  SegmentReader {
            arena: ArenaPtr::Null,
            ptr: unsafe { segment0.get_unchecked(0) },
            size: segment0.len() as u32,
            read_limiter: limiter.clone(),
        };

        let mut arena = Box::new(ReaderArena {
            raw_segments: segments,
            segment0: segment0_reader,
            more_segments: HashMap::new(),
            read_limiter: limiter.clone(),
        });

        let arena_ptr = ArenaPtr::Reader(&mut *arena);
        arena.segment0.arena = arena_ptr;
        arena
    }

    fn try_get_segment(&mut self, id: SegmentId) -> Result<*const SegmentReader> {
        if id == 0 {
            Ok(&self.segment0)
        } else if self.more_segments.contains_key(&id) {
            Ok(&*self.more_segments[&id])
        } else {
            let cloned_limiter = self.read_limiter.clone();
            let new_segment = match self.raw_segments.get_segment(id) {
                Some(s) => s,
                None => {
                    return Err(Error::failed(format!("Invalid segment id: {}", id)));
                }
            };
            let new_segment_reader = SegmentReader {
                arena: ArenaPtr::Reader(&mut *self),
                ptr: unsafe { new_segment.get_unchecked(0) },
                size: new_segment.len() as u32,
                read_limiter: cloned_limiter
            };
            self.more_segments.insert(id, Box::new(new_segment_reader));
            Ok(&*self.more_segments[&id])
        }
    }
}

pub struct BuilderArena {
    allocator: &'static mut Allocator,
    pub segment0: SegmentBuilder,
    pub more_segments: Vec<Box<SegmentBuilder>>,
    pub dummy_limiter: Rc<ReadLimiter>,
}

impl BuilderArena  {
    pub fn new(allocator: &'static mut Allocator) -> Box<BuilderArena> {
        let limiter = Rc::new(ReadLimiter::new(u64::MAX));
        let (first_segment, num_words) = allocator.allocate_segment(2);

        let mut result = Box::new(BuilderArena {
            allocator: allocator,
            segment0: SegmentBuilder {
                reader: SegmentReader {
                    ptr: first_segment,
                    size: num_words,
                    arena: ArenaPtr::Null,
                    read_limiter: limiter.clone()},
                id: 0,
                pos: first_segment,
            },
            more_segments: Vec::new(),
            dummy_limiter: limiter,
        });

        let arena_ptr = ArenaPtr::Builder(&mut *result);
        result.segment0.reader.arena = arena_ptr;
        result
    }

    pub fn try_get_segment(&self, id: SegmentId) -> Result<*const SegmentReader> {
        if id == 0 {
            Ok(&self.segment0.reader)
        } else if ((id - 1) as usize) < self.more_segments.len() {
            Ok(&self.more_segments[(id - 1) as usize].reader)
        } else {
            Err(Error::failed(format!("Invalid segment id: {}", id)))
        }
    }

    #[inline]
    pub fn allocate(&mut self, amount: WordCount32) -> (*mut SegmentBuilder, *mut Word) {
        unsafe {
            match self.segment0.allocate(amount) {
                Some(result) => { return (&mut self.segment0, result) }
                None => {}
            }

            //# Need to fall back to additional segments.

            let id = {
                let len = self.more_segments.len();
                if len == 0 { 1 }
                else {
                    let result_ptr: *mut SegmentBuilder = &mut *self.more_segments[len-1];
                    match self.more_segments[len - 1].allocate(amount) {
                        Some(result) => { return (result_ptr, result) }
                        None => { len + 1 }
                    }
                }};

            let (words, size) = self.allocator.allocate_segment(amount);
            let mut new_builder = Box::new(SegmentBuilder::new(self, self.dummy_limiter.clone(),
                                                               id as u32, words, size));
            let builder_ptr: *mut SegmentBuilder = &mut *new_builder;

            self.more_segments.push(new_builder);

            (builder_ptr, (*builder_ptr).allocate(amount).unwrap() )
        }
    }

    pub fn get_segment(&mut self, id: SegmentId) -> Result<*mut SegmentBuilder> {
        if id == 0 {
            Ok(&mut self.segment0)
        } else if ((id - 1) as usize) < self.more_segments.len() {
            Ok(&mut *self.more_segments[(id - 1) as usize])
        } else {
            Err(Error::failed(format!("Invalid segment id: {}", id)))
        }
    }

    pub fn get_segments_for_output<'a>(&'a self) -> OutputSegments<'a> {
        if self.more_segments.len() == 0 {
            OutputSegments::SingleSegment([self.segment0.currently_allocated()])
        } else {
            let mut v = Vec::new();
            v.push(self.segment0.currently_allocated());
            for seg in self.more_segments.iter() {
                v.push(seg.currently_allocated());
            }
            OutputSegments::MultiSegment(v)
        }
    }
}

#[derive(Clone, Copy)]
pub enum ArenaPtr {
    Reader(*mut ReaderArena),
    Builder(*mut BuilderArena),
    Null
}

impl ArenaPtr {
    pub fn try_get_segment(&self, id: SegmentId) -> Result<*const SegmentReader> {
        unsafe {
            match self {
                &ArenaPtr::Reader(reader) => {
                    (&mut *reader).try_get_segment(id)
                }
                &ArenaPtr::Builder(builder) => {
                    (&*builder).try_get_segment(id)
                }
                &ArenaPtr::Null => {
                    Err(Error::failed("Null arena.".to_string()))
                }
            }
        }
    }
}
