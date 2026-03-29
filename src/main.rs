fn main() {
    #[cfg(feature = "server")]
    dotenvy::dotenv().ok();

    // En dev, on s'assure que les fichiers de public/ du projet sont bien
    // accessibles depuis le répertoire public/ à côté du binaire dx
    #[cfg(all(feature = "server", debug_assertions))]
    {
        let exe_public = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("public");
        let project_public = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");

        if exe_public.exists() && project_public.exists() {
            if let Ok(entries) = std::fs::read_dir(&project_public) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let dest = exe_public.join(&name);
                    if !dest.exists() {
                        let src = entry.path();
                        #[cfg(unix)]
                        let _ = std::os::unix::fs::symlink(&src, &dest);
                    }
                }
            }
        }
    }

    dioxus::launch(traiteur_website::App); // v2
}
