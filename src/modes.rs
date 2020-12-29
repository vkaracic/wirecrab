use std::collections::HashMap;

pub enum Modes {
    IndividualPrint,
    IndividualPrintWithData,
    DefaultWithData,
    Default
}

pub fn get_modes_map() -> HashMap<u8, &'static Modes> {
    let mut modes = HashMap::new();
    modes.insert(0, &Modes::IndividualPrint);
    modes.insert(1, &Modes::Default);
    modes.insert(2, &Modes::IndividualPrintWithData);
    modes.insert(3, &Modes::DefaultWithData);
    return modes;
}
