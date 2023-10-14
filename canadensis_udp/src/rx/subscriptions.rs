//! Subscription tracking

use alloc::collections::BTreeMap;

use canadensis_core::{ServiceId, SubjectId};

use crate::rx::Subscription;

pub struct Subscriptions<T> {
    /// Message subscriptions
    message: BTreeMap<SubjectId, Subscription<T>>,
    /// Service request subscriptions
    request: BTreeMap<ServiceId, Subscription<T>>,
    /// Service response subscriptions
    response: BTreeMap<ServiceId, Subscription<T>>,
}

impl<T> Subscriptions<T> {
    pub fn new() -> Self {
        Subscriptions {
            message: BTreeMap::new(),
            request: BTreeMap::new(),
            response: BTreeMap::new(),
        }
    }

    /// Stores a message subscription
    pub fn subscribe_message(&mut self, subject: SubjectId, subscription: Subscription<T>) {
        self.message.insert(subject, subscription);
    }
    /// Removes and returns a message subscription
    pub fn unsubscribe_message(&mut self, subject: SubjectId) -> Option<Subscription<T>> {
        self.message.remove(&subject)
    }
    /// Stores a service request subscription
    pub fn subscribe_request(&mut self, service: ServiceId, subscription: Subscription<T>) {
        self.request.insert(service, subscription);
    }
    /// Removes and returns a service request subscription
    pub fn unsubscribe_request(&mut self, service: ServiceId) -> Option<Subscription<T>> {
        self.request.remove(&service)
    }
    /// Stores a service response subscription
    pub fn subscribe_response(&mut self, service: ServiceId, subscription: Subscription<T>) {
        self.response.insert(service, subscription);
    }
    /// Removes and returns a service response subscription
    pub fn unsubscribe_response(&mut self, service: ServiceId) -> Option<Subscription<T>> {
        self.response.remove(&service)
    }

    pub fn find_message_subscription_mut(
        &mut self,
        subject: SubjectId,
    ) -> Option<&mut Subscription<T>> {
        self.message.get_mut(&subject)
    }
    pub fn find_request_subscription_mut(
        &mut self,
        service: ServiceId,
    ) -> Option<&mut Subscription<T>> {
        self.request.get_mut(&service)
    }
    pub fn find_response_subscription_mut(
        &mut self,
        service: ServiceId,
    ) -> Option<&mut Subscription<T>> {
        self.response.get_mut(&service)
    }

    /// Returns an iterator over mutable references to message subscriptions
    pub fn message_iter_mut(&mut self) -> MessageIterMut<'_, T> {
        MessageIterMut(self.message.values_mut())
    }
    /// Returns an iterator over mutable references to service request subscriptions
    pub fn request_iter_mut(&mut self) -> ServiceIterMut<'_, T> {
        ServiceIterMut(self.request.values_mut())
    }
    /// Returns an iterator over mutable references to service response subscriptions
    pub fn response_iter_mut(&mut self) -> ServiceIterMut<'_, T> {
        ServiceIterMut(self.response.values_mut())
    }

    /// Returns true if any request or response subscriptions exist
    pub fn any_service_subscriptions(&self) -> bool {
        !(self.request.is_empty() && self.response.is_empty())
    }
}

/// An iterator over mutable references to message subscriptions
pub struct MessageIterMut<'m, T>(
    alloc::collections::btree_map::ValuesMut<'m, SubjectId, Subscription<T>>,
);

impl<'m, T> Iterator for MessageIterMut<'m, T> {
    type Item = &'m mut Subscription<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
/// An iterator over mutable references to service subscriptions
pub struct ServiceIterMut<'m, T>(
    alloc::collections::btree_map::ValuesMut<'m, ServiceId, Subscription<T>>,
);

impl<'m, T> Iterator for ServiceIterMut<'m, T> {
    type Item = &'m mut Subscription<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
