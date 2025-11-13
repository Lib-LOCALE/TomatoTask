// Requêtes SQL pour la gestion des projets
use crate::db::models::{CreateProjectInput, Project};
use rusqlite::{Connection, Result, params};

/// Récupère tous les projets
///
/// # Arguments
/// * `conn` - Connexion à la base de données
pub fn get_all_projects(conn: &Connection) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, color, created_at, updated_at
         FROM projects
         ORDER BY created_at DESC",
    )?;

    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    projects.collect()
}

/// Récupère un projet par son ID
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `project_id` - ID du projet
pub fn get_project_by_id(conn: &Connection, project_id: i64) -> Result<Project> {
    conn.query_row(
        "SELECT id, name, color, created_at, updated_at
         FROM projects
         WHERE id = ?1",
        [project_id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    )
}

/// Crée un nouveau projet
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `input` - Données du projet à créer
pub fn create_project(conn: &Connection, input: &CreateProjectInput) -> Result<Project> {
    conn.execute(
        "INSERT INTO projects (name, color) VALUES (?1, ?2)",
        params![&input.name, &input.color],
    )?;

    let project_id = conn.last_insert_rowid();
    get_project_by_id(conn, project_id)
}

/// Supprime un projet
///
/// Note: Les tâches associées auront leur project_id mis à NULL (ON DELETE SET NULL)
///
/// # Arguments
/// * `conn` - Connexion à la base de données
/// * `project_id` - ID du projet à supprimer
pub fn delete_project(conn: &Connection, project_id: i64) -> Result<()> {
    conn.execute("DELETE FROM projects WHERE id = ?1", [project_id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations::run_migrations;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_create_and_get_project() {
        let conn = setup_test_db();
        let input = CreateProjectInput {
            name: "Work".to_string(),
            color: Some("#FF0000".to_string()),
        };

        let project = create_project(&conn, &input).unwrap();
        assert_eq!(project.name, "Work");
        assert_eq!(project.color, Some("#FF0000".to_string()));

        let retrieved = get_project_by_id(&conn, project.id).unwrap();
        assert_eq!(retrieved.name, project.name);
    }

    #[test]
    fn test_get_all_projects() {
        let conn = setup_test_db();

        let input1 = CreateProjectInput {
            name: "Project 1".to_string(),
            color: None,
        };
        let input2 = CreateProjectInput {
            name: "Project 2".to_string(),
            color: Some("#00FF00".to_string()),
        };

        create_project(&conn, &input1).unwrap();
        create_project(&conn, &input2).unwrap();

        let projects = get_all_projects(&conn).unwrap();
        assert_eq!(projects.len(), 2);
    }
}
