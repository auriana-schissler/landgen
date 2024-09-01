use crate::render::RenderState;
use std::io;
use std::io::Write;
use std::sync::Arc;

pub(super) fn write_to<W: Write>(state: Arc<RenderState>, writer: &mut W) -> Result<(), io::Error> {
    for v in state.heightfield.read().unwrap().iter() {
        for h in v.iter() {
            for z in h.iter() {
                writer.write_all(&z.to_be_bytes())?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}
