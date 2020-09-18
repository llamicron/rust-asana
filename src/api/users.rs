/// Returns the current authenticated user. Equivalent to "/users/{gid}" where
/// `{gid}` is the authenticated user's gid
pub fn me() -> String {
    format!("/users/me")
}

/// Returns the user records for all users in all workspaces and
/// organizations accessible to the authenticated user. Accepts
/// an optional workspace ID parameter. Results are sorted by user ID.
pub fn users() -> String {
    format!("/users")
}

/// Returns the full user record for the single user with
/// the provided ID. Results are sorted by user ID.
pub fn user(user_gid: &str) -> String {
    format!("/users/{}", user_gid)
}


/// Returns all of a user's favorites in the given workspace,
/// of the given type. Results are given in order
/// (The same order as Asana's sidebar).
pub fn favorites(user_gid: &str, resource_type: &str, workspace_gid: &str) -> String {
    format!(
        "/users/{}/favorites?resource_type={}&workspace={}",
        user_gid,
        resource_type,
        workspace_gid
    )
}

/// Returns the compact records for all users
/// that are members of the team.
pub fn team(team_gid: &str) -> String {
    format!("/teams/{}/users", team_gid)
}

/// Returns the user records for all users in
/// the specified workspace or organization.
/// Results are sorted alphabetically by user names.
pub fn workspace(workspace_gid: &str) -> String {
    format!("/workspaces/{}/users", workspace_gid)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::API;
    use crate::schema;

    fn api() -> API {
        API::from_token(crate::get_pat())
    }

    #[test]
    fn test_get_me() {
        let mut asana = api();
        let resp = asana.request( me() ).get().expect("Couldn't perform request");
        assert!(resp.errors().is_none());
        let me = resp.value::<schema::UserCompact>();
        assert!(me.is_some());
        assert_eq!(me.unwrap().resource_type, "user");
    }

    #[test]
    fn test_get_user_by_gid() {
        let mut asana = api();
        let resp = asana.request( user("811156077822027") ).get().unwrap();
        assert!(resp.errors().is_none());
        let user = resp.value::<schema::UserCompact>();
        assert!(user.is_some());
        assert!(user.unwrap().name.contains("Luke"));
    }

    #[test]
    fn test_get_user_invalid_gid() {
        let mut asana = api();
        let resp = asana.request( user("something not valid") ).get().unwrap();
        assert!(resp.errors().is_some());
        let errors = resp.errors().unwrap();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_get_users() {
        let mut asana = api();
        let resp = asana.request( users() ).get().expect("Couldn't perform request");
        let users = resp.values::<schema::UserCompact>();
        assert!(resp.errors().is_none());
        assert!(users.unwrap().len() > 1);
    }
}
