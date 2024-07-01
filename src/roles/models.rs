#[derive(PartialEq, Debug, Clone)]
pub enum Role {
    FilesAdmin,
    FilesEditor,
    FilesViewer,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Permission {
    DirsCreate,
    DirsEdit,
    DirsDelete,
    DirsList,
    DirsView,
    DirsManage,

    FilesCreate,
    FilesEdit,
    FilesDelete,
    FilesList,
    FilesView,
    FilesManage,
}

impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FilesAdmin" => Ok(Role::FilesAdmin),
            "FilesEditor" => Ok(Role::FilesEditor),
            "FilesViewer" => Ok(Role::FilesViewer),
            _ => Err(format!("Invalid role: {}", value)),
        }
    }
}

pub fn to_roles(list: Vec<String>) -> crate::Result<Vec<Role>> {
    let mut roles: Vec<Role> = Vec::new();
    for item in list.into_iter() {
        match Role::try_from(item.as_str()) {
            Ok(role) => roles.push(role),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(roles)
}

impl TryFrom<&str> for Permission {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dirs.create" => Ok(Permission::DirsCreate),
            "dirs.edit" => Ok(Permission::DirsEdit),
            "dirs.delete" => Ok(Permission::DirsDelete),
            "dirs.list" => Ok(Permission::DirsList),
            "dirs.view" => Ok(Permission::DirsView),
            "dirs.manage" => Ok(Permission::DirsManage),
            "files.create" => Ok(Permission::FilesCreate),
            "files.edit" => Ok(Permission::FilesEdit),
            "files.delete" => Ok(Permission::FilesDelete),
            "files.list" => Ok(Permission::FilesList),
            "files.view" => Ok(Permission::FilesView),
            "files.manage" => Ok(Permission::FilesManage),
            _ => Err(format!("Invalid permission: {}", value)),
        }
    }
}

/// Role to permissions mapping
pub fn role_permissions(role: &Role) -> Vec<Permission> {
    match role {
        Role::FilesAdmin => vec![
            Permission::DirsCreate,
            Permission::DirsEdit,
            Permission::DirsDelete,
            Permission::DirsList,
            Permission::DirsView,
            Permission::DirsManage,
            Permission::FilesCreate,
            Permission::FilesEdit,
            Permission::FilesDelete,
            Permission::FilesList,
            Permission::FilesView,
            Permission::FilesManage,
        ],
        Role::FilesEditor => vec![
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesCreate,
            Permission::FilesList,
            Permission::FilesView,
        ],
        Role::FilesViewer => vec![
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesList,
            Permission::FilesView,
        ],
    }
}

/// Checks whether the given roles have the required permissions
pub fn has_permissions(roles: &Vec<Role>, req_permissions: &Vec<Permission>) -> bool {
    // Collect all permissions for all roles
    let mut all_permissions: Vec<Permission> = Vec::new();
    for role in roles {
        all_permissions.extend(role_permissions(role));
    }

    req_permissions
        .iter()
        .all(|permission| all_permissions.contains(permission))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_permissions() {
        let roles = vec![Role::FilesAdmin];
        let permissions = vec![
            Permission::DirsCreate,
            Permission::DirsEdit,
            Permission::DirsDelete,
            Permission::DirsList,
            Permission::DirsView,
            Permission::DirsManage,
            Permission::FilesCreate,
            Permission::FilesEdit,
            Permission::FilesDelete,
            Permission::FilesList,
            Permission::FilesView,
            Permission::FilesManage,
        ];
        assert!(has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_editor_permissions() {
        let roles = vec![Role::FilesEditor];
        let permissions = vec![
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesCreate,
            Permission::FilesList,
            Permission::FilesView,
        ];
        assert!(has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_viewer_permissions() {
        let roles = vec![Role::FilesViewer];
        let permissions = vec![
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesList,
            Permission::FilesView,
        ];
        assert!(has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_multiple_roles() {
        let roles = vec![Role::FilesAdmin, Role::FilesViewer];
        let permissions = vec![
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesCreate,
            Permission::FilesEdit,
            Permission::FilesDelete,
            Permission::FilesList,
            Permission::FilesView,
            Permission::FilesManage,
        ];
        assert!(has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_no_create_dir_permission_editor() {
        let roles = vec![Role::FilesEditor];
        let permissions = vec![Permission::DirsCreate];
        assert!(!has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_no_create_dir_permission_viewer() {
        let roles = vec![Role::FilesViewer];
        let permissions = vec![Permission::DirsCreate];
        assert!(!has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_has_create_dir_permission_admin() {
        let roles = vec![Role::FilesAdmin];
        let permissions = vec![Permission::DirsCreate];
        assert!(has_permissions(&roles, &permissions));
    }

    #[test]
    fn test_to_roles_valid() {
        let data = vec!["FilesAdmin".to_string(), "FilesViewer".to_string()];
        let roles = to_roles(data).unwrap();
        assert_eq!(roles, vec![Role::FilesAdmin, Role::FilesViewer]);
    }

    #[test]
    fn test_to_roles_invalid() {
        let data = vec!["FilesAdmin".to_string(), "InvalidRole".to_string()];
        let roles = to_roles(data);
        assert!(roles.is_err());
    }
}
