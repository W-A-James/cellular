#[cfg(test)]
mod image_manip_tests {
    use ca_110::image_manip::*;
    use std::fs::File;

    fn init() {
        env_logger::init();
    }

    #[test]
    fn test_init_image() {
        let mut bmp_0 = BitMap::new(10);
        let mut bmp_1 = BitMap::new(10);

        for _ in 0..10 {
            bmp_1 = bitmap::rule110_step(&mut bmp_1);
        }
        init_image(10, 11, &mut bmp_0).unwrap();

        let vec_0 = bmp_0.get_vec();
        let vec_1 = bmp_1.get_vec();

        assert!(vec_0.len() == vec_1.len());
        for i in 0..vec_0.len() {
            assert!(vec_0[i] == vec_1[i]);
        }
    }

    #[test]
    fn test_gen_next_image() {
        let mut bmp_0 = BitMap::new(10);
        let mut bmp_1 = BitMap::new(10);
        for _ in 0..11 {
            bmp_1 = bitmap::rule110_step(&mut bmp_1);
        }
        let mut img = init_image(10, 11, &mut bmp_0).unwrap();
        gen_next_image(&mut img, 10, 11, &mut bmp_0).unwrap();

        let vec_0 = bmp_0.get_vec();
        let vec_1 = bmp_1.get_vec();

        assert!(vec_0.len() == vec_1.len());
        for i in 0..vec_0.len() {
            assert!(vec_0[i] == vec_1[i]);
        }
    }

    use log::{error, info};
    use std::time::Instant;

    fn set_half(bmp: &mut BitMap) {
        for i in 0..bmp.size() / 2 {
            bmp.set(i);
        }
    }
    #[test]
    fn test_build_gif() {
        init();
        // Ensure that all images end in 0x3b
        let sizes = vec![
            (10, 10),
            (20, 20),
            (40, 40),
            (80, 80),
            (160, 160),
            (320, 320),
            (640, 640),
            (1280, 1280),
            (2560, 2560),
        ];

        for size in sizes {
            let w = size.0 as u16;
            let h = size.1 as u16;
            let steps = 1;
            let file_name = format!("test_{}x{}.gif", w, h);
            let mut bmp = BitMap::new(w.into());
            set_half(&mut bmp);
            println!("{:#?}", bmp.to_bit_vec());

            // Build gif and write to file
            build_gif(w, h, steps, &mut bmp, file_name.as_str(), None).unwrap();

            // Open newly written file
            info!("{}", format!("Attempting to read {}", file_name));
            let file = match File::open(file_name.as_str()) {
                Ok(f) => {
                    info!("Successfully opened {}", file_name);
                    f
                }
                Err(e) => {
                    info!("Error: {:?}", e);
                    panic!(e);
                }
            };
            let mut gif_opts = gif::DecodeOptions::new();
            gif_opts.set_color_output(gif::ColorOutput::Indexed);
            let mut decoder = match gif_opts.read_info(file) {
                Ok(d) => {
                    info!("Successfully created decoder!");
                    d
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    panic!(e);
                }
            };
            //let screen = gif_dispose::Screen::new_decoder(&decoder);

            let mut i = 0;
            let mut exit_loop = false;
            while !exit_loop {
                // Loop until EOF
                assert!(decoder.width() == w);
                assert!(decoder.height() == h);
                let palette = decoder.global_palette().unwrap();
                // Make sure that global palette is valid
                // [0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00]
                for j in 0..3 {
                    assert!(0xFF == palette[j]);
                    assert!(0x00 == palette[5 - j]);
                }
                let frame = decoder.read_next_frame();
                match frame {
                    Ok(img) => {
                        info!("{}", format!("Successfully read frame {}\n", i));
                        match img {
                            Some(image) => {
                                assert!(image.width == w);
                                assert!(image.height == h);
                                let buff = &image.buffer;
                                if i == 0 {
                                    for ii in 0..(w / 2) {
                                        println! {"{:?}", buff};

                                        assert!(
                                            buff[ii as usize] == 1,
                                            "Failed on file, ii: {}, {}",
                                            file_name,
                                            ii
                                        );
                                    }
                                }
                            }
                            None => {
                                exit_loop = true;
                            }
                        };
                    }
                    Err(e) => {
                        error!("{}", format!("Failed to read frame {}: {:?}", i, e));
                        panic!("{:?}", e);
                    }
                }
                i += 1;
            }
            info!("{}", format!("Successfully read {}\n", file_name));
        }
    }
}

