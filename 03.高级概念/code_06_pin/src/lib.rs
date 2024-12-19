mod inline_buf;

mod tests {
    #[allow(dead_code)]
    #[derive(Default)]
    struct AddrTracker(usize);

    impl AddrTracker {
        pub fn show_address(&self) {
            println!("Address: {:?}", self as *const _);
        }
    }

    /*
    fn take_addr_tracker(tracker: AddrTracker) {
        tracker.show_address();
    }

    #[test]
    fn test() {
        let tracker = AddrTracker::default();
        tracker.show_address();
        take_addr_tracker(tracker);
        // 地址不同
    }
    */

    fn take_addr_tracker(tracker: Box<AddrTracker>) {
        tracker.show_address();
    }

    #[test]
    fn test() {
        let tracker = Box::new(AddrTracker::default());
        tracker.show_address();
        take_addr_tracker(tracker);
        // 地址相同
    }

}

fn main() {}
