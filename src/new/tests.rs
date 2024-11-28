#[cfg(test)]
mod tests {
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
  \"mainClass\": \"BetterMatchmaking\"
}
"
        );

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
