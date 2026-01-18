/// Parse SQL statements from a migration file, respecting dollar-quoted strings.
/// This handles PostgreSQL's DO $$ ... END $$; blocks correctly.
pub fn parse_sql_statements(content: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current_statement = String::new();
    let mut in_dollar_quote = false;
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        current_statement.push(ch);

        // Check for dollar quote delimiter
        if ch == '$' && chars.peek() == Some(&'$') {
            current_statement.push(chars.next().unwrap()); // consume second $
            in_dollar_quote = !in_dollar_quote;
            continue;
        }

        // Only split on semicolon if we're not inside a dollar-quoted block
        if ch == ';' && !in_dollar_quote {
            let stmt = current_statement.trim().to_string();
            if !stmt.is_empty() && stmt != ";" {
                statements.push(stmt);
            }
            current_statement.clear();
        }
    }

    // Add any remaining statement
    let stmt = current_statement.trim().to_string();
    if !stmt.is_empty() && stmt != ";" {
        statements.push(stmt);
    }

    statements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sql_statements_simple() {
        let sql = "CREATE TABLE users (id INT); INSERT INTO users VALUES (1);";
        let statements = parse_sql_statements(sql);

        assert_eq!(statements.len(), 2);
        assert_eq!(statements[0], "CREATE TABLE users (id INT);");
        assert_eq!(statements[1], "INSERT INTO users VALUES (1);");
    }

    #[test]
    fn test_parse_sql_statements_with_dollar_quotes() {
        let sql = r#"
            INSERT INTO roles VALUES ('Admin');
            DO $$
            DECLARE
                admin_id UUID;
            BEGIN
                SELECT id INTO admin_id FROM roles WHERE name = 'Admin';
                INSERT INTO permissions VALUES (admin_id, 'page', TRUE);
            END $$;
            SELECT * FROM roles;
        "#;

        let statements = parse_sql_statements(sql);

        // Should have 3 statements: INSERT, DO block, SELECT
        assert_eq!(statements.len(), 3);

        // First statement
        assert!(statements[0].contains("INSERT INTO roles"));

        // Second statement should contain the entire DO block
        assert!(statements[1].contains("DO $$"));
        assert!(statements[1].contains("END $$;"));
        assert!(statements[1].contains("DECLARE"));
        assert!(statements[1].contains("BEGIN"));

        // Third statement
        assert!(statements[2].contains("SELECT * FROM roles"));
    }

    #[test]
    fn test_parse_sql_statements_nested_semicolons() {
        let sql = r#"
            CREATE TABLE test (id INT);
            DO $$
            BEGIN
                EXECUTE 'SELECT 1; SELECT 2;';
                INSERT INTO test VALUES (1);
            END $$;
            DROP TABLE test;
        "#;

        let statements = parse_sql_statements(sql);

        // Should have 3 statements
        assert_eq!(statements.len(), 3);

        // The DO block should contain all its semicolons
        let do_block = &statements[1];
        assert!(do_block.contains("DO $$"));
        assert!(do_block.contains("SELECT 1; SELECT 2;"));
        assert!(do_block.contains("INSERT INTO test"));
        assert!(do_block.contains("END $$;"));
    }

    #[test]
    fn test_parse_sql_statements_empty_and_whitespace() {
        let sql = "  ;  ; CREATE TABLE test (id INT);  ;  ";
        let statements = parse_sql_statements(sql);

        // Should only have one non-empty statement
        assert_eq!(statements.len(), 1);
        assert_eq!(statements[0], "CREATE TABLE test (id INT);");
    }

    #[test]
    fn test_parse_sql_statements_real_migration() {
        // Test with actual migration content
        let sql = include_str!("../migrations/001_initial_schema.sql");
        let statements = parse_sql_statements(sql);

        // Should have multiple statements
        assert!(
            statements.len() > 10,
            "Expected more than 10 statements, got {}",
            statements.len()
        );

        // Check that DO blocks are preserved as statements
        let do_blocks: Vec<_> = statements.iter().filter(|s| s.contains("DO $$")).collect();
        assert!(
            do_blocks.len() >= 2,
            "Expected at least 2 DO blocks, found {}",
            do_blocks.len()
        );

        // Check that at least one DO block contains DECLARE (the second one with permissions)
        let has_declare = do_blocks.iter().any(|s| s.contains("DECLARE"));
        assert!(has_declare, "Expected at least one DO block with DECLARE");

        // All DO blocks should have BEGIN and END $$
        for do_block in &do_blocks {
            assert!(do_block.contains("BEGIN"));
            assert!(do_block.contains("END $$;"));
        }

        // The permissions DO block should contain INSERT statements
        let permissions_block = do_blocks
            .iter()
            .find(|s| s.contains("INSERT INTO permissions"));
        assert!(
            permissions_block.is_some(),
            "Expected a DO block with INSERT INTO permissions"
        );
    }
}
