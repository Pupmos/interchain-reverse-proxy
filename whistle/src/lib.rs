// Project Struct
pub struct Project {
    // project name
    pub project_name: String,
    // project id
    pub project_id: String,
    // id of account that owns this project
    pub account_id: String,
    // we don't actually store secrets. just the bcrypt hash of them.
    pub secret_hash: String,
    // maximum requests per second
    pub max_rps: Option<i32>,
    // maximum requests per day
    pub max_rpd: Option<i32>,
    // allowed request origin urls
    pub allowed_origins: Vec<String>,
}

// load project by id
pub async fn load_project_by_id(project_id: &str) -> Option<Project> {
    let project = Project {
        project_name: "test".to_string(),
        project_id: "test".to_string(),
        account_id: "test".to_string(),
        secret_hash: "$2b$12$QXZ1c2Vyc2hpcHNlY3JldA==".to_string(),
        max_rps: Some(100),
        max_rpd: Some(1000),
        allowed_origins: vec!["http://localhost:3000".to_string()],
    };
    return Some(project);
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse_basic_credentials(authorization: &str) -> Option<(String, String)> {
    let mut parts = authorization.splitn(2, ' ');
    let scheme = parts.next()?;
    let credentials = parts.next()?;

    if scheme != "Basic" {
        return None;
    }

    let decoded = base64::decode(credentials).ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let mut parts = decoded.splitn(2, ':');
    let project_id = parts.next()?.to_string();
    let project_secret = parts.next()?.to_string();

    Some((project_id, project_secret))
}

// verify that the project id and secret are valid
pub async fn verify_credentials(project_id: &str, project_secret: &str) -> Option<bool> {
    let project = load_project_by_id(project_id).await?;
    // load project by id and verify secret
    let is_valid = bcrypt::verify(project_secret, &project.secret_hash).unwrap();
    return Some(is_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
