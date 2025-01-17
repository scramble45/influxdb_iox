//! A NOP implementation of [`DmlHandler`].

use std::{fmt::Debug, marker::PhantomData};

use async_trait::async_trait;
use data_types::{DatabaseName, DeletePredicate, NamespaceId};
use observability_deps::tracing::*;
use trace::ctx::SpanContext;

use super::{DmlError, DmlHandler};

/// A [`DmlHandler`] implementation that does nothing.
#[derive(Debug)]
pub struct NopDmlHandler<T>(PhantomData<T>);

impl<T> Default for NopDmlHandler<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> DmlHandler for NopDmlHandler<T>
where
    T: Debug + Send + Sync,
{
    type WriteError = DmlError;
    type DeleteError = DmlError;
    type WriteInput = T;
    type WriteOutput = T;

    async fn write(
        &self,
        namespace: &DatabaseName<'static>,
        namespace_id: NamespaceId,
        batches: Self::WriteInput,
        _span_ctx: Option<SpanContext>,
    ) -> Result<Self::WriteOutput, Self::WriteError> {
        info!(%namespace, %namespace_id, ?batches, "dropping write operation");
        Ok(batches)
    }

    async fn delete(
        &self,
        namespace: &DatabaseName<'static>,
        table_name: &str,
        predicate: &DeletePredicate,
        _span_ctx: Option<SpanContext>,
    ) -> Result<(), Self::DeleteError> {
        info!(%namespace, %table_name, ?predicate, "dropping delete operation");
        Ok(())
    }
}
