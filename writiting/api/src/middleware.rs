use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};

pub struct RequiredPermission(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredPermission
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_user = parts
            .extract::<AuthUser>()
            .await
            .map_err(|err| err.into_response())?;
            
        let permission = parts
            .extensions
            .get::<RequiredPermission>()
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Permission requirement not specified",
                )
                    .into_response()
            })?;
            
        if !auth_user.user.has_permission(&permission.0) {
            return Err((
                StatusCode::FORBIDDEN,
                "You don't have permission to access this resource",
            )
                .into_response());
        }
        
        Ok(permission.clone())
    }
}

// Extend the User struct with permission checking
impl User {
    pub fn has_permission(&self, permission_code: &str) -> bool {
        if let Some(role) = &self.role {
            role.permissions
                .iter()
                .any(|p| p.code == permission_code)
        } else {
            false
        }
    }
}