use sqlite::{Connection, State, Statement};

pub struct SqlLiteConfig {
    path: String,
    connection: Connection,
}

impl SqlLiteConfig {
    pub fn new(path: &str) -> Result<SqlLiteConfig, Box<dyn std::error::Error>> {
        let connection = sqlite::open(&path)?;
        Ok(SqlLiteConfig {
            path: path.to_string(),
            connection,
        })
    }

    pub fn add_hook(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut query =
            "CREATE TABLE IF NOT EXISTS hooks (name TEXT, total_runs INTEGER, succesful_runs INTEGER)";
        self.connection.execute(query)?;
        query = "INSERT OR IGNORE INTO hooks VALUES ('?', 0, 0, 0)";
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

    pub fn add_successful_run(&self, name: &String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn add_failed_run(&self, name: &String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
