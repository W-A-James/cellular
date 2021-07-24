#[cfg(test)]
mod image_manip_tests {
    use cellular::image_manip::bitmap::BitMap;
    use cellular::image_manip::*;
    use std::fs::File;

    #[test]
    fn test_init_image() {
        let mut bmp_0 = BitMap::new(10);
        let mut bmp_1 = BitMap::new(10);
        let rule = 110;

        for _ in 0..10 {
            bmp_1.rule_step(rule);
        }
        init_image(10, 11, &mut bmp_0, rule).unwrap();

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
        let rule = 110;
        for _ in 0..11 {
            bmp_1.rule_step(rule);
        }
        let mut img = init_image(10, 11, &mut bmp_0, rule).unwrap();
        gen_next_image(&mut img, 10, 11, &mut bmp_0, rule).unwrap();

        let vec_0 = bmp_0.get_vec();
        let vec_1 = bmp_1.get_vec();

        assert!(vec_0.len() == vec_1.len());
        for i in 0..vec_0.len() {
            assert!(vec_0[i] == vec_1[i]);
        }
    }

    use log::{error, trace};
    use std::fs;

    fn set_half(bmp: &mut BitMap) {
        for i in 0..bmp.size() / 2 {
            bmp.set(i);
        }
    }

    #[test]
    fn test_build_gif() {
        //init_logger();
        let rule = 110;
        // Ensure that all images end in 0x3b
        let sizes = vec![
            (10, 10),
            (20, 20),
            (40, 40),
            (80, 80),
            (160, 160),
            (320, 320),
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
            build_gif(w, h, steps, &mut bmp, file_name.as_str(), None, rule).unwrap();

            // Open newly written file
            trace!("{}", format!("Attempting to read {}", file_name));
            let file = match File::open(file_name.as_str()) {
                Ok(f) => {
                    trace!("Successfully opened {}", file_name);
                    f
                }
                Err(e) => {
                    trace!("Error: {:?}", e);
                    panic!("{:?}", e);
                }
            };
            let mut gif_opts = gif::DecodeOptions::new();
            gif_opts.set_color_output(gif::ColorOutput::Indexed);
            let mut decoder = match gif_opts.read_info(file) {
                Ok(d) => {
                    trace!("Successfully created decoder!");
                    d
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    panic!("{:?}", e);
                }
            };

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
                        trace!("{}", format!("Successfully read frame {}\n", i));
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
            trace!("{}", format!("Successfully read {}\n", file_name));
            fs::remove_file(file_name).unwrap();
        }
    }
}

#[cfg(test)]
mod bitmap_tests {
    use cellular::image_manip::bitmap::*;
    fn get_integer_value(bv: &BitMap) -> u64 {
        let mut rv: u64 = 0;
        for i in 0..bv.size() {
            let val: u64 = bv.get(i) as u64;
            rv += (val << i) as u64;
        }

        rv
    }
    #[test]
    fn test_bitmap_constructor() {
        let bmp = BitMap::new(55);
        assert!(bmp.size() == 55);
        let vec = bmp.get_vec();
        assert!(vec.len() == 55);
    }

    #[test]
    #[should_panic]
    fn test_bitmap_constructor_bad_args() {
        BitMap::new(0);
    }

    #[test]
    #[should_panic]
    fn test_random_constructor_bad_args() {
        BitMap::random(0, -1.0);
    }

    #[test]
    #[should_panic]
    fn test_get_with_out_of_bounds_val() {
        let bmp = BitMap::random(10, 0.5);
        bmp.get(10);
    }

    #[test]
    #[should_panic]
    fn test_set_with_out_of_bounds_val() {
        let mut bmp = BitMap::random(10, 0.5);
        bmp.set(10);
    }

    #[test]
    #[should_panic]
    fn test_unset_with_out_of_bounds_val() {
        let mut bmp = BitMap::random(10, 0.5);
        bmp.unset(10);
    }

    #[test]
    fn test_set() {
        let mut bmp = BitMap::new(64);
        let mut cur_val: u64 = 0;
        for i in 0..64 {
            bmp.set(i);
            cur_val += 2u64.pow(i as u32);
            assert!(get_integer_value(&bmp) == cur_val);
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
            assert!(get_integer_value(&bmp) == cur_val);
        }
    }

    #[test]
    fn test_rule_step_normal_cases() {
        let rule = 110;
        let mut bmp = BitMap::new(3);
        // 000
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 0);
    }

    #[test]
    fn test_rule_step_wrap_around_cases() {
        let rule = 110;
        // Testing bit 2
        let mut bmp = BitMap::new(3);
        // 000
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(2) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(1) == 0);

        // Testing bit 0
        let mut bmp = BitMap::new(3);
        // 000
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 0);
        // 001
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 1);
        // 010
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 1);
        // 011
        let mut bmp = BitMap::new(3);
        bmp.set(0);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 1);
        // 100
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 0);
        // 101
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(2);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 1);
        // 110
        let mut bmp = BitMap::new(3);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 1);
        // 111
        let mut bmp = BitMap::new(3);
        bmp.set(2);
        bmp.set(1);
        bmp.set(0);
        bmp.rule_step(rule);
        assert!(bmp.get(0) == 0);
    }
}

#[cfg(test)]
mod image_manip_bench {
    use cellular::image_manip::bitmap::BitMap;
    use cellular::image_manip::*;
    use std::fs;
    use std::io::Write;
    use std::time::Instant;

    #[test]
    fn build_gif_profiling() {
        let rule = 110;

        let widths = vec![200, 400, 800, 1600];
        let heights = vec![200, 400, 800, 1600];
        let frames = vec![200, 400, 800, 1600];
        let file_name = "build_gif_profiling_data.csv";
        let mut file_handle = fs::File::create(file_name).unwrap();
        file_handle
            .write("Height,Width,Frames,Time(s)\n".as_bytes())
            .unwrap();
        for w in &widths {
            for h in &heights {
                for f in &frames {
                    let fname = format!("test_{}_{}_{}_{}.gif", w, h, f, rule);
                    let mut line = BitMap::random((*w).into(), 0.5);

                    let start = Instant::now();
                    build_gif(*w, *h, *f, &mut line, &fname, None, rule).unwrap();
                    let end = Instant::now();
                    std::fs::remove_file(fname).unwrap();

                    file_handle
                        .write(
                            format!(
                                "{:4},{:4},{:4},{:4.2}",
                                h,
                                w,
                                f,
                                end.duration_since(start).as_secs_f64()
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod bitmap_bench {
    use cellular::image_manip::bitmap::*;
    use std::fs;
    use std::io::Write;
    use std::time::Instant;
    const RULE: u8 = 110;

    #[test]
    fn rule_step_profiling() {
        let file_name = "rule_step_profiling_data.csv";
        let mut file_handle = fs::File::create(file_name).unwrap();
        let sizes = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];
        let num_iterations = 100000;

        file_handle
            .write("Size,Iters,Time(s)\n".as_bytes())
            .unwrap();
        for size in &sizes {
            let mut bmp = BitMap::random(*size, 0.5);
            let start = Instant::now();
            for _ in 0..num_iterations {
                bmp.rule_step(RULE);
            }
            let end = Instant::now();
            file_handle
                .write(
                    format!(
                        "{},{},{:.3}\n",
                        size,
                        num_iterations,
                        end.duration_since(start).as_secs_f64()
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}
