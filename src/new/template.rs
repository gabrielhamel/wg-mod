use super::{Error, NewArgs};
use crate::utils::file_template::write_template;
use convert_case::{Case, Casing};
use serde_json::json;
use std::fs::create_dir;

pub fn create_mod_files(args: NewArgs) -> Result<(), Error> {
    let kebab_name = args.name.from_case(Case::Alternating).to_case(Case::Kebab);

    let mod_path = args.directory.join(kebab_name);
    create_dir(&mod_path).map_err(Error::DirectoryCreateError)?;

    write_template(
        mod_path.join("meta.xml"),
        "<root>
    <id>{{package_name}}</id>
    <version>{{version}}</version>
    <name>{{name}}</name>
    <description>{{description}}</description>
</root>",
        &json!({
            "package_name": args.package_name,
            "version": args.version,
            "name": args.name,
            "description": args.description
        }),
    )?;

    let scripts_path = &mod_path.join("scripts");
    create_dir(scripts_path).map_err(Error::DirectoryCreateError)?;

    let mod_entrypoint_path = scripts_path.join(format!("mod_{}.py", args.name.to_case(Case::Snake)));

    write_template(
        mod_entrypoint_path,
        "def init():
    print(\"Hello world from {{name}}\")

def fini():
    print(\"Good bye world from {{name}}\")
",
        &json!({
            "name": args.name
        }),
    )?;

    Ok(())
}

#[test]
fn mod_files() {
    use std::fs::read_to_string;
    use tempfile::tempdir;

    let tmp_dir = tempdir().unwrap();

    let args = NewArgs {
        version: "1.0.2".to_owned(),
        description: "Best mod ever".to_owned(),
        name: "Better matchmaking".to_owned(),
        directory: tmp_dir.path().to_owned(),
        package_name: "fr.gabouchet.better-matchmaking".to_owned(),
    };

    create_mod_files(args).unwrap();

    let mod_path = tmp_dir.path().join("better-matchmaking");

    assert_eq!(mod_path.exists(), true);

    let meta_content = read_to_string(mod_path.join("meta.xml")).unwrap();
    assert_eq!(
        meta_content,
        "<root>
    <id>fr.gabouchet.better-matchmaking</id>
    <version>1.0.2</version>
    <name>Better matchmaking</name>
    <description>Best mod ever</description>
</root>"
    );

    let script_entrypoint_content =
        read_to_string(mod_path.join("scripts/mod_better_matchmaking.py")).unwrap();
    assert_eq!(
        script_entrypoint_content,
        "def init():
    print(\"Hello world from Better matchmaking\")

def fini():
    print(\"Good bye world from Better matchmaking\")
"
    );

    tmp_dir.close().unwrap();
}
