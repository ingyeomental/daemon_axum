# Axum Web API 서버

Rust의 Axum 프레임워크를 사용한 웹 API 서버 프로젝트입니다.

## 프로젝트 구조

```
src/
├── main.rs              # 애플리케이션 진입점
├── models/              # 데이터 모델
│   ├── mod.rs          # 모델 모듈 정의
│   └── payload.rs      # Payload 구조체 정의
├── handlers/           # API 핸들러
│   ├── mod.rs         # 핸들러 모듈 정의
│   └── api.rs         # API 엔드포인트 핸들러 구현
├── routes/            # 라우팅 설정
│   ├── mod.rs        # 라우트 모듈 정의
│   └── api.rs        # API 라우트 정의
├── services/          # 비즈니스 로직
│   ├── mod.rs        # 서비스 모듈 정의
│   └── item_service.rs # 아이템 관련 비즈니스 로직
├── repositories/      # 데이터 접근 계층
│   ├── mod.rs        # 리포지토리 모듈 정의
│   └── item_repository.rs # 아이템 데이터 접근 로직
└── middleware/        # 미들웨어
    └── mod.rs        # 미들웨어 모듈 (향후 확장 예정)
```

## 모듈 설명

### models
- `payload.rs`: API 요청/응답에 사용되는 데이터 구조체 정의
  - `Payload`: JSON 요청 본문을 처리하기 위한 구조체

### handlers
- `api.rs`: API 엔드포인트의 핸들러 함수 구현
  - `health()`: 서버 상태 확인 엔드포인트
  - `hello()`: 이름을 받아 인사말을 반환하는 엔드포인트
  - `echo()`: JSON 요청 본문을 그대로 반환하는 엔드포인트
  - CRUD 핸들러들: 아이템 생성, 조회, 수정, 삭제

### routes
- `api.rs`: API 라우트 설정
  - `/health`: GET 요청 처리
  - `/hello/:name`: GET 요청 처리
  - `/echo`: POST 요청 처리
  - `/items`: POST 요청 처리 (아이템 생성)
  - `/items/:id`: GET, PUT, PATCH, DELETE 요청 처리

### services
- `item_service.rs`: 아이템 관련 비즈니스 로직 구현
  - 유효성 검사
  - 비즈니스 규칙 적용
  - 리포지토리 계층과의 상호작용

### repositories
- `item_repository.rs`: 데이터 접근 로직 구현
  - 데이터베이스 작업 추상화
  - CRUD 작업 구현
  - 향후 실제 데이터베이스 연동 예정

### middleware
- 현재는 기본적인 로깅 미들웨어만 포함
- 향후 인증, CORS, 요청 제한 등의 미들웨어 추가 예정

## API 엔드포인트

### GET /health
서버 상태 확인
- 응답: `200 OK` 상태 코드와 "OK" 메시지

### GET /hello/:name
이름을 받아 인사말 반환
- 경로 파라미터: `name`
- 응답: "Hello, {name}!"

### POST /echo
JSON 요청 본문을 그대로 반환
- 요청 본문: `{ "message": "string" }`
- 응답: `201 Created` 상태 코드와 요청의 message 값

### 아이템 관리 API

#### POST /items
새 아이템 생성
- 요청 본문: `{ "message": "string" }`
- 응답: `201 Created` 상태 코드와 생성된 아이템 ID

#### GET /items/:id
아이템 조회
- 경로 파라미터: `id`
- 응답: `200 OK` 상태 코드와 아이템 정보

#### PUT /items/:id
아이템 전체 업데이트
- 경로 파라미터: `id`
- 요청 본문: `{ "message": "string" }`
- 응답: `200 OK` 상태 코드와 업데이트 결과

#### PATCH /items/:id
아이템 부분 업데이트
- 경로 파라미터: `id`
- 요청 본문: `{ "message": "string" }`
- 응답: `200 OK` 상태 코드와 업데이트 결과

#### DELETE /items/:id
아이템 삭제
- 경로 파라미터: `id`
- 응답: `200 OK` 상태 코드와 삭제 결과

## 실행 방법

```bash
cargo run
```

서버는 기본적으로 `0.0.0.0:8080`에서 실행됩니다.

## 향후 개선 사항

1. 데이터베이스 연동
   - SQLx 또는 Diesel ORM 사용
   - 마이그레이션 관리

2. 에러 처리 개선
   - 커스텀 에러 타입 정의
   - 에러 변환 및 처리

3. 인증/인가
   - JWT 기반 인증
   - 역할 기반 접근 제어

4. 테스트
   - 단위 테스트
   - 통합 테스트
   - E2E 테스트

5. 문서화
   - OpenAPI/Swagger 문서
   - API 사용 예제