#[cfg(test)]
mod bitmap_tests {
    use ca_110::image_manip::bitmap::*;
    #[test]
    fn test_bitmap_constructor() {
        let mut bmp = BitMap::new(55);
        assert!(bmp.size() == 55);
        let vec = bmp.get_vec();
        assert!(vec.len() == 1);
        bmp = BitMap::new(1500);
        assert!(bmp.size() == 1500);
        let vec = bmp.get_vec();
        assert!(vec.len() == 24);

        bmp = BitMap::new(64);
        assert!(bmp.size() == 64);
        let vec = bmp.get_vec();
        assert!(vec.len() == 1);
    }

    #[test]
    #[should_panic]
    fn test_bitmap_constructor_bad_args() {
        BitMap::new(0);
    }

    #[test]
    #[should_panic]
    fn test_random_constructor_bad_args() {
        BitMap::random(0);
    }

    #[test]
    #[should_panic]
    fn test_get_with_out_of_bounds_val() {
        let bmp = BitMap::random(10);
        bmp.get(10);
    }

    #[test]
    #[should_panic]
    fn test_set_with_out_of_bounds_val() {
        let mut bmp = BitMap::random(10);
        bmp.set(10);
    }

    #[test]
    #[should_panic]
    fn test_unset_with_out_of_bounds_val() {
        let mut bmp = BitMap::random(10);
        bmp.unset(10);
    }

    #[test]
    fn test_set() {
        let mut bmp = BitMap::new(64);
        let mut cur_val: u64 = 0;
        for i in 0..64 {
            bmp.set(i);
            let vec = bmp.get_vec();
            cur_val += 2u64.pow(i as u32);
            assert!(vec[0] == cur_val);
        }
    }

    #[test]
    fn test_unset() {
        let mut bmp = BitMap::new(64);
        let mut cur_val: u64 = u64::MAX;
        for i in 0..64 {
            bmp.set(i);
        }

        for i in 0..64 {
            bmp.unset(i);
            cur_val -= 2u64.pow(i as u32);
            let vec = bmp.get_vec();
            assert!(vec[0] == cur_val);
        }
    }

    #[test]
    fn test_rule110_step_normal_cases() {
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);
    }

    #[test]
    fn test_rule110_step_wrap_around_cases() {
        // Testing bit 2
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(2) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(1) == 0);

        // Testing bit 0
        let mut bmp = BitMap::new(3);
        // 000
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp = rule110_step(&mut bmp);
        assert!(bmp.get(0) == 0);
    }

    use std::time::Duration;
    use std::time::Instant;
    #[test]
    #[ignore]
    fn rule_110_profiling() {
        let sizes = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];
        let len = sizes.len();
        let mut rule110_times: Vec<Duration> = Vec::with_capacity(sizes.len());
        let num_iterations = 1000;

        for size in &sizes {
            let mut bmp = BitMap::random(*size);
            let start = Instant::now();
            for _ in 0..num_iterations {
                bmp = rule110_step(&mut bmp);
            }
            let end = Instant::now();
            rule110_times.push(end.duration_since(start));
        }

        for i in 0..len {
            println!(
                "N: {}, rule110_step: {}s",
                sizes[i],
                rule110_times[i].as_secs_f64() / (num_iterations as f64)
            );
        }
    }

    #[test]
    #[ignore]
    fn constructor_profiling() {
        let sizes: Vec<u64> = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];
        let len = sizes.len();
        let mut constructor_times: Vec<Duration> = Vec::with_capacity(sizes.len());
        let num_iterations = 1000;

        for size in &sizes {
            let start = Instant::now();
            for _ in 0..num_iterations {
                let _bmp = BitMap::random(*size);
            }
            let end = Instant::now();

            constructor_times.push(end.duration_since(start));
        }

        for i in 0..len {
            println!(
                "N: {}, constructor: {:?}",
                sizes[i],
                constructor_times[i].as_secs_f64() / (num_iterations as f64)
            );
        }
    }
}
