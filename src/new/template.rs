use super::{Error, NewArgs};
use crate::utils::file_template::write_template;
use convert_case::{Case, Casing};
use serde_json::json;
use std::fs::create_dir;

pub fn create_mod_files(args: NewArgs) -> Result<(), Error> {
    create_dir(&args.directory).map_err(Error::DirectoryCreateError)?;

    write_template(
        args.directory.join("meta.xml"),
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

    create_dir(&args.directory.join("scripts")).map_err(Error::DirectoryCreateError)?;

    let mod_entrypoint_path = args
        .directory
        .join(format!("scripts/mod_{}.py", args.name.to_case(Case::Snake)));

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
