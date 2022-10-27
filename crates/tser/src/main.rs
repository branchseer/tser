use tser::{generate_from_ts, Language};

fn main() -> anyhow::Result<()> {
    let rust_src = generate_from_ts(
        r"
    enum MyStringEnum {
        Foo = 'FOO', Bar = 'BAR'
    }
    enum MyIntEnum {
        a = 2, b = 4, c
    }
    interface MyStruct {
        foo: (string | null)[]
        bar: number
    }
    ",
        Language::Swift,
    )?;
    print!("{rust_src}");
    Ok(())
}
