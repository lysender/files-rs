use std::collections::HashSet;

use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Role {
    FilesAdmin,
    FilesEditor,
    FilesViewer,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize)]
pub enum Permission {
    BucketsList,
    BucketsView,

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
            _ => Err(format!(
                "Valid roles are: FilesAdmin, FilesEditor, FilesViewer, got: {}",
                value
            )),
        }
    }
}

impl core::fmt::Display for Role {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Role::FilesAdmin => write!(f, "FilesAdmin"),
            Role::FilesEditor => write!(f, "FilesEditor"),
            Role::FilesViewer => write!(f, "FilesViewer"),
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
            "buckets.list" => Ok(Permission::BucketsList),
            "buckets.view" => Ok(Permission::BucketsView),
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

impl core::fmt::Display for Permission {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Permission::BucketsList => write!(f, "buckets.list"),
            Permission::BucketsView => write!(f, "buckets.view"),
            Permission::DirsCreate => write!(f, "dirs.create"),
            Permission::DirsEdit => write!(f, "dirs.edit"),
            Permission::DirsDelete => write!(f, "dirs.delete"),
            Permission::DirsList => write!(f, "dirs.list"),
            Permission::DirsView => write!(f, "dirs.view"),
            Permission::DirsManage => write!(f, "dirs.manage"),
            Permission::FilesCreate => write!(f, "files.create"),
            Permission::FilesEdit => write!(f, "files.edit"),
            Permission::FilesDelete => write!(f, "files.delete"),
            Permission::FilesList => write!(f, "files.list"),
            Permission::FilesView => write!(f, "files.view"),
            Permission::FilesManage => write!(f, "files.manage"),
        }
    }
}

/// Role to permissions mapping
pub fn role_permissions(role: &Role) -> Vec<Permission> {
    match role {
        Role::FilesAdmin => vec![
            Permission::BucketsList,
            Permission::BucketsView,
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
            Permission::BucketsList,
            Permission::BucketsView,
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesCreate,
            Permission::FilesList,
            Permission::FilesView,
        ],
        Role::FilesViewer => vec![
            Permission::BucketsList,
            Permission::BucketsView,
            Permission::DirsList,
            Permission::DirsView,
            Permission::FilesList,
            Permission::FilesView,
        ],
    }
}

/// Get all permissions for the given roles
pub fn roles_permissions(roles: &Vec<Role>) -> HashSet<Permission> {
    let mut permissions: HashSet<Permission> = HashSet::new();
    roles.iter().for_each(|role| {
        role_permissions(role).iter().for_each(|p| {
            permissions.insert(p.clone());
        });
    });
    permissions
}

/// Checks whether the given roles have the required permissions
pub fn has_permissions(roles: &Vec<Role>, req_permissions: &Vec<Permission>) -> bool {
    let permissions = roles_permissions(roles);
    req_permissions
        .iter()
        .all(|permission| permissions.contains(permission))
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
