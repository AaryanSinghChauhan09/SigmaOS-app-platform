#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppManifest {
    pub app_id: String,
    pub display_name: String,
    pub exec_cmd: String,
    pub icon_name: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Default)]
pub struct AppPlatformRegistry {
    apps: Vec<AppManifest>,
}

impl AppPlatformRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_app(&mut self, manifest: AppManifest) {
        self.apps.push(manifest);
    }

    pub fn find_by_category(&self, cat: &str) -> Vec<&AppManifest> {
        self.apps.iter().filter(|a| a.categories.iter().any(|c| c == cat)).collect()
    }

    pub fn count(&self) -> usize {
        self.apps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_registration_and_category_lookup() {
        // Arrange
        let mut reg = AppPlatformRegistry::new();
        reg.register_app(AppManifest {
            app_id: "org.sigmaos.writer".into(),
            display_name: "GenOffice Writer".into(),
            exec_cmd: "/bin/genoffice-writer".into(),
            icon_name: "document-writer".into(),
            categories: vec!["Office".into(), "Productivity".into()],
        });

        // Act
        let office_apps = reg.find_by_category("Office");

        // Assert
        assert_eq!(office_apps.len(), 1);
        assert_eq!(office_apps[0].display_name, "GenOffice Writer");
    }
}
