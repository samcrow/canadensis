//! Subscription tracking

use crate::rx::Subscription;
use canadensis_core::time::Instant;
use canadensis_core::{OutOfMemoryError, ServiceId, SubjectId};
use core::slice;
use fallible_collections::FallibleVec;
use std::iter::Map;

pub struct Subscriptions<I, T>
where
    I: Instant,
{
    /// Message subscriptions
    message: Vec<(SubjectId, Subscription<I, T>)>,
    /// Service request subscriptions
    request: Vec<(ServiceId, Subscription<I, T>)>,
    /// Service response subscriptions
    response: Vec<(ServiceId, Subscription<I, T>)>,
}

impl<I, T> Subscriptions<I, T>
where
    I: Instant,
{
    pub fn new() -> Self {
        Subscriptions {
            message: Vec::new(),
            request: Vec::new(),
            response: Vec::new(),
        }
    }

    /// Stores a message subscription
    pub fn subscribe_message(
        &mut self,
        subject: SubjectId,
        subscription: Subscription<I, T>,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_message(subject);
        FallibleVec::try_push(&mut self.message, (subject, subscription))?;
        Ok(())
    }
    /// Removes and returns a message subscription
    pub fn unsubscribe_message(&mut self, subject: SubjectId) -> Option<Subscription<I, T>> {
        if let Some(index) = self
            .message
            .iter()
            .position(|(stored_subject, _)| *stored_subject == subject)
        {
            let (_, subscription) = self.message.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }
    /// Stores a service request subscription
    pub fn subscribe_request(
        &mut self,
        service: ServiceId,
        subscription: Subscription<I, T>,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_request(service);
        FallibleVec::try_push(&mut self.request, (service, subscription))?;
        Ok(())
    }
    /// Removes and returns a service request subscription
    pub fn unsubscribe_request(&mut self, service: ServiceId) -> Option<Subscription<I, T>> {
        if let Some(index) = self
            .request
            .iter()
            .position(|(stored_service, _)| *stored_service == service)
        {
            let (_, subscription) = self.request.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }
    /// Stores a service response subscription
    pub fn subscribe_response(
        &mut self,
        service: ServiceId,
        subscription: Subscription<I, T>,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_response(service);
        FallibleVec::try_push(&mut self.response, (service, subscription))?;
        Ok(())
    }
    /// Removes and returns a service response subscription
    pub fn unsubscribe_response(&mut self, service: ServiceId) -> Option<Subscription<I, T>> {
        if let Some(index) = self
            .response
            .iter()
            .position(|(stored_service, _)| *stored_service == service)
        {
            let (_, subscription) = self.response.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }

    /// Returns an iterator over mutable references to message subscriptions
    pub fn message_iter_mut(&mut self) -> MessageIterMut<'_, I, T> {
        MessageIterMut(self.message.iter_mut().map(message_tuple_to_subscription))
    }
    /// Returns an iterator over mutable references to service request subscriptions
    pub fn request_iter_mut(&mut self) -> ServiceIterMut<'_, I, T> {
        ServiceIterMut(self.request.iter_mut().map(service_tuple_to_subscription))
    }
    /// Returns an iterator over mutable references to service response subscriptions
    pub fn response_iter_mut(&mut self) -> ServiceIterMut<'_, I, T> {
        ServiceIterMut(self.response.iter_mut().map(service_tuple_to_subscription))
    }
}

/// An iterator over mutable references to message subscriptions
pub struct MessageIterMut<'m, I, T>(
    Map<
        slice::IterMut<'m, (SubjectId, Subscription<I, T>)>,
        fn(&mut (SubjectId, Subscription<I, T>)) -> &mut Subscription<I, T>,
    >,
)
where
    I: Instant;

impl<'m, I, T> Iterator for MessageIterMut<'m, I, T>
where
    I: Instant,
{
    type Item = &'m mut Subscription<I, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
/// An iterator over mutable references to service subscriptions
pub struct ServiceIterMut<'m, I, T>(
    Map<
        slice::IterMut<'m, (ServiceId, Subscription<I, T>)>,
        fn(&mut (ServiceId, Subscription<I, T>)) -> &mut Subscription<I, T>,
    >,
)
where
    I: Instant;

impl<'m, I, T> Iterator for ServiceIterMut<'m, I, T>
where
    I: Instant,
{
    type Item = &'m mut Subscription<I, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

fn message_tuple_to_subscription<I, T>(
    (_, subscription): &mut (SubjectId, Subscription<I, T>),
) -> &mut Subscription<I, T>
where
    I: Instant,
{
    subscription
}
fn service_tuple_to_subscription<I, T>(
    (_, subscription): &mut (ServiceId, Subscription<I, T>),
) -> &mut Subscription<I, T>
where
    I: Instant,
{
    subscription
}
