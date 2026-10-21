pub struct PowerCollector {

}

impl PowerCollector {
    pub fn new() -> Self {
        // consultar informacion de la carpeta
        let powerInfo = 
    }

    fn battery_capacity() -> std::io::Result<u8> {
    let content = fs::read_to_string(
        "/sys/class/power_supply/BAT0/capacity"
    )?;

    let capacity = content.trim().parse::<u8>()
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e,
        ))?;

    Ok(capacity)
}
}