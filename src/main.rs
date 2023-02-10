mod screenshot;

fn main() -> Result<(), xcb::Error>{
    screenshot::screenshot()?;
    Ok(())
}