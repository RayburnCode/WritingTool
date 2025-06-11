use dioxus::prelude::*;
use crate::Role;
use crate::ServerFnError;
use crate::db::connection_pool::get_db; 
use tracing::info;


#[server]
    pub async fn find_by_id( id: i32) -> Result<Option<RoleWithPermissions>, sqlx::Error> {
            let db = get_db().await;
        let role = sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?;

        if let Some(role) = role {
            let permissions = self.get_role_permissions(role.id).await?;
            Ok(Some(RoleWithPermissions { role, permissions }))
        } else {
            Ok(None)
        }
    }

    #[server]
    pub async fn get_default_role() -> Result<Role, sqlx::Error> {
            let db = get_db().await;
        sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles
            WHERE is_default = true
            LIMIT 1
            "#
        )
        .fetch_one(db)
        .await
    }

    #[server]
    pub async fn create_role(
    
        name: &str,
        description: Option<&str>,
        permission_ids: &[i32],
    ) -> Result<RoleWithPermissions, sqlx::Error> {
    let db = get_db().await;

        let role = sqlx::query_as!(
            Role,
            r#"
            INSERT INTO roles (name, description)
            VALUES ($1, $2)
            RETURNING *
            "#,
            name,
            description
        )
        .fetch_one(&mut *db)
        .await?;

        self.assign_permissions_to_role(&mut *db, role.id, permission_ids)
            .await?;

        let permissions = self.get_role_permissions(role.id).await?;

        tx.commit().await?;

        Ok(RoleWithPermissions { role, permissions })
    }


    #[server]
    pub async fn update_role(
        id: i32,
        name: Option<&str>,
        description: Option<&str>,
        permission_ids: Option<&[i32]>,
    ) -> Result<RoleWithPermissions, sqlx::Error> {
    let mut db = get_db().await;

        let role = if let Some(name) = name {
            sqlx::query_as!(
                Role,
                r#"
                UPDATE roles
                SET name = $1, description = $2, updated_at = NOW()
                WHERE id = $3
                RETURNING *
                "#,
                name,
                description,
                id
            )
            .fetch_one(&mut *db)
            .await?
        } else {
            sqlx::query_as!(
                Role,
                r#"
                UPDATE roles
                SET description = $1, updated_at = NOW()
                WHERE id = $2
                RETURNING *
                "#,
                description,
                id
            )
            .fetch_one(&mut *db)
            .await?
        };

        if let Some(permission_ids) = permission_ids {
            sqlx::query!(
                r#"
                DELETE FROM role_permissions
                WHERE role_id = $1
                "#,
                role.id
            )
            .execute(&mut *db)
            .await?;

            self.assign_permissions_to_role(&mut *db, role.id, permission_ids)
                .await?;
        }

        let permissions = self.get_role_permissions(role.id).await?;

        tx.commit().await?;

        Ok(RoleWithPermissions { role, permissions })
    }

    #[server]
    pub async fn delete_role(id: i32) -> Result<(), sqlx::Error> {
            let db = get_db().await;

        sqlx::query!(
            r#"
            DELETE FROM roles
            WHERE id = $1 AND is_default = false
            "#,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }

    #[server]
    pub async fn list_roles() -> Result<Vec<RoleWithPermissions>, sqlx::Error> {
            let db = get_db().await;

        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles
            ORDER BY name
            "#
        )
        .fetch_all(db)
        .await?;

        let mut result = Vec::new();
        for role in roles {
            let permissions = self.get_role_permissions(role.id).await?;
            result.push(RoleWithPermissions { role, permissions });
        }

        Ok(result)
    }

    #[server]
    pub async fn list_permissions() -> Result<Vec<Permission>, sqlx::Error> {
            let db = get_db().await;

        sqlx::query_as!(
            Permission,
            r#"
            SELECT * FROM permissions
            ORDER BY code
            "#
        )
        .fetch_all(db)
        .await
    }

    #[server]
    async fn get_role_permissions(role_id: i32) -> Result<Vec<Permission>, sqlx::Error> {
            let db = get_db().await;

        sqlx::query_as!(
            Permission,
            r#"
            SELECT p.* FROM permissions p
            JOIN role_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = $1
            ORDER BY p.code
            "#,
            role_id
        )
        .fetch_all(db)
        .await
    }

    #[server]
    async fn assign_permissions_to_role(
        role_id: i32,
        permission_ids: &[i32],
    ) -> Result<(), sqlx::Error> {
        for permission_id in permission_ids {
                let db = get_db().await;

            sqlx::query!(
                r#"
                INSERT INTO role_permissions (role_id, permission_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                role_id,
                permission_id
            )
            .execute(db)
            .await?;
        }

        Ok(())
    }
