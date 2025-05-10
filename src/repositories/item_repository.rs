use crate::models::payload::Payload;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::sync::{Arc, Mutex};
use tokio::task::spawn_blocking;

#[derive(Debug, Clone)]
pub struct ItemRepository {
    // 여기에 데이터베이스 연결이나 다른 의존성을 주입
    conn: Arc<Mutex<Connection>>,
}

impl ItemRepository {
    pub fn new() -> Self {
        // 데이터베이스 연결
        let conn = Connection::open("data/items.db").expect("데이터베이스 연결 실패");

        // 테이블 생성
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY, message TEXT)",
            [],
        )
        .expect("테이블 생성 실패");

        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub async fn create(&self, payload: &Payload) -> Result<i32, String> {
        // 실제로는 여기서 데이터베이스에 저장
        // 예: INSERT INTO items (message) VALUES ($1) RETURNING id
        let conn = Arc::clone(&self.conn);
        let message = payload.message.clone();
        spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute("INSERT INTO items (message) VALUES (?1)", params![message])
                .map_err(|e| e.to_string())?;

            Ok(conn.last_insert_rowid() as i32)
        })
        .await
        .map_err(|e| e.to_string())?
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Payload, String> {
        // 실제로는 여기서 데이터베이스에서 조회
        // 예: SELECT * FROM items WHERE id = $1
        let conn = Arc::clone(&self.conn);
        spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.query_row(
                "SELECT message FROM items WHERE id = ?1",
                params![id],
                |row| {
                    let message: String = row.get(0)?;
                    Ok(Payload { message })
                },
            )
            .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())?
    }

    pub async fn update(&self, id: i32, payload: &Payload) -> Result<(), String> {
        // 실제로는 여기서 데이터베이스 업데이트
        // 예: UPDATE items SET message = $1 WHERE id = $2
        let conn = Arc::clone(&self.conn);
        let message = payload.message.clone();
        spawn_blocking(move || {
            let conn = conn.lock().unwrap();
            conn.execute(
                "UPDATE items SET message = ?1 WHERE id = ?2",
                params![message, id],
            )
            .map_err(|e| e.to_string())?;
            Ok(())
        })
        .await
        .map_err(|e| e.to_string())?
    }

    pub async fn delete(&self, id: i32) -> Result<(), String> {
        // 실제로는 여기서 데이터베이스에서 삭제
        // 예: DELETE FROM items WHERE id = $1
        Ok(())
    }
}
