#[cfg(test)]
mod frame {
    use rand::{self, Rng};
    use std::{
        fs::{self, File},
        io::Write,
        os::fd::AsRawFd,
    };
    use videostream::frame;
    #[test]
    fn frame() {
        //let fourcc = 0x33424752 as u32; //Hex for RGB3
        let frame = frame::Frame::new(640, 480, 0, "RGB3").unwrap();

        assert_eq!(frame.width(), 640);
        assert_eq!(frame.height(), 480);
        assert_eq!(frame.fourcc(), 0x33424752);
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

        let frame2 = frame::Frame::new(640, 480, 0, "RGB3").unwrap();
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

    #[test]
    fn attach_file() {
        let frame = frame::Frame::new(640, 480, 0, "RGB3").unwrap();

        let mut expect = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..(frame.height() * frame.width() * 3) {
            expect.push(rng.gen::<u8>() as u8);
        }
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open("./temp.txt")
            .unwrap();
        file.write_all(&expect).unwrap();

        frame
            .attach(
                file.as_raw_fd(),
                (frame.height() * frame.width() * 3) as usize,
                0,
            )
            .unwrap();

        let mem = frame.mmap().unwrap();

        for i in 0..mem.len() {
            assert_eq!(mem[i], expect[i])
        }
        fs::remove_file("./temp.txt");
    }
}
