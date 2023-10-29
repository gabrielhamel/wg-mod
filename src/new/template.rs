use super::NewArgs;
use crate::utils::convert_to_absolute_path::convert_to_absolute_path;
use crate::utils::file_template::{write_template, TemplateError};
use convert_case::{Case, Casing};
use serde_json::json;
use std::path::PathBuf;

fn template_meta(
    args: &NewArgs, parent_dir: &PathBuf,
) -> Result<(), TemplateError> {
    write_template(
        &parent_dir,
        "meta.xml",
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

    Ok(())
}

fn template_script_entrypoint(
    args: &NewArgs, parent_dir: &PathBuf,
) -> Result<(), TemplateError> {
    write_template(
        &parent_dir,
        &format!("mod_{}.py", args.name.to_case(Case::Snake)),
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

fn template_git_ignore(parent_dir: &PathBuf) -> Result<(), TemplateError> {
    write_template(
        &parent_dir,
        ".gitignore",
        "/.idea
/.vscode
/target
.DS_Store
",
        &json!({}),
    )?;

    Ok(())
}

fn init_git_repository(directory: &PathBuf) -> Result<(), TemplateError> {
    template_git_ignore(directory)?;

    git2::Repository::init(directory)?;

    Ok(())
}

pub fn create_mod_files(args: NewArgs) -> Result<(), TemplateError> {
    let kebab_name =
        args.name.from_case(Case::Alternating).to_case(Case::Kebab);

    let root_path = args.directory.join(&kebab_name);
    template_meta(&args, &root_path)?;

    let scripts_entrypoint_path = &root_path.join("scripts");
    template_script_entrypoint(&args, &scripts_entrypoint_path)?;

    init_git_repository(&root_path)?;

    let absolute_mod_path = convert_to_absolute_path(&root_path)?;
    println!("Success! Created {kebab_name} at {absolute_mod_path}");

    Ok(())
}
