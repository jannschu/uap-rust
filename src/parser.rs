use {Browser, Device, OS};

rental! {
    mod owned {
        use std::ops::Deref;
        use stable_deref_trait::StableDeref;
        use {Browser as BrowserBorrowed, Device as DeviceBorrowed, OS as OSBorrowed};

        #[rental(covariant)]
        pub struct Browser<T>
        where
            T: Deref<Target=str> + StableDeref,
        {
            ua: T,
            browser: BrowserBorrowed<'ua>,
        }

        #[rental(covariant)]
        pub struct Device<T>
        where
            T: Deref<Target=str> + StableDeref,
        {
            ua: T,
            device: DeviceBorrowed<'ua>,
        }

        #[rental(covariant)]
        pub struct OS<T>
        where
            T: Deref<Target=str> + StableDeref,
        {
            ua: T,
            os: OSBorrowed<'ua>,
        }
    }
}

pub trait UserAgentInformation {
    fn user_agent(&self) -> &str;
    fn browser(&self) -> &Browser;
    fn device(&self) -> &Device;
    fn os(&self) -> &OS;
    fn is_bot(&self) -> bool;
}

macro_rules! create_parser {
    ($rc:path, $cell:path) => {
        use super::owned;
        use super::UserAgentInformation;
        use $cell;
        use $rc as RefCount;
        use {Browser, Device, OS};

        /// Parser for a user agent. The parser takes
        /// an browser user agent string by reference.
        ///
        /// It lazily parses information about the browser used, the device, and
        /// the operating system.
        ///
        /// To store the user agent with the parser use `OwningParser`. This
        /// also allows stroing a `Arc<str>` or `Rc<str>` respectively.
        pub struct BorrowingParser<'a> {
            user_agent: &'a str,
            browser: OnceCell<Browser<'a>>,
            device: OnceCell<Device<'a>>,
            os: OnceCell<OS<'a>>,
        }

        impl<'a> BorrowingParser<'a> {
            pub fn new(user_agent: &'a str) -> Self {
                Self {
                    user_agent,
                    browser: OnceCell::INIT,
                    device: OnceCell::INIT,
                    os: OnceCell::INIT,
                }
            }

            pub fn user_agent(&self) -> &str {
                self.user_agent
            }

            pub fn browser(&self) -> &Browser {
                self.browser.get_or_init(|| self.user_agent.into())
            }

            pub fn device(&self) -> &Device {
                self.device.get_or_init(|| self.user_agent.into())
            }

            pub fn os(&self) -> &OS {
                self.os.get_or_init(|| self.user_agent.into())
            }

            pub fn is_bot(&self) -> bool {
                self.device().family == "Spider"
            }
        }

        impl<'a> UserAgentInformation for BorrowingParser<'a> {
            fn user_agent(&self) -> &str {
                BorrowingParser::user_agent(self)
            }

            fn browser(&self) -> &Browser {
                BorrowingParser::browser(self)
            }

            fn device(&self) -> &Device {
                BorrowingParser::device(self)
            }

            fn os(&self) -> &OS {
                BorrowingParser::os(self)
            }

            fn is_bot(&self) -> bool {
                BorrowingParser::is_bot(self)
            }
        }

        /// Parser for a user agent. The parser takes
        /// an browser user agent string by value.
        ///
        /// It lazily parses information about the browser, the device, and
        /// the operating system.
        ///
        /// If you hold a reference to user agent string for the lifetime
        /// of the parser, `BorrowingParser` might be better suited.
        pub struct OwningParser {
            ua: RefCount<str>,
            browser: OnceCell<owned::Browser<RefCount<str>>>,
            device: OnceCell<owned::Device<RefCount<str>>>,
            os: OnceCell<owned::OS<RefCount<str>>>,
        }

        impl OwningParser {
            pub fn new<T: Into<RefCount<str>>>(user_agent: T) -> Self {
                OwningParser {
                    ua: user_agent.into(),
                    browser: OnceCell::INIT,
                    device: OnceCell::INIT,
                    os: OnceCell::INIT,
                }
            }

            pub fn user_agent(&self) -> &str {
                &*self.ua
            }

            pub fn browser(&self) -> &Browser {
                self.browser
                    .get_or_init(|| owned::Browser::new(self.ua.clone(), |ua| ua.into()))
                    .suffix()
            }

            pub fn device(&self) -> &Device {
                self.device
                    .get_or_init(|| owned::Device::new(self.ua.clone(), |ua| ua.into()))
                    .suffix()
            }

            pub fn os(&self) -> &OS {
                self.os
                    .get_or_init(|| owned::OS::new(self.ua.clone(), |ua| ua.into()))
                    .suffix()
            }

            pub fn is_bot(&self) -> bool {
                self.device().family == "Spider"
            }
        }

        impl UserAgentInformation for OwningParser {
            fn user_agent(&self) -> &str {
                OwningParser::user_agent(self)
            }

            fn browser(&self) -> &Browser {
                OwningParser::browser(self)
            }

            fn device(&self) -> &Device {
                OwningParser::device(self)
            }

            fn os(&self) -> &OS {
                OwningParser::os(self)
            }

            fn is_bot(&self) -> bool {
                OwningParser::is_bot(self)
            }
        }

        #[test]
        fn test_owning_client() {
            let parser = {
                let ua = "Firefox".to_string();
                OwningParser::new(ua)
            };
            let other_client = parser;
            assert!(!other_client.is_bot());
        }

        #[test]
        fn test_ref_count() {
            let ua: RefCount<str> = RefCount::from("Mozilla".to_string());
            {
                let parser = OwningParser::new(ua.clone());
                assert!(!parser.is_bot());
            }
        }

        #[test]
        fn test_user_agent() {
            assert_eq!(
                BorrowingParser::new("Samba 1234").user_agent(),
                "Samba 1234"
            );
            assert_eq!(OwningParser::new("Samba 1234").user_agent(), "Samba 1234");
        }
    };
}

pub mod sync {
    create_parser!{std::sync::Arc, once_cell::sync::OnceCell}

    #[test]
    fn test_sync() {
        use std::sync::Arc;
        use std::thread;
        // Send
        {
            let parser = BorrowingParser::new("Firefox");
            assert!(!thread::spawn(move || parser.is_bot()).join().unwrap());
        }
        {
            let parser = OwningParser::new("Firefox".to_string());
            assert!(!thread::spawn(move || parser.is_bot()).join().unwrap());
        }
        // Sync
        {
            let client_local = Arc::new(BorrowingParser::new("Firefox"));
            let parser = client_local.clone();
            assert!(!thread::spawn(move || parser.is_bot()).join().unwrap());
        }
        {
            let client_local = Arc::new(OwningParser::new("Firefox".to_string()));
            let parser = client_local.clone();
            assert!(!thread::spawn(move || parser.is_bot()).join().unwrap());
        }
    }
}

pub mod unsync {
    create_parser!{std::rc::Rc, once_cell::unsync::OnceCell}
}
