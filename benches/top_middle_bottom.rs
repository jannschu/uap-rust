#[macro_use]
extern crate criterion;
extern crate uap_rust;

use criterion::{Criterion, black_box};

use uap_rust::Client;


fn bench_device(c: &mut Criterion) {
    c.bench_function("device top", |b| {
    	b.iter(|| {
	    	black_box(Client::new("Mozilla/5.0 (Android 3.0; YRSpider; +http://www.yunrang.com/yrspider.html)").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; U; Android 1.6; ja-jp; SonyEricssonSO-01B Build/R1EA018) AppleWebKit/528.5 (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1 (compatible; ichiro/mobile goo; http://help.goo.ne.jp/help/article/1142)42)").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; Android 4.4.2; C6833 Build/14.3.A.0.681) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/30.0.0.0 Mobile Safari/537.36 SmartWatch(Sony;SW2;660x800)0x800)").device());
	    	black_box(Client::new("ERO247 Android Application (13, ero247 v2.1) - Sony Ericsson X10i SEMC - 00000000-310E-3C0B-90FE-4916687129771668712977").device());
	    	black_box(Client::new("VMVID Android Application (13, vmvid v2.1) - MEDION MEDION P4013 MEDION - 00000000-5B1F-AC16-EC17-E20D463B9A580D463B9A58").device());
	    	black_box(Client::new("VMVID Android Application (13, vmvid v2.1) - ZTE ATLAS W zte - 00000000-280C-CBB7-E544-8790554E377FFFA4B7C595").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; Android 4.0.4; RC9717B Build/IMM76D) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/34.0.1847.131 YaBrowser/14.5.1847.18432.00 Safari/537.36").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; U; Android 4.0.3; de-de; 3Q_ER71B Build/IML74K) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; Android 4.1.1; V360 Build/JRO03C) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19").device());
	    	black_box(Client::new("Mozilla/5.0 (Linux; U; Android 3.1; en-us; Acer Iconia Tab A500 Build/HMJ37) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13").device());
    	});
    });

    c.bench_function("device middle", |b| {
    	b.iter(|| {
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 4.0.4; af-za; PLT7035-B Build/IMM76D) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Safari/534.30").device());
    		black_box(Client::new("Mozilla 5.0 (Linux; U; Android 2.3.6; zh-cn; A900 Build MocorDroid2.3.5) UC AppleWebKit 534.31 (KHTML, like Gecko) Mobile Safari 534.31").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 2.3.5; en-US; QMobile_A2_Lite Build/MocorDroid2.3.5) AppleWebKit/528.5+ (KHTML, like Gecko) Version/3.1.2 Mobile Safari/525.20.1 UCBrowser/8.2.0.242 Mobile").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 4.0.4; En-us; QMobile A80 Build/IMM76D) AppleWebKit/534.30 (KHTML, Like Gecko) Version/4.0 Mobile Safari/534.30").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 4.0.4; vi-vn; Q-Smart S20 Build/IMM76I) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 4.0.3; en-us; Q-mobile S11 Revolution Build/GRK39F) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; U; Android 3.2; Pt-br; TA1013 Build/SEMP_111130_001) AppleWebKit/534.13 (KHTML, Like Gecko) Version/4.0 Safari/534.13").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; Android 5.0; RCT6773W22B Build/LRX21M) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.85").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; Android 4.4.2; RCA G1 Build/KOT49H) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/30.0.0.0 Mobile Safari/537.36").device());
    		black_box(Client::new("Mozilla/5.0 (Linux; Android 4.0.4; RK701 Build/RK2906) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19").device());
    	});
    });

    c.bench_function("device bottom", |b| {
    	b.iter(|| {
    		black_box(Client::new("Mozilla/5.0 (Android 5.0; Tablet; rv:41.0) Gecko/41.0 Firefox/41.0").device());
    		black_box(Client::new("Mozilla/5.0 (Android) ownCloud-android/2.0.0").device());
    		black_box(Client::new("Mozilla/4.0 (compatible; AvantGo 6.0; FreeBSD)").device());
    		black_box(Client::new("Mozilla/5.0 (compatible; Qwantify/2.4w; +https://www.qwant.com/)/2.4w").device());
    		black_box(Client::new("EricssonT68/R101 (;; ;; ;; Smartphone; SDA/1.0 Profile/MIDP-2.0 Configuration/CLDC-1.1)").device());
    		black_box(Client::new("MediaMonkey 4.1.9.1764").device());
    		black_box(Client::new("NOKIA /C13JBIB2 Profile/MIDP-2.0 Configuration/CLDC-1.1 UP.Browser/6.2.3.3.c.1.101 (GUI) MMP/2.0").device());
    		black_box(Client::new("Toshiba TS608_TS30/v1.0 UP.Browser/6.2.3.9.d.1 (GUI) MMP/2.0").device());
    		black_box(Client::new("Ice").device());
    		black_box(Client::new("Mozilla/5.0 (compatible; Daum/4.1; +http://cs.daum.net/faq/15/4118.html?faqId=28966)").device());
    	});
    });
}



