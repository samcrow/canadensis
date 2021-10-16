//! Subscription management for transport receivers

use alloc::vec::Vec;
use fallible_collections::FallibleVec;

use crate::transfer::{Header, MessageHeader, ServiceHeader};
use crate::transport::Transport;
use crate::{OutOfMemoryError, ServiceId, SubjectId};

/// Something that can keep track of active subscriptions
pub trait SubscriptionManager<S> {
    /// Stores a message subscription
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError>;
    /// Removes and returns a message subscription
    fn unsubscribe_message(&mut self, subject: SubjectId) -> Option<S>;
    /// Stores a service request subscription
    fn subscribe_request(
        &mut self,
        service: ServiceId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError>;
    /// Removes and returns a service request subscription
    fn unsubscribe_request(&mut self, service: ServiceId) -> Option<S>;
    /// Stores a service response subscription
    fn subscribe_response(
        &mut self,
        service: ServiceId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError>;
    /// Removes and returns a service response subscription
    fn unsubscribe_response(&mut self, service: ServiceId) -> Option<S>;

    /// Returns a subscription corresponding to the provided header, if one exists
    fn find_subscription<I, T: Transport>(&self, header: &Header<I, T>) -> Option<&S> {
        match header {
            Header::Message(MessageHeader { subject, .. }) => {
                self.find_message_subscription(*subject)
            }
            Header::Request(ServiceHeader { service, .. }) => {
                self.find_request_subscription(*service)
            }
            Header::Response(ServiceHeader { service, .. }) => {
                self.find_response_subscription(*service)
            }
        }
    }
    /// Returns a subscription corresponding to the provided header, if one exists
    fn find_subscription_mut<I, T: Transport>(&mut self, header: &Header<I, T>) -> Option<&mut S> {
        match header {
            Header::Message(MessageHeader { subject, .. }) => {
                self.find_message_subscription_mut(*subject)
            }
            Header::Request(ServiceHeader { service, .. }) => {
                self.find_request_subscription_mut(*service)
            }
            Header::Response(ServiceHeader { service, .. }) => {
                self.find_response_subscription_mut(*service)
            }
        }
    }

    /// Returns a reference to the message subscription matching the provided subject
    fn find_message_subscription(&self, subject: SubjectId) -> Option<&S>;
    /// Returns a mutable reference to the message subscription matching the provided subject
    fn find_message_subscription_mut(&mut self, subject: SubjectId) -> Option<&mut S>;
    /// Returns a reference to the service request subscription matching the provided subject
    fn find_request_subscription(&self, service: ServiceId) -> Option<&S>;
    /// Returns a mutable reference to the service request subscription matching the provided subject
    fn find_request_subscription_mut(&mut self, service: ServiceId) -> Option<&mut S>;
    /// Returns a reference to the service response subscription matching the provided subject
    fn find_response_subscription(&self, service: ServiceId) -> Option<&S>;
    /// Returns a mutable reference to the service response subscription matching the provided subject
    fn find_response_subscription_mut(&mut self, service: ServiceId) -> Option<&mut S>;

    /// Executes the provided operation for each message subscription
    fn for_each_message_subscription_mut<F, R>(&mut self, operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>;
    /// Executes the provided operation for each request subscription
    fn for_each_request_subscription_mut<F, R>(&mut self, operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>;
    /// Executes the provided operation for each response subscription
    fn for_each_response_subscription_mut<F, R>(&mut self, operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>;
}

/// A subscription manager that dynamically allocates memory
pub struct DynamicSubscriptionManager<S> {
    message_subscriptions: Vec<(SubjectId, S)>,
    request_subscriptions: Vec<(ServiceId, S)>,
    response_subscriptions: Vec<(ServiceId, S)>,
}

impl<S> SubscriptionManager<S> for DynamicSubscriptionManager<S> {
    fn subscribe_message(
        &mut self,
        subject: SubjectId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_message(subject);
        FallibleVec::try_push(&mut self.message_subscriptions, (subject, subscription))?;
        Ok(())
    }

    fn unsubscribe_message(&mut self, subject: SubjectId) -> Option<S> {
        if let Some(index) = self
            .message_subscriptions
            .iter()
            .position(|(stored_subject, _)| *stored_subject == subject)
        {
            let (_, subscription) = self.message_subscriptions.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }

    fn subscribe_request(
        &mut self,
        service: ServiceId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_request(service);
        FallibleVec::try_push(&mut self.request_subscriptions, (service, subscription))?;
        Ok(())
    }

    fn unsubscribe_request(&mut self, service: ServiceId) -> Option<S> {
        if let Some(index) = self
            .request_subscriptions
            .iter()
            .position(|(stored_service, _)| *stored_service == service)
        {
            let (_, subscription) = self.request_subscriptions.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }

    fn subscribe_response(
        &mut self,
        service: ServiceId,
        subscription: S,
    ) -> Result<(), OutOfMemoryError> {
        self.unsubscribe_response(service);
        FallibleVec::try_push(&mut self.response_subscriptions, (service, subscription))?;
        Ok(())
    }

    fn unsubscribe_response(&mut self, service: ServiceId) -> Option<S> {
        if let Some(index) = self
            .response_subscriptions
            .iter()
            .position(|(stored_service, _)| *stored_service == service)
        {
            let (_, subscription) = self.response_subscriptions.swap_remove(index);
            Some(subscription)
        } else {
            None
        }
    }

    fn find_message_subscription(&self, subject: SubjectId) -> Option<&S> {
        self.message_subscriptions
            .iter()
            .find(|(sub_subject, _)| *sub_subject == subject)
            .map(|(_, sub)| sub)
    }

    fn find_message_subscription_mut(&mut self, subject: SubjectId) -> Option<&mut S> {
        self.message_subscriptions
            .iter_mut()
            .find(|(sub_subject, _)| *sub_subject == subject)
            .map(|(_, sub)| sub)
    }

    fn find_request_subscription(&self, service: ServiceId) -> Option<&S> {
        self.request_subscriptions
            .iter()
            .find(|(sub_service, _)| *sub_service == service)
            .map(|(_, sub)| sub)
    }

    fn find_request_subscription_mut(&mut self, service: ServiceId) -> Option<&mut S> {
        self.request_subscriptions
            .iter_mut()
            .find(|(sub_service, _)| *sub_service == service)
            .map(|(_, sub)| sub)
    }

    fn find_response_subscription(&self, service: ServiceId) -> Option<&S> {
        self.response_subscriptions
            .iter()
            .find(|(sub_service, _)| *sub_service == service)
            .map(|(_, sub)| sub)
    }

    fn find_response_subscription_mut(&mut self, service: ServiceId) -> Option<&mut S> {
        self.response_subscriptions
            .iter_mut()
            .find(|(sub_service, _)| *sub_service == service)
            .map(|(_, sub)| sub)
    }

    fn for_each_message_subscription_mut<F, R>(&mut self, mut operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>,
    {
        for (_, subscription) in &mut self.message_subscriptions {
            if let Some(result) = operation(subscription) {
                return Some(result);
            }
        }
        None
    }

    fn for_each_request_subscription_mut<F, R>(&mut self, mut operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>,
    {
        for (_, subscription) in &mut self.request_subscriptions {
            if let Some(result) = operation(subscription) {
                return Some(result);
            }
        }
        None
    }

    fn for_each_response_subscription_mut<F, R>(&mut self, mut operation: F) -> Option<R>
    where
        F: FnMut(&mut S) -> Option<R>,
    {
        for (_, subscription) in &mut self.response_subscriptions {
            if let Some(result) = operation(subscription) {
                return Some(result);
            }
        }
        None
    }
}

impl<S> Default for DynamicSubscriptionManager<S> {
    fn default() -> Self {
        DynamicSubscriptionManager {
            message_subscriptions: Default::default(),
            request_subscriptions: Default::default(),
            response_subscriptions: Default::default(),
        }
    }
}
