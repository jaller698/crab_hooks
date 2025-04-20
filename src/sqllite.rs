use sqlite::{Connection, State};

use crate::hook_types::HookTypes;

pub struct SqlLiteConfig {
    // path: String,
    connection: Connection,
}

impl SqlLiteConfig {
    pub fn new(path: &str) -> Result<SqlLiteConfig, Box<dyn std::error::Error>> {
        let connection = sqlite::open(path)?;
        let config = SqlLiteConfig {
            //      path: path.to_string(),
            connection,
        };
        config.init()?;
        Ok(config)
    }
    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut query =
            "CREATE TABLE IF NOT EXISTS hooks (name TEXT UNIQUE, total_runs INTEGER, succesful_runs INTEGER)";
        self.connection.execute(query)?;
        query = "CREATE TABLE IF NOT EXISTS repo_hooks (name TEXT, repo TEXT, type TEXT, FOREIGN KEY(name) REFERENCES hooks(name))";
        self.connection.execute(query)?;

        Ok(())
    }

    pub fn add_hook(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query =
            "INSERT OR IGNORE INTO hooks (name, total_runs, succesful_runs) VALUES (?, 0, 0)";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, name))?;
        statement.next()?;
        Ok(())
    }

    pub fn is_hook_managed(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let query = "SELECT * FROM hooks WHERE name = ?";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, name))?;
        if let Ok(state) = statement.next() {
            if state == State::Row {
                return Ok(true);
            }
        };
        Ok(false)
    }

    pub fn add_hook_to_repo(
        &self,
        name: &str,
        repo: &str,
        hook_type: &HookTypes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let query = "INSERT INTO repo_hooks VALUES (?, ?, ?)";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, name))?;
        statement.bind((2, repo))?;
        statement.bind((3, hook_type.to_string().as_str()))?;
        statement.next()?;
        Ok(())
    }

    pub fn check_if_new_hook_is_known(
        &self,
        repo: &str,
        hook_type: &HookTypes,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let query = "SELECT * FROM repo_hooks WHERE repo = ? AND type = ?";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, repo))?;
        statement.bind((2, hook_type.to_string().as_str()))?;
        if let Ok(state) = statement.next() {
            if state == State::Row {
                return Ok(true);
            }
        };
        Ok(false)
    }

    pub fn check_if_new_hook_is_same(
        &self,
        repo: &str,
        hook_type: &HookTypes,
        name: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let query = "SELECT * FROM repo_hooks WHERE repo = ? AND type = ? AND name = ?";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, repo))?;
        statement.bind((2, hook_type.to_string().as_str()))?;
        statement.bind((3, name))?;
        if let Ok(state) = statement.next() {
            if state == State::Row {
                return Ok(true);
            }
        };
        Ok(false)
    }

    pub fn add_successful_run(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = "UPDATE hooks SET total_runs = total_runs + 1, succesful_runs = succesful_runs + 1 WHERE name = ?";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, name))?;
        statement.next()?;
        Ok(())
    }

    pub fn add_failed_run(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let query = "UPDATE hooks SET total_runs = total_runs + 1 WHERE name = ?";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((1, name))?;
        statement.next()?;
        Ok(())
    }
}