fn bench_browser(c: &mut Criterion) {
    c.bench_function("browser top", |b| {
    	b.iter(|| {
			black_box(Client::new("ESPN Radio/3.2.113 CFNetwork/485.12.30 Darwin/10.4.0").browser());
			black_box(Client::new("Antenna/965 CFNetwork/758.2.8 Darwin/15.0.0").browser());
			black_box(Client::new("TopPodcastsPro/201 CFNetwork/758.2.8 Darwin/15.0.0").browser());
			black_box(Client::new("MusicDownloaderLite/1.0.1 CFNetwork/609.1.4 Darwin/13.0.0").browser());
			black_box(Client::new("IMPlusFull-iPad/7.9.1 CFNetwork/548.0.4 Darwin/11.0.0").browser());
			black_box(Client::new("AngryBirdsBlack-iPhone/1.1.0 CFNetwork/548.1.4 Darwin/11.0.0").browser());
			black_box(Client::new("MyApp/1.0 CFNetwork/887 Darwin/17.0.0").browser());
			black_box(Client::new("Mozilla/5.0 (compatible; heritrix/3.2.0 +http://espn.go.com").browser());
			black_box(Client::new("http://c.espnradio.com/s:").browser());
			black_box(Client::new("Mozilla/5.0 (Linux; Android 4.4.4; HP Slate 17 Build/KTU84P) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/33.0.0.0 Safari/537.36ESPN APP").browser());
		});
	});

    c.bench_function("browser middle", |b| {
    	b.iter(|| {
			black_box(Client::new("Evolution/3.26.2.1").browser());
			black_box(Client::new("RCM CardDAV plugin/0.9.2-dev").browser());
			black_box(Client::new("Mozilla/4.0 (Brew MP 1.0.4; U; en-us; Kyocera; NetFront/4.1/AMB) Sprint S2151-BST").browser());
			black_box(Client::new("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko; Google Web Preview) Chrome/27.0 .1453 Safari/537.36.").browser());
			black_box(Client::new("Mozilla/4.0 (compatible; MSIE 7.0; Windows Phone OS 7.0; Trident/3.1; IEMobile/7.0;  ; LEO70)").browser());
			black_box(Client::new("BacaBerita App/5.5.0 (Linux; U; Android 4.4.4; en-us) Mobile Safari").browser());
			black_box(Client::new("Player FM").browser());
			black_box(Client::new("VLC/2.2.0-rc1 LibVLC/2.2.0-rc1").browser());
			black_box(Client::new("DoggCatcher/1.2").browser());
			black_box(Client::new("Liferea/0.x.x (Linux; en_US.UTF-8; http://liferea.sf.net/)").browser());
		});
	});

    c.bench_function("browser bottom", |b| {
    	b.iter(|| {
			black_box(Client::new("Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_3; es-es) AppleWebKit/531.22.7 (KHTML, like Gecko)").browser());
			black_box(Client::new("SAMSUNG-SGH-A947/A947UCJF3 Mozilla/5.0 (BREW 5.0.3; BP251/162; U; en; rv:1.8.1) Gecko/20061208 Firefox/2.0.0 profile/MIDP-2.1 configuration/CLDC-1.1").browser());
			black_box(Client::new("Mozilla/5.0 (Android; Tablet: SAMSUNG-SGH-I467; rv:20.0) Gecko/20.0 Firefox/20.0").browser());
			black_box(Client::new("Mozilla/4.0 (compatible; MSIE 4.01; Windows CE; Sprint:SPH-ip830w; PPC; 240x320)").browser());
			black_box(Client::new("python-requests/0.14 CPython/2.6 Linux/2.6-43-server").browser());
			black_box(Client::new("W3C-checklink/4.1 [4.14] libwww-perl/5.803").browser());
			black_box(Client::new("Mozilla/5.0 (Java 1.4.2_02; Windows XP 5.1 x86; en) ICEbrowser/v6_0_0").browser());
			black_box(Client::new("Roku/DVP-5.0 (025.00E08043A)").browser());
			black_box(Client::new("Kurio/3.0.8 Build 65303(Android Kitkat 4.4.4; Phone)").browser());
			black_box(Client::new("Mozilla/5.0 (compatible; Daum/4.1; +http://cs.daum.net/faq/15/4118.html?faqId=28966)").browser());
		});
	});
}

