/*!
Provides for writing a graph in the
[RDF 1.1 N-Triples](https://www.w3.org/TR/n-triples/), _a line-based syntax for an RDF graph_
format.

# Example

TBD

*/

use crate::GraphWriter;
use rdftk_graph::Graph;
use std::io::Write;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct NTripleWriter {
    inner: PhantomData<u8>,
}

pub const NAME: &str = "N-Triples";

pub const FILE_EXTENSION: &str = "nt";

pub const MIME_TYPE: &str = "application/n-triples";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for NTripleWriter {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<W: Write, G: Graph> GraphWriter<W, G> for NTripleWriter {
    fn write(&self, w: &mut W, graph: &G) -> std::io::Result<()> {
        for statement in graph.statements() {
            write!(
                w,
                "{} <{}> {} .",
                statement.subject(),
                statement.predicate(),
                statement.object()
            )?;
        }
        Ok(())
    }
}
