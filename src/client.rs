use {Browser, Device, OS};

///`Client` struct, contains the parsed user agent information.
#[derive(Debug, PartialEq, Eq)]
pub struct Client<'a> {
    pub(crate) user_agent: &'a str,
    browser: Option<Browser>,
    os: Option<OS>,
    device: Option<Device>,
}

impl<'a> Client<'a> {
    pub fn new(user_agent: &str) -> Client {
        Client {
            user_agent: user_agent,
            browser: None,
            os: None,
            device: None,
        }
    }

    pub fn browser(&mut self) -> &Browser {
        if self.browser.is_none() {
            self.browser = self.user_agent.parse().ok();
        }
        &self.browser.as_ref().unwrap()
    }

    pub fn os(&mut self) -> &OS {
        if self.os.is_none() {
            self.os = self.user_agent.parse().ok();
        }
        &self.os.as_ref().unwrap()
    }

    pub fn device(&mut self) -> &Device {
        if self.device.is_none() {
            self.device = self.user_agent.parse().ok();
        }
        &self.device.as_ref().unwrap()
    }
}