fn bench_os(c: &mut Criterion) {
    c.bench_function("os top", |b| {
    	b.iter(|| {
			black_box(Client::new("Mozilla/5.0 (Unknown; Linux armv7l) AppleWebKit/537.1+ (KHTML, like Gecko) Safari/537.1+ HbbTV/1.1.1 ( ;LGE ;NetCast 4.0 ;03.20.30 ;1.0M ;)").os());
			black_box(Client::new("Mozilla/5.0 (DirectFB; Linux armv7l) AppleWebKit/534.26+ (KHTML, like Gecko) Version/5.0 Safari/534.26+ HbbTV/1.1.1 ( ;LGE ;NetCast 3.0 ;1.0 ;1.0M ;)").os());
			black_box(Client::new("HbbTV/1.1.1 (;;;;;) Maple_2011").os());
			black_box(Client::new("HbbTV/1.1.1 (;Samsung;SmartTV2013;T-FXPDEUC-1102.2;;) WebKit").os());
			black_box(Client::new("HbbTV/1.1.1 (;Samsung;SmartTV2013;T-MST12DEUC-1102.1;;) WebKit").os());
			black_box(Client::new("Opera/9.80 (Linux mips; U; HbbTV/1.1.1 (; Philips; ; ; ; ) CE-HTML/1.0 NETTV/4.1.3 PHILIPSTV/1.1.1; en) Presto/2.10.250 Version/11.60").os());
			black_box(Client::new("Opera/9.80 (Linux mips ; U; HbbTV/1.1.1 (; Philips; ; ; ; ) CE-HTML/1.0 NETTV/3.2.1; en) Presto/2.6.33 Version/10.70").os());
			black_box(Client::new("HbbTV/1.1.1 (;;;;;) firetv-firefox-plugin 1.1.20").os());
			black_box(Client::new("Opera/9.80 (Linux mips; U;  HbbTV/1.1.1 (; Sony; KDL40HX751; PKG1.902EUA; 2012;);; en) Presto/2.10.250 Version/11.60").os());
			black_box(Client::new("Mozilla/5.0 (compatible; MSIE 9.0; Windows Phone OS 7.5; Trident/5.0; IEMobile/9.0; ZTE; N880e_Dawoer_Fulllock; China Telecom)").os());
		});
	});

    c.bench_function("os middle", |b| {
    	b.iter(|| {
        	black_box(Client::new("TestApp/1.0 CFNetwork/758.0.2 Darwin/15.0.0").os());
        	black_box(Client::new("TestApp/1.0 CFNetwork/808.0.2 Darwin/16.0.0").os());
        	black_box(Client::new("MyApp/1.0 CFNetwork/893.13.1 Darwin/17.3.0 (x86_64)").os());
        	black_box(Client::new("Safari/12602.2.14.0.7 CFNetwork/807.1.3 Darwin/16.1.0 (x86_64)").os(), );
        	black_box(Client::new("Cooliris/1.3 CFNetwork/342.1 Darwin/9.4.1").os());
        	black_box(Client::new("MobileRSSFree-iPad/3.1 CFNetwork/467.12 Darwin/10.3.1").os());
        	black_box(Client::new("Yelp/8.2.1 CFNetwork/705.1 Darwin/14.0.0").os());
        	black_box(Client::new("MyApp/1.0 CFNetwork/811.4.18 Darwin/16.5.0").os());
        	black_box(Client::new("MyApp/1.0 CFNetwork/811.5.4 Darwin/16.6.0").os());
        	black_box(Client::new("MyApp/1.0 CFNetwork/811.5.4 Darwin/16.7.0").os());
    	});
    });

    c.bench_function("os bottom", |b| {
    	b.iter(|| {
			black_box(Client::new("Mozilla/5.0 (hp-tablet; Linux; hpwOS/3.0.5; U; en-US) AppleWebKit/534.6 (KHTML, like Gecko) wOSBrowser/234.83 Safari/534.6 TouchPad/1.0").os());
			black_box(Client::new("Opera/9.80 (VRE; Opera Mini/4.2/28.2794; U; en) Presto/2.8.119 Version/11.10").os());
			black_box(Client::new("Mozilla/5.0 (SAMSUNG; SAMSUNG-GT-S8600/S8600BOKK6; U; Bada/2.0; de-de) AppleWebKit/534.20 (KHTML, like Gecko) Dolfin/3.0 Mobile WVGA SMM-MMS/1.2.0 OPN-B").os());
			black_box(Client::new("Mozilla/5.0 (Linux 2.4.20-gentoo-r5 i686; U) Opera 7.11  [en]").os());
			black_box(Client::new("Opera/9.80 (Bada; Opera Mini/6.5/32.855; U; en) Presto/2.8.119 Version/11.10").os());
			black_box(Client::new("Mozilla/5.0 (Web0S; Linux/SmartTV) AppleWebKit/537.41 (KHTML, like Gecko) Large Screen WebAppManager Safari/537.41").os());
			black_box(Client::new("Opera/9.80 (X11; Linux x86_64; U; Slackware; lt) Presto/2.8.131 Version/11.11").os());
			black_box(Client::new("Mozilla/5.0 (Linux mipsel; U; HbbTV/1.1.1 (; TOSHIBA; RL858; 43.2.54.0; 1; ) ; ToshibaTP/1.1.0 () ; en) AppleWebKit/534.1 (KHTML, like Gecko)").os());
			black_box(Client::new("Mozilla/4.0 (compatible; MSIE 6.0; X11; SunOS sun4u) Opera 7.23  [en]").os());
			black_box(Client::new("Google Web Preview").os());
		});
	});
}

criterion_group!(benches, bench_device, bench_browser, bench_os);
criterion_main!(benches);