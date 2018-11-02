use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use lazy_init::LazyTransform;
use owning_ref::{OwningHandle, StableAddress};

use {Browser, Device, OS};

struct UA<'a>(Cow<'a, str>);

impl<'a> Deref for UA<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<'a> StableAddress for UA<'a> {}

type OwningLazyParser<'a, T> = OwningHandle<Arc<UA<'a>>, LazyParser<'a, T>>;

///`Client` struct, contains the parsed user agent information.
pub struct Client<'a, T> {
    browser: OwningLazyParser<'a, Browser<'a>>,
    os: OwningLazyParser<'a, OS<'a>>,
    device: OwningLazyParser<'a, Device<'a>>,
    #[allow(dead_code)]
    marker: PhantomData<T>,
}

impl<'a, T> Client<'a, T> {
    pub fn browser(&self) -> &Browser {
        &*self.browser
    }

    pub fn os(&self) -> &OS {
        &*self.os
    }

    pub fn device(&self) -> &Device {
        &*self.device
    }

    pub fn is_bot(&self) -> bool {
        self.device().family == "Spider"
    }
}

impl<'a, T> Client<'a, T>
where
    T: Into<Cow<'a, str>>,
{
    pub fn new(user_agent: T) -> Client<'a, T> {
        let user_agent = Arc::new(UA(user_agent.into()));
        Client {
            browser: OwningLazyParser::new_with_fn(user_agent.clone(), |ua| {
                LazyParser::new(unsafe { &*ua })
            }),
            os: OwningLazyParser::new_with_fn(user_agent.clone(), |ua| {
                LazyParser::new(unsafe { &*ua })
            }),
            device: OwningLazyParser::new_with_fn(user_agent.clone(), |ua| {
                LazyParser::new(unsafe { &*ua })
            }),
            marker: PhantomData,
        }
    }
}

struct LazyParser<'a, T>
where
    T: From<&'a str> + Sync,
{
    lazy: LazyTransform<&'a str, T>,
}

impl<'a, T> LazyParser<'a, T>
where
    T: From<&'a str> + Sync,
{
    fn new(input: &'a str) -> Self {
        Self {
            lazy: LazyTransform::new(input),
        }
    }
}

impl<'a, T> Deref for LazyParser<'a, T>
where
    T: From<&'a str> + Sync,
{
    type Target = T;
    fn deref(&self) -> &T {
        self.lazy.get_or_create(T::from)
    }
}

#[test]
fn test_owning_client() {
    let client = {
        let ua = "Firefox".to_string();
        Client::new(ua)
    };
    let other_client = client;
    assert!(!other_client.is_bot());
}

#[test]
fn test_thread() {
    use std::thread;
    {
        let client = Client::new("Firefox");
        assert!(!thread::spawn(move || client.is_bot()).join().unwrap());
    }
    {
        let client = Client::new("Firefox".to_string());
        assert!(!thread::spawn(move || client.is_bot()).join().unwrap());
    }
    {
        let client_local = Arc::new(Client::new("Firefox"));
        let client = client_local.clone();
        assert!(!thread::spawn(move || client.is_bot()).join().unwrap());
    }
    {
        let client_local = Arc::new(Client::new("Firefox".to_string()));
        let client = client_local.clone();
        assert!(!thread::spawn(move || client.is_bot()).join().unwrap());
    }
}
