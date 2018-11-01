use std::ops::Deref;

use lazy_init::LazyTransform;

use {Browser, Device, OS};

///`Client` struct, contains the parsed user agent information.
pub struct Client<'a> {
    browser: LazyParser<'a, Browser<'a>>,
    os: LazyParser<'a, OS<'a>>,
    device: LazyParser<'a, Device<'a>>,
}

impl<'a> Client<'a> {
    pub fn new(user_agent: &str) -> Client {
        Client {
            browser: LazyParser::new(user_agent),
            os: LazyParser::new(user_agent),
            device: LazyParser::new(user_agent),
        }
    }

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
