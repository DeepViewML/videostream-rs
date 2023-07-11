#[cfg(test)]
mod frame {
    use rand::{self, Rng};
    use videostream::frame;
    #[test]
    fn frame_test() {
        let fourcc = 0x33424752 as u32; //Hex for RGB3
        let frame = frame::Frame::new(640, 480, 0, fourcc).unwrap();

        assert_eq!(frame.width(), 640);
        assert_eq!(frame.height(), 480);
        assert_eq!(frame.fourcc(), fourcc);
        assert_eq!(frame.path(), None);
        assert_eq!(frame.handle(), None);

        frame.alloc().unwrap();

        assert_eq!(frame.size(), 640 * 480 * 3);

        let mem: &mut [u8] = frame.mmap_mut().unwrap();
        let mut rng = rand::thread_rng();
        for elem in &mut *mem {
            let num: u8 = rng.gen();
            *elem = num;
        }
        let mem2 = frame.mmap().unwrap();
        for i in 0..mem.len() {
            assert_eq!(mem[i], mem2[i]);
        }

        let frame2 = frame::Frame::new(640, 480, 0, fourcc).unwrap();
        frame2
            .attach(frame.handle().unwrap(), frame.size() as usize, 0)
            .unwrap();
        let v2: &mut [u8] = frame2.mmap_mut().unwrap();
        for i in 0..mem.len() {
            assert_eq!(mem[i], v2[i]);
        }

        for elem in &mut *v2 {
            let num: u8 = rng.gen();
            *elem = num;
        }
        assert_eq!(mem[0], v2[0]);
        assert_eq!(mem2[0], v2[0]);
    }
}
