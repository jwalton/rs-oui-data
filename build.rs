use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("oui.rs");
    let out = &mut BufWriter::new(File::create(dest_path).unwrap());

    let mut used_ouis = HashSet::new();

    out.write_all(" /// Raw database of OUI records.\n".as_bytes()).unwrap();
    out.write_all(
        "pub static OUI_ENTRIES: phf::Map<&'static str, OuiData> = phf::phf_map! {\n".as_bytes(),
    )
    .unwrap();

    generate_oui_data("data/oui.csv", out, &mut used_ouis).unwrap();
    generate_oui_data("data/oui28.csv", out, &mut used_ouis).unwrap();
    generate_oui_data("data/oui36.csv", out, &mut used_ouis).unwrap();
    generate_oui_data("data/cid.csv", out, &mut used_ouis).unwrap();
    generate_oui_data("data/iab.csv", out, &mut used_ouis).unwrap();

    out.write_all("};\n".as_bytes()).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_oui_data(
    path: &str,
    out: &mut BufWriter<File>,
    used_ouis: &mut HashSet<String>,
) -> io::Result<()> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let registry = get_registry(record.get(0).unwrap());
        let oui = clean_string(record.get(1).unwrap()).to_ascii_uppercase();
        let organization = clean_string(record.get(2).unwrap());

        if used_ouis.contains(&oui) {
            println!("Discarding duplicate OUI: {oui}: {organization}");
            continue;
        }
        used_ouis.insert(oui.clone());

        let oui_data = format!(
            "\"{oui}\" => OuiData {{
                registry: {registry},
                oui: \"{oui}\",
                organization: \"{organization}\"
            }},\n",
        );
        out.write_all(oui_data.as_bytes())?;
    }

    println!("cargo:rerun-if-changed={path}");

    Ok(())
}

fn get_registry(registry: &str) -> &'static str {
    match registry {
        "MA-L" => "Registry::MAL",
        "MA-M" => "Registry::MAM",
        "MA-S" => "Registry::MAS",
        "CID" => "Registry::CID",
        "IAB" => "Registry::IAB",
        _ => panic!("Unknown registry: {registry}"),
    }
}

fn clean_string(s: &str) -> String {
    s.replace('\u{00A0}', " ")
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace("  ", "\\n")
        .replace('"', "\\\"")
        .trim()
        .to_string()
}
