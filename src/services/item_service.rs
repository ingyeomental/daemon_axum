use crate::models::payload::Payload;
use crate::repositories::item_repository::ItemRepository;

#[derive(Debug)]
pub struct ItemService {
    repository: ItemRepository,
}

impl ItemService {
    pub fn new() -> Self {
        Self {
            repository: ItemRepository::new(),
        }
    }

    pub async fn create_item(&self, payload: Payload) -> Result<String, String> {
        // 비즈니스 로직 예시
        if payload.message.is_empty() {
            return Err("메시지는 비어있을 수 없습니다".to_string());
        }

        // 리포지토리를 통한 데이터 저장
        let id = self.repository.create(&payload).await?;
        Ok(format!("Created item with id: {}", id))
    }

    pub async fn get_item(&self, id: i32) -> Result<String, String> {
        // 비즈니스 로직 예시
        if id <= 0 {
            return Err("유효하지 않은 ID입니다".to_string());
        }

        // 리포지토리를 통한 데이터 조회
        let item = self.repository.find_by_id(id).await?;
        Ok(format!("Retrieved: {}", item.message))
    }

    pub async fn update_item(&self, id: i32, payload: Payload) -> Result<String, String> {
        // 비즈니스 로직 예시
        if id <= 0 {
            return Err("유효하지 않은 ID입니다".to_string());
        }

        if payload.message.is_empty() {
            return Err("메시지는 비어있을 수 없습니다".to_string());
        }

        // 리포지토리를 통한 데이터 업데이트
        self.repository.update(id, &payload).await?;
        Ok(format!("Updated item {}", id))
    }

    pub async fn delete_item(&self, id: i32) -> Result<String, String> {
        // 비즈니스 로직 예시
        if id <= 0 {
            return Err("유효하지 않은 ID입니다".to_string());
        }

        // 리포지토리를 통한 데이터 삭제
        self.repository.delete(id).await?;
        Ok(format!("Deleted item {}", id))
    }
}
