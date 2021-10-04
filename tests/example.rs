use db_tx::*;

#[run_in_tx(get_connect())]
fn usecase(conn: &Connection) -> i32 {
    conn.execute()
}

fn get_connect() -> Connection {
    Connection {}
}

struct Connection {}

impl Connection {
    fn transaction<F, T>(&self, func_in_tx: F) -> T
    where
        F: Fn() -> T,
    {
        println!("transaction begin");
        let result = func_in_tx();
        println!("transaction end");
        result
    }

    fn execute(&self) -> i32 {
        println!("execute sql");
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        usecase();
        // this will output
        // ーーーーーーーーーーーーー
        // ｜transaction begin
        // ｜execute sql
        // ｜transaction end
        // ーーーーーーーーーーーーー
    }
}
