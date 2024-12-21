#[cfg(test)]
mod tests {
    use crate::config::asconfig_json::{AsconfigcJson, CompilerOption};
    use crate::config::get_tool_home;
    use crate::utils::convert_pathbuf_to_string::Stringify;

    #[test]
    fn mod_files() {
        use crate::new::template::create_mod_files;
        use crate::new::template::template_nvm_config;
        use crate::new::NewArgs;
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

        let meta_content = read_to_string(mod_path.join("mod.json")).unwrap();
        assert_eq!(
            meta_content,
            "{
  \"id\": \"fr.gabouchet.better-matchmaking\",
  \"version\": \"1.0.2\",
  \"name\": \"Better matchmaking\",
  \"description\": \"Best mod ever\"
}"
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

        let ui_entrypoint_content = read_to_string(
            mod_path.join("ui/src/fr/gabouchet/BetterMatchmaking.as"),
        )
        .unwrap();
        assert_eq!(
            ui_entrypoint_content,
            "package fr.gabouchet {
  import net.wg.infrastructure.base.AbstractView;

  class BetterMatchmaking extends AbstractView {

  }
}
"
        );

        let ui_config_content =
            read_to_string(mod_path.join("ui/asconfig.json")).unwrap();
        let wg_home = get_tool_home().unwrap();
        let flash_lib_home = wg_home.join("flash_lib");
        let lib_content = vec![
            "base_app-1.0-SNAPSHOT.swc",
            "battle.swc",
            "common-1.0-SNAPSHOT.swc",
            "common_i18n_library-1.0-SNAPSHOT.swc",
            "gui_base-1.0-SNAPSHOT.swc",
            "gui_battle-1.0-SNAPSHOT.swc",
            "gui_lobby-1.0-SNAPSHOT.swc",
            "lobby.swc",
        ];
        let lib_content_path = lib_content
            .iter()
            .map(|filename| flash_lib_home.join(filename).to_string().unwrap())
            .collect::<Vec<_>>();
        let json_ui_config = AsconfigcJson {
            config: "flex".to_string(),
            compiler_option: CompilerOption {
                output: "".to_string(),
                source_path: vec!["src".to_string()],
                library_path: lib_content_path,
            },
            main_class: "fr.gabouchet.BetterMatchmaking".to_string(),
        };
        let attended = serde_json::to_string_pretty(&json_ui_config).unwrap();
        assert_eq!(ui_config_content, attended);

        template_nvm_config(&mod_path).unwrap();
        let nvm_config_content =
            read_to_string(mod_path.join("settings.txt")).unwrap();
        assert_eq!(
            nvm_config_content,
            "root: ".to_owned()
                + mod_path.to_str().unwrap()
                + "\n
path: " + mod_path.to_str().unwrap()
                + "\\nodejs\n
arch: 64\n
proxy: none\n"
        );

        tmp_dir.close().unwrap();
    }
}
