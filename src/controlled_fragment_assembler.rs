use std::collections::HashMap;
use std::marker::PhantomData;

use agrona::buffer::{AtomicBuffer, BufferBuilder};
use agrona::collections::HashMap as AgronaHashMap;
use agrona::util::BitUtil;

use crate::logbuffer::{ControlledFragmentHandler, Header};
use crate::protocol::{DataHeaderFlyweight, FrameDescriptor};

struct ControlledFragmentAssembler<'a, F>
where
    F: ControlledFragmentHandler,
{
    delegate: F,
    builder_by_session_id_map: AgronaHashMap<i32, BufferBuilder<'a>>,
    initial_buffer_length: usize,
    is_direct_byte_buffer: bool,
}

impl<'a, F> ControlledFragmentAssembler<'a, F>
where
    F: ControlledFragmentHandler,
{
    fn new(delegate: F) -> Self {
        ControlledFragmentAssembler {
            delegate,
            builder_by_session_id_map: HashMap::new(),
            initial_buffer_length: 0,
            is_direct_byte_buffer: false,
        }
    }

    fn with_initial_buffer_length(delegate: F, initial_buffer_length: usize) -> Self {
        ControlledFragmentAssembler {
            delegate,
            builder_by_session_id_map: HashMap::new(),
            initial_buffer_length,
            is_direct_byte_buffer: false,
        }
    }

    fn with_direct_byte_buffer(
        delegate: F,
        initial_buffer_length: usize,
        is_direct_byte_buffer: bool,
    ) -> Self {
        ControlledFragmentAssembler {
            delegate,
            builder_by_session_id_map: HashMap::new(),
            initial_buffer_length,
            is_direct_byte_buffer,
        }
    }

    fn delegate(&self) -> &F {
        &self.delegate
    }

    fn is_direct_byte_buffer(&self) -> bool {
        self.is_direct_byte_buffer
    }

    fn on_fragment(
        &mut self,
        buffer: &AtomicBuffer,
        offset: usize,
        length: usize,
        header: &Header,
    ) -> ControlledFragmentHandler::Action {
        let flags = header.flags();
        let mut action = ControlledFragmentHandler::Action::Continue;

        if (flags & FrameDescriptor::UNFRAGMENTED) == FrameDescriptor::UNFRAGMENTED {
            action = self.delegate.on_fragment(buffer, offset, length, header);
        } else if (flags & FrameDescriptor::BEGIN_FRAG_FLAG) == FrameDescriptor::BEGIN_FRAG_FLAG {
            let builder = self.get_buffer_builder(header.session_id());
            builder
                .reset()
                .append(buffer, offset, length)
                .next_term_offset(BitUtil::align(
                    offset + length + DataHeaderFlyweight::HEADER_LENGTH,
                    FrameDescriptor::FRAME_ALIGNMENT,
                ));
        } else {
            let builder = self.builder_by_session_id_map.get_mut(&header.session_id());
            if let Some(builder) = builder {
                if offset == builder.next_term_offset() {
                    let limit = builder.limit();

                    builder.append(buffer, offset, length);

                    if (flags & FrameDescriptor::END_FRAG_FLAG) == FrameDescriptor::END_FRAG_FLAG {
                        action = self.delegate.on_fragment(
                            builder.buffer(),
                            0,
                            builder.limit(),
                            header,
                        );

                        if action == ControlledFragmentHandler::Action::Abort {
                            builder.limit(limit);
                        } else {
                            builder.reset();
                        }
                    } else {
                        builder.next_term_offset(BitUtil::align(
                            offset + length + DataHeaderFlyweight::HEADER_LENGTH,
                            FrameDescriptor::FRAME_ALIGNMENT,
                        ));
                    }
                } else {
                    builder.reset();
                }
            }
        }

        action
    }

    fn free_session_buffer(&mut self, session_id: i32) -> bool {}
}

