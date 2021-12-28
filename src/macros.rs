#[macro_export]
macro_rules! aoc_tests {
    ( @get_day ) => {
            ::std::path::Path::new(file!())
                .file_stem()
                .unwrap()
                .to_os_string()
    };

    ( @get_input_file_name $name:ident ) => {
        ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("input_data")
            .join(
                [
                    super::aoc_tests!(@get_day),
                    "_".into(),
                    stringify!($name).into(),
                ]
                .into_iter()
                .collect::<::std::ffi::OsString>(),
            )
            .with_extension("txt")
    };

    ( @main ) => {
        pub fn main() -> Result<()> {
            use advent_of_code_helpers::command_line::Parser;
            use advent_of_code_helpers::{anyhow, bail};

            let opts = advent_of_code_helpers::command_line::Options::parse();

            let day = aoc_tests!(@get_day).into_string().unwrap();
            println!(
                "Running solver {}::task{} ...",
                day,
                opts.task
            );
            let data = ::std::fs::read_to_string(&opts.data).map_err(
                |e| anyhow!("Unable to open '{}': {}", opts.data.into_os_string().into_string().unwrap(), e)
            )?;
            let t0 = ::std::time::Instant::now();
            let input_data = parse_input(&data)?;
            let t1 = ::std::time::Instant::now();
            let solution = match opts.task {
                1 => format!("{}", task1(input_data)?),
                2 => format!("{}", task2(input_data)?),
                _ => bail!("Invalid task number!"),
            };
            let t2 = ::std::time::Instant::now();
            println!("   ... parse input: {} ms", (t1 - t0).as_millis());
            println!("   ... calculate: {} ms", (t2 - t1).as_millis());

            println!("─ Result: ──────────────────────────────────────");
            println!("{}", solution);
            println!("────────────────────────────────────────────────");

            Ok(())
        }
    };

    {} => {
        aoc_tests!(@main);
    };

    { $( $suite:ident : { $( $name:ident => $expected_result:expr, )* } ),* } => {
        aoc_tests!(@main);

        $(
        #[cfg(test)]
        mod $suite {
            $(
            #[test]
            fn $name() {
                let data = {
                    let input_file = super::aoc_tests!(@get_input_file_name $name);
                    ::std::fs::read_to_string(&input_file).unwrap_or_else(
                        |e| panic!("Unable to open '{}': {}", input_file.into_os_string().into_string().unwrap(), e)
                    )
                };

                let input_data = super::parse_input(&data).unwrap();
                let actual_result = super::$suite(input_data).unwrap();

                assert_eq!($expected_result, actual_result);
            }
            )*
        }
        )*
    };
}
