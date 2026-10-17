use anyhow::{bail, Result};
use zkperf::da51::Da51Address;

fn main() -> Result<()> {
    let input = std::env::args().nth(1).unwrap_or_default();
    if input.is_empty() {
        bail!("usage: zkperf-da51 <16-hex-digit-address>");
    }

    let address: Da51Address = input.parse()?;
    println!("address={}", address);
    println!("prefix=0x{:04X}", address.prefix());
    println!("type={}", address.type_code());
    println!("data=0x{:011X}", address.data());
    println!(
        "known_type={}",
        address
            .known_type()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| "unassigned".to_string())
    );

    if let Some(coord) = address.atlas_coord()? {
        println!("atlas_level={:?}", coord.level);
        println!("atlas_biome={:?}", coord.biome);
        println!("atlas_status={:?}", coord.status);
        println!("atlas_serial={}", coord.serial);
    }

    Ok(())
}
