use async_graphql_parser::schema::Field;

use super::{Context, FileRender, RenderType};

pub struct RendererMutationType<'a, 'b> {
    pub doc: &'a Field,
    pub context: &'a Context<'b>,
}

impl<'a, 'b> FileRender for RendererMutationType<'a, 'b> {}

impl<'a, 'b> RenderType for RendererMutationType<'a, 'b> {
    #[must_use]
    fn name(&self) -> String {
        self.doc.name.node.clone()
    }

    #[must_use]
    fn description(&self) -> Option<&String> {
        match &self.doc.description {
            Some(_f) => panic!("Not Implemented"),
            _ => None,
        }
    }
}

impl<'a, 'b> RendererMutationType<'a, 'b> {}