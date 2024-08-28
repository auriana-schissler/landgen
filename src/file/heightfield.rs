use std::io;
use std::io::Write;
use std::sync::Arc;
use crate::render::RenderState;

pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    unimplemented!()
}