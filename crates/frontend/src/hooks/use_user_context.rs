use std::fmt;
use std::ops::Deref;

use types::user::ResponseUser;
use yew::prelude::*;

/// State handle for the [`use_user_context`] hook.
pub struct UseUserContextHandle {
    inner: UseStateHandle<ResponseUser>
}

impl UseUserContextHandle {
    pub fn set_user(&self, value: ResponseUser) {
        self.inner.set(value);
    }

    pub fn clear_user(&self) {
        self.inner.set(ResponseUser::default());
    }
}

impl Deref for UseUserContextHandle {
    type Target = ResponseUser;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

/// This hook is used to manage user context.
#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<ResponseUser>>().unwrap();

    UseUserContextHandle { inner }
}