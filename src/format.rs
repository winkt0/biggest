struct Unit<'a> {
    size: u64,
    name: &'a str,
}

const GB: Unit = Unit {
    size: 1 << 30,
    name: "GB",
};

const MB: Unit = Unit {
    size: 1 << 20,
    name: "MB",
};

const KB: Unit = Unit {
    size: 1 << 10,
    name: "KB",
};

pub fn format_size(bytes: u64) -> String {
    match [GB, MB, KB].iter().find(|unit| bytes > unit.size) {
        Some(unit) => {
            let num_units = bytes / unit.size;
            return num_units.to_string()
                + " "
                + &unit.name
                + " "
                + &format_size(bytes - num_units * unit.size);
        }
        None => return bytes.to_string() + " bytes",
    };
}