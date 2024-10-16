#[test]
fn mod_files() {
    use super::{template::create_mod_files, NewArgs};
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
        read_to_string(mod_path.join("scripts/mod_better_matchmaking.py"))
            .unwrap();
    assert_eq!(
        script_entrypoint_content,
        "def init():
    print(\"Hello world from Better matchmaking\")

def fini():
    print(\"Good bye world from Better matchmaking\")
"
    );

    let ui_entrypoint_content =
        read_to_string(mod_path.join("ui/better_matchmaking.as")).unwrap();
    assert_eq!(
        ui_entrypoint_content,
        "package{
        class BetterMatchmaking {}
        }"
    );

    let ui_config_content =
        read_to_string(mod_path.join("ui/asconfig.json")).unwrap();
    assert_eq!(
        ui_config_content,
        "{
  \"config\": \"flex\",
  \"type\": \"lib\",
  \"compilerOptions\": {
    \"output\": \".\",
    \"targets\": [
      \"SWF\"
    ],
    \"source-map\": true
  },
  \"mainClass\": \"BetterMatchmaking\"\
  }"
    );

    tmp_dir.close().unwrap();
}
