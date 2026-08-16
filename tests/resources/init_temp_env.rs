use mocked_up::file_system::TempEnv;

pub fn init_temp_env() -> TempEnv {
    let mut temp = TempEnv::new().unwrap();
    temp.env()
        .mkdir_and("d1", |d| {
            d.mkdir_and("d1d1", |d| {
                d.touch_and("d1d1f1", |f| {
                    f.write("Test content for d1d1f1.").unwrap();
                })
                .unwrap();
            })
            .unwrap();
        })
        .unwrap()
        .mkdir_and("d2", |d| {
            d.touch_and("d2f1", |f| {
                f.write("Test content for d2f1.").unwrap();
            })
            .unwrap();
        })
        .unwrap()
        .mkdir_and("d3", |d| {
            d.touch_and("d3f1", |f| {
                f.write("Test content for d3f1.").unwrap();
            })
            .unwrap();
        })
        .unwrap()
        .touch_and("f1", |f| {
            f.write("Test content for f1.").unwrap();
        })
        .unwrap()
        .touch_and("f2", |f| {
            f.write("Test content for f2.").unwrap();
        })
        .unwrap()
        .touch_and("f3", |f| {
            f.write("Test content for f3.").unwrap();
        })
        .unwrap();
    temp
}